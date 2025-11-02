use std::collections::HashMap;
use std::io;
use tera::Context;

use crate::pipeline::template_parser;
use crate::utils;

/// Prepare context and render blank template
pub fn generate(
    project_name: &str,
    rendering_method: &str,
    engine_version: &str,
) -> Result<HashMap<String, String>, io::Error> {
    let mut context = Context::new();

    // Prepare all necessary variables for blank template
    let project_kebab_name =
        utils::naming::split_into_kebab_case(&project_name.to_string(), utils::naming::Casing::SnakeCase);
    let struct_name =
        utils::naming::split_into_pascal_case(&project_name.to_string(), utils::naming::Casing::SnakeCase);
    let rendering_method_formatted =
        utils::rendering_method::format_rendering_method(&rendering_method.to_string());

    // Fetch latest godot crate version
    let godot_version = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(utils::packages_version::get_crate_version("godot"))
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to get godot crate version: {}", e),
            )
        })?;

    // Insert all variables into context
    context.insert("project_name", project_name);
    context.insert("project_kebab_name", &project_kebab_name);
    context.insert("struct_name", &struct_name);
    context.insert("godot_version", &godot_version);
    context.insert("rendering_method", rendering_method);
    context.insert("rendering_method_formatted", &rendering_method_formatted);
    context.insert("engine_version", engine_version);

    // Render all template files
    template_parser::render_template("blank", context)
}
