use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tera::{Context, Tera, Value};
use include_dir::{Dir, File};

use crate::utils::uid_generator;

#[derive(Debug)]
struct TemplateStructure<'a> {
    tera_files: Vec<&'a File<'a>>,
    static_files: Vec<&'a File<'a>>,
    relative_paths: HashMap<String, PathBuf>,
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
    template_dir: &Dir,
    dest_path: &Path,
    context: Context,
) -> Result<(), io::Error> {
    let template_structure = discover_template_structure(template_dir)?;

    validate_destination(dest_path, &template_structure, &context)?;

    let tera = init_tera_engine(template_dir)?;

    for tera_file in &template_structure.tera_files {
        let file_path = tera_file.path();
        let path_str = file_path.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid UTF-8 in path: {:?}", file_path),
            )
        })?;

        let relative_path = template_structure
            .relative_paths
            .get(path_str)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Missing relative path for: {}", path_str),
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
        let file_path = static_file.path();
        let path_str = file_path.to_str().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid UTF-8 in path: {:?}", file_path),
            )
        })?;

        let relative_path = template_structure
            .relative_paths
            .get(path_str)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Missing relative path for: {}", path_str),
                )
            })?;

        let substituted_path = substitute_path_variables(relative_path, &context)?;
        let dest_file = dest_path.join(substituted_path);

        if let Some(parent) = dest_file.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write embedded file contents to destination
        fs::write(&dest_file, static_file.contents())?;
    }

    Ok(())
}

fn discover_template_structure<'a>(template_dir: &'a Dir<'a>) -> Result<TemplateStructure<'a>, io::Error> {
    let mut tera_files = Vec::new();
    let mut static_files = Vec::new();
    let mut relative_paths = HashMap::new();

    // Get the base path that we need to strip from all file paths
    // This is the template directory name (e.g., "blank" or "blank_ecs")
    let base_prefix = template_dir.path();

    // Recursively traverse embedded directory
    fn traverse_dir<'a>(
        dir: &'a Dir,
        tera_files: &mut Vec<&'a File<'a>>,
        static_files: &mut Vec<&'a File<'a>>,
        relative_paths: &mut HashMap<String, PathBuf>,
        base_prefix: &Path,
    ) {
        for file in dir.files() {
            let file_path = file.path();
            let path_str = file_path.to_str().unwrap_or("");

            if file_path.extension().and_then(|s| s.to_str()) == Some("tera") {
                tera_files.push(file);
            } else {
                static_files.push(file);
            }

            // Strip the template directory prefix to get relative path
            let relative_path = file_path.strip_prefix(base_prefix)
                .unwrap_or(file_path);

            relative_paths.insert(path_str.to_string(), relative_path.to_path_buf());
        }

        for subdir in dir.dirs() {
            traverse_dir(subdir, tera_files, static_files, relative_paths, base_prefix);
        }
    }

    traverse_dir(template_dir, &mut tera_files, &mut static_files, &mut relative_paths, base_prefix);

    Ok(TemplateStructure {
        tera_files,
        static_files,
        relative_paths,
    })
}

fn init_tera_engine(template_dir: &Dir) -> Result<Tera, io::Error> {
    let mut tera = Tera::default();

    // Get the base path to strip from template names
    let base_prefix = template_dir.path();

    // Recursively add all .tera files to the engine
    fn add_tera_files(dir: &Dir, tera: &mut Tera, base_prefix: &Path) -> Result<(), io::Error> {
        for file in dir.files() {
            let file_path = file.path();
            if file_path.extension().and_then(|s| s.to_str()) == Some("tera") {
                // Strip the template directory prefix to get relative path
                let relative_path = file_path.strip_prefix(base_prefix)
                    .unwrap_or(file_path);

                let template_name = relative_path.to_str().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Invalid UTF-8 in template path: {:?}", relative_path),
                    )
                })?.replace('\\', "/");

                let content = std::str::from_utf8(file.contents()).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Invalid UTF-8 in template file '{}': {}", template_name, e),
                    )
                })?;

                tera.add_raw_template(&template_name, content).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to add template '{}': {}", template_name, e),
                    )
                })?;
            }
        }

        for subdir in dir.dirs() {
            add_tera_files(subdir, tera, base_prefix)?;
        }

        Ok(())
    }

    add_tera_files(template_dir, &mut tera, base_prefix)?;

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
        let file_path = tera_file.path();
        let path_str = file_path.to_str().unwrap_or("");

        if let Some(relative_path) = template_structure.relative_paths.get(path_str) {
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
        let file_path = static_file.path();
        let path_str = file_path.to_str().unwrap_or("");

        if let Some(relative_path) = template_structure.relative_paths.get(path_str) {
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
