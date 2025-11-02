use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tera::{Context, Tera, Value};
use walkdir::WalkDir;

use crate::utils::uid_generator;

/// Represents the structure of discovered template files
#[derive(Debug)]
struct TemplateStructure {
    /// Files ending in .tera that need to be rendered
    tera_files: Vec<PathBuf>,
    /// Non-.tera files that should be copied as-is
    static_files: Vec<PathBuf>,
    /// Maps absolute paths to their relative paths within the template
    relative_paths: HashMap<PathBuf, PathBuf>,
}

/// Main entry point for template parsing
///
/// # Arguments
/// * `template_path` - Absolute path to the template directory (e.g., "/path/to/templates/blank")
/// * `dest_path` - Absolute path where files should be written
/// * `context` - Tera Context containing all template variables
///
/// # Behavior
/// - Discovers all files in template_path recursively
/// - Validates destination is empty (returns error if files exist)
/// - Renders .tera files with provided context
/// - Copies non-.tera files as-is
/// - Preserves exact folder structure from template to destination
pub fn parse_template(
    template_path: &Path,
    dest_path: &Path,
    context: Context,
) -> Result<(), io::Error> {
    // Verify template directory exists
    if !template_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Template directory not found: {}", template_path.display()),
        ));
    }

    // Discover all files in template directory
    let template_structure = discover_template_structure(template_path)?;

    // Validate destination directory
    validate_destination(dest_path, &template_structure)?;

    // Initialize Tera engine with custom functions
    let tera = init_tera_engine(template_path)?;

    // Process all .tera files
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

        // Determine destination path by stripping .tera extension
        let dest_file = dest_path.join(
            relative_path
                .to_str()
                .and_then(|s| s.strip_suffix(".tera"))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Invalid .tera filename: {}", relative_path.display()),
                    )
                })?,
        );

        render_and_write_file(&tera, tera_file, &dest_file, &context, relative_path)?;
    }

    // Process all static files (copy as-is)
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

        let dest_file = dest_path.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = dest_file.parent() {
            fs::create_dir_all(parent)?;
        }

        // Copy file as-is
        fs::copy(static_file, &dest_file)?;
    }

    Ok(())
}

/// Discovers all files in template directory recursively
///
/// Walks the template directory tree and categorizes files into:
/// - .tera files (need rendering)
/// - Static files (copy as-is)
/// - Relative path mapping for structure preservation
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

        // Skip directories, only process files
        if !path.is_file() {
            continue;
        }

        // Calculate relative path from template root
        let relative_path = path.strip_prefix(template_path).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to calculate relative path: {}", e),
            )
        })?;

        // Categorize by file extension
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

/// Initializes Tera engine with custom functions
///
/// - Loads all .tera files from template directory
/// - Registers generate_uid custom function for Godot UID generation
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

    // Register custom function for UID generation
    tera.register_function("generate_uid", generate_uid_function);

    Ok(tera)
}

/// Custom Tera function to generate Godot UIDs
///
/// Called from templates as: {{ generate_uid() }}
/// Returns just the UID portion without "uid://" prefix
fn generate_uid_function(_args: &HashMap<String, Value>) -> tera::Result<Value> {
    let uid = uid_generator::generate_godot_uid();
    // Remove the "uid://" prefix since templates add it themselves
    let uid_without_prefix = uid.trim_start_matches("uid://");
    Ok(Value::String(uid_without_prefix.to_string()))
}

/// Validates that destination directory is ready for writing
///
/// Checks:
/// - Destination directory exists
/// - No conflicting files exist (returns error with list if any found)
fn validate_destination(
    dest_path: &Path,
    template_structure: &TemplateStructure,
) -> Result<(), io::Error> {
    if !dest_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Destination directory does not exist: {}", dest_path.display()),
        ));
    }

    let mut conflicts = Vec::new();

    // Check for .tera file conflicts (with .tera extension stripped)
    for tera_file in &template_structure.tera_files {
        if let Some(relative_path) = template_structure.relative_paths.get(tera_file) {
            if let Some(relative_str) = relative_path.to_str() {
                if let Some(stripped) = relative_str.strip_suffix(".tera") {
                    let dest_file = dest_path.join(stripped);
                    if dest_file.exists() {
                        conflicts.push(dest_file);
                    }
                }
            }
        }
    }

    // Check for static file conflicts
    for static_file in &template_structure.static_files {
        if let Some(relative_path) = template_structure.relative_paths.get(static_file) {
            let dest_file = dest_path.join(relative_path);
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

/// Renders a single .tera file and writes it to destination
///
/// - Creates parent directories if needed
/// - Renders template with provided context
/// - Writes rendered content to destination file
fn render_and_write_file(
    tera: &Tera,
    template_file: &Path,
    dest_file: &Path,
    context: &Context,
    relative_path: &Path,
) -> Result<(), io::Error> {
    // Create parent directories if needed
    if let Some(parent) = dest_file.parent() {
        fs::create_dir_all(parent)?;
    }

    // Convert relative path to string for Tera (uses forward slashes)
    let template_name = relative_path
        .to_str()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Invalid template path (non-UTF8): {}", relative_path.display()),
            )
        })?
        .replace('\\', "/"); // Ensure forward slashes for cross-platform compatibility

    // Render template
    let rendered = tera.render(&template_name, context).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to render template '{}': {}", template_name, e),
        )
    })?;

    // Write rendered content to destination
    fs::write(dest_file, rendered)?;

    Ok(())
}
