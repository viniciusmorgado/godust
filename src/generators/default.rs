use std::error::Error;
use tera::Context;

use crate::utils::{crates_version, naming};

pub async fn build_context(
    project_name: &str,
    engine_version: &str,
    rendering_method: &str,
    rendering_method_formatted: &str,
) -> Result<Context, Box<dyn Error>> {
    let mut context = Context::new();

    let detected_casing = naming::detect_casing(project_name);
    let kebab_name = naming::split_into_kebab_case(project_name, detected_casing);
    let pascal_name = naming::split_into_pascal_case( &project_name.to_string()
                                                           , detected_casing );

    let godot_version = crates_version::get_crate_version("godot")
        .await
        .unwrap_or_else(|_| "0.1.0".to_string());

    context.insert("project_name", project_name);
    context.insert("project_kebab_name", &kebab_name);
    context.insert("struct_name", &pascal_name);
    context.insert("godot_version", &godot_version);
    context.insert("engine_version", engine_version);
    context.insert("rendering_method", rendering_method);
    context.insert("rendering_method_formatted", rendering_method_formatted);

    Ok(context)
}
