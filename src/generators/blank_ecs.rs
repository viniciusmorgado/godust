use std::error::Error;
use tera::Context;

use crate::utils::{crates_version, naming};

/// Builds the Tera Context for the "blank_ecs" template
///
/// This template creates a Godot + Rust + Bevy ECS project with:
/// - Cargo.toml with godot and bevy_ecs dependencies
/// - lib.rs with GDExtension setup and examples module
/// - EntitySpawner example demonstrating Bevy ECS integration with Godot
/// - Godot project files (project.godot, extension.gdextension, Main.tscn, icon.svg)
///
/// # Arguments
/// * `project_name` - Snake case project name (e.g., "my_game")
/// * `engine_version` - Godot engine version (e.g., "4.3")
/// * `rendering_method` - Rendering method identifier (e.g., "forward_plus")
/// * `rendering_method_formatted` - Human-readable rendering method (e.g., "Forward Plus")
///
/// # Returns
/// A Tera Context with all required template variables
pub async fn build_context(
    project_name: &str,
    engine_version: &str,
    rendering_method: &str,
    rendering_method_formatted: &str,
) -> Result<Context, Box<dyn Error>> {
    let mut context = Context::new();

    // Derive name variations
    let detected_casing = naming::detect_casing(project_name);
    let kebab_name = naming::split_into_kebab_case(project_name, detected_casing);
    let pascal_name = naming::split_into_pascal_case(&project_name.to_string(), detected_casing);

    // Fetch latest crate versions from crates.io
    let godot_version = crates_version::get_crate_version("godot")
        .await
        .unwrap_or_else(|_| "0.1.0".to_string()); // Fallback if crates.io is unreachable

    let bevy_ecs_version = crates_version::get_crate_version("bevy_ecs")
        .await
        .unwrap_or_else(|_| "0.14.0".to_string()); // Fallback if crates.io is unreachable

    // Insert all template variables
    context.insert("project_name", project_name);
    context.insert("project_kebab_name", &kebab_name);
    context.insert("struct_name", &pascal_name);
    context.insert("godot_version", &godot_version);
    context.insert("bevy_ecs_version", &bevy_ecs_version);
    context.insert("engine_version", engine_version);
    context.insert("rendering_method", rendering_method);
    context.insert("rendering_method_formatted", rendering_method_formatted);

    Ok(context)
}
