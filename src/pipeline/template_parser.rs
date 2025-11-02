use std::collections::HashMap;
use std::io;
use tera::{Context, Tera, Value};

use crate::utils::uid_generator;

/// Initialize Tera engine with custom functions and template directory
pub fn init_tera() -> Result<Tera, io::Error> {
    // Load all .tera templates from src/templates directory
    let template_path = format!(
        "{}/src/templates/**/*.tera",
        env!("CARGO_MANIFEST_DIR")
    );

    let mut tera = Tera::new(&template_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to initialize Tera: {}", e),
        )
    })?;

    // Register custom function for UID generation
    tera.register_function("generate_uid", generate_uid_function);

    Ok(tera)
}

/// Custom Tera function to generate Godot UIDs
fn generate_uid_function(_args: &HashMap<String, Value>) -> tera::Result<Value> {
    let uid = uid_generator::generate_godot_uid();
    // Remove the "uid://" prefix since templates add it
    let uid_without_prefix = uid.trim_start_matches("uid://");
    Ok(Value::String(uid_without_prefix.to_string()))
}

/// Render all template files for a given template type
pub fn render_template(
    template_name: &str,
    context: Context,
) -> Result<HashMap<String, String>, io::Error> {
    let tera = init_tera()?;
    let mut rendered_files = HashMap::new();

    // List of template files to render for each template type
    let template_files = vec![
        ("Cargo.toml.tera", "Cargo.toml"),
        ("lib.rs.tera", "lib.rs"),
        ("project.godot.tera", "project.godot"),
        ("icon.svg.tera", "icon.svg"),
        ("extension.gdextension.tera", "extension.gdextension"),
        ("Main.tscn.tera", "Main.tscn"),
    ];

    for (template_file, output_file) in template_files {
        let template_path = format!("{}/{}", template_name, template_file);

        let rendered = tera.render(&template_path, &context).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to render template '{}': {}", template_path, e),
            )
        })?;

        rendered_files.insert(output_file.to_string(), rendered);
    }

    Ok(rendered_files)
}
