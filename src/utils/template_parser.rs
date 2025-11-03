use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tera::{Context, Tera, Value};
use walkdir::WalkDir;

use crate::utils::uid_generator;

#[derive(Debug)]
struct TemplateStructure {
    tera_files: Vec<PathBuf>,
    static_files: Vec<PathBuf>,
    relative_paths: HashMap<PathBuf, PathBuf>,
}

fn substitute_path_variables(path: &Path, context: &Context) -> Result<PathBuf, io::Error> {
    let mut result = PathBuf::new();

    for component in path.components() {
        if let Some(component_str) = component.as_os_str().to_str() {
            let mut substituted = component_str.to_string();

            while let Some(start) = substituted.find("{{") {
                if let Some(end) = substituted[start..].find("}}") {
                    let end = start + end;
                    let var_name = &substituted[start + 2..end].trim();

                    let value =
                        context
                            .get(var_name)
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| {
                                io::Error::new(
                                    io::ErrorKind::Other,
                                    format!(
                                        "Variable '{}' not found in context for path: {}",
                                        var_name,
                                        path.display()
                                    ),
                                )
                            })?;

                    substituted = format!(
                        "{}{}{}",
                        &substituted[..start],
                        value,
                        &substituted[end + 2..]
                    );
                } else {
                    break;
                }
            }

            result.push(substituted);
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid path component (non-UTF8): {}", path.display()),
            ));
        }
    }

    Ok(result)
}

pub fn parse_template(
    template_path: &Path,
    dest_path: &Path,
    context: Context,
) -> Result<(), io::Error> {
    if !template_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Template directory not found: {}", template_path.display()),
        ));
    }

    let template_structure = discover_template_structure(template_path)?;

    validate_destination(dest_path, &template_structure, &context)?;

    let tera = init_tera_engine(template_path)?;

    for tera_file in &template_structure.tera_files {
        let relative_path = template_structure
            .relative_paths
            .get(tera_file)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Missing relative path for: {}", tera_file.display()),
                )
            })?;

        let substituted_path = substitute_path_variables(relative_path, &context)?;

        let dest_file = dest_path.join(
            substituted_path
                .to_str()
                .and_then(|s| s.strip_suffix(".tera"))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Invalid .tera filename: {}", substituted_path.display()),
                    )
                })?,
        );

        render_and_write_file(&tera, &dest_file, &context, relative_path)?;
    }

    for static_file in &template_structure.static_files {
        let relative_path = template_structure
            .relative_paths
            .get(static_file)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Missing relative path for: {}", static_file.display()),
                )
            })?;

        let substituted_path = substitute_path_variables(relative_path, &context)?;
        let dest_file = dest_path.join(substituted_path);

        if let Some(parent) = dest_file.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(static_file, &dest_file)?;
    }

    Ok(())
}

fn discover_template_structure(template_path: &Path) -> Result<TemplateStructure, io::Error> {
    let mut tera_files = Vec::new();
    let mut static_files = Vec::new();
    let mut relative_paths = HashMap::new();

    for entry in WalkDir::new(template_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let relative_path = path.strip_prefix(template_path).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to calculate relative path: {}", e),
            )
        })?;

        if path.extension().and_then(|s| s.to_str()) == Some("tera") {
            tera_files.push(path.to_path_buf());
        } else {
            static_files.push(path.to_path_buf());
        }

        relative_paths.insert(path.to_path_buf(), relative_path.to_path_buf());
    }

    Ok(TemplateStructure {
        tera_files,
        static_files,
        relative_paths,
    })
}

fn init_tera_engine(template_path: &Path) -> Result<Tera, io::Error> {
    let pattern = template_path
        .join("**/*.tera")
        .to_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Invalid template path (non-UTF8 characters)",
            )
        })?
        .to_string();

    let mut tera = Tera::new(&pattern).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize Tera engine: {}", e),
        )
    })?;

    tera.register_function("generate_uid", generate_uid_function);

    Ok(tera)
}

fn generate_uid_function(_args: &HashMap<String, Value>) -> tera::Result<Value> {
    let uid = uid_generator::generate_godot_uid();
    let uid_without_prefix = uid.trim_start_matches("uid://");
    Ok(Value::String(uid_without_prefix.to_string()))
}

fn validate_destination(
    dest_path: &Path,
    template_structure: &TemplateStructure,
    context: &Context,
) -> Result<(), io::Error> {
    if !dest_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "Destination directory does not exist: {}",
                dest_path.display()
            ),
        ));
    }

    let mut conflicts = Vec::new();

    for tera_file in &template_structure.tera_files {
        if let Some(relative_path) = template_structure.relative_paths.get(tera_file) {
            let substituted_path = substitute_path_variables(relative_path, context)?;

            if let Some(relative_str) = substituted_path.to_str() {
                if let Some(stripped) = relative_str.strip_suffix(".tera") {
                    let dest_file = dest_path.join(stripped);
                    if dest_file.exists() {
                        conflicts.push(dest_file);
                    }
                }
            }
        }
    }

    for static_file in &template_structure.static_files {
        if let Some(relative_path) = template_structure.relative_paths.get(static_file) {
            let substituted_path = substitute_path_variables(relative_path, context)?;

            let dest_file = dest_path.join(substituted_path);
            if dest_file.exists() {
                conflicts.push(dest_file);
            }
        }
    }

    if !conflicts.is_empty() {
        let conflict_list: Vec<String> = conflicts
            .iter()
            .map(|p| format!("  - {}", p.display()))
            .collect();

        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "Destination has conflicting files:\n{}",
                conflict_list.join("\n")
            ),
        ));
    }

    Ok(())
}

fn render_and_write_file(
    tera: &Tera,
    // template_file: &Path,
    dest_file: &Path,
    context: &Context,
    relative_path: &Path,
) -> Result<(), io::Error> {
    if let Some(parent) = dest_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let template_name = relative_path
        .to_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Invalid template path (non-UTF8): {}",
                    relative_path.display()
                ),
            )
        })?
        .replace('\\', "/");

    let rendered = tera.render(&template_name, context).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to render template '{}': {}", template_name, e),
        )
    })?;

    fs::write(dest_file, rendered)?;

    Ok(())
}
