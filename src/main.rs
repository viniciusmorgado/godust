mod generators;
mod utils;

use clap::Parser;
use std::io;
use std::path::{Path, PathBuf};

use crate::utils::project::Project;
use crate::utils::template_parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Project name
    #[arg(short, long)]
    name: String,
    /// Engine version
    #[arg(short, long)]
    engine: String,
    /// Render method (forward_plus, gl_compatibility, mobile)
    #[arg(short, long)]
    rendering_method: String,
    /// Template (available: blank, blank_ecs)
    #[arg(short, long, default_value = "blank")]
    template: String,
}

/// Determines the runtime path to the template directory
///
/// Templates are expected to be in a "templates" folder next to the executable.
/// This works for both development (target/debug/templates) and production builds.
fn determine_template_path(template_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or("Failed to get executable directory")?;

    let template_path = exe_dir.join("templates").join(template_name);

    if !template_path.exists() {
        return Err(format!(
            "Template '{}' not found at: {}\nEnsure templates are installed correctly (run make.sh to copy templates).",
            template_name,
            template_path.display()
        )
        .into());
    }

    Ok(template_path)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let detected_casing = utils::naming::detect_casing(args.name.as_str());

    // Validate project name is snake_case
    if detected_casing != utils::naming::Casing::SnakeCase {
        eprintln!(
            "Error: The project name '{}' is not in snake_case.",
            args.name
        );
        eprintln!("Please provide a name in snake_case (e.g., 'my_project_name').");
        return Ok(());
    }

    // Validate template exists
    let available_templates = vec!["blank", "blank_ecs"];
    if !available_templates.contains(&args.template.as_str()) {
        eprintln!("Error: Template '{}' is not supported.", args.template);
        eprintln!("Available templates: {}", available_templates.join(", "));
        return Ok(());
    }

    // Create base directory structure
    let project = Project::default();
    match project.generate_structure(&args.name) {
        Ok(()) => {
            println!(
                "✅ Successfully created project structure for '{}'...",
                args.name
            );
        }
        Err(e) => {
            eprintln!(
                "Error creating project structure for '{}': {}",
                args.name, e
            );
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied. Check your user permissions.");
                }
                io::ErrorKind::AlreadyExists => {
                    eprintln!(
                        "Directory already exists. Please choose a different name or remove the existing directory."
                    );
                }
                _ => {
                    eprintln!("An unexpected I/O error occurred.");
                }
            }
            return Err(e.into());
        }
    }

    // Build context using appropriate generator based on template
    println!("🔨 Building template context...");
    let context = match args.template.as_str() {
        "blank" => {
            generators::blank::build_context(
                &args.name,
                &args.engine,
                &args.rendering_method,
                &Project::format_rendering_method(&args.rendering_method),
            )
            .await?
        }
        "blank_ecs" => {
            generators::blank_ecs::build_context(
                &args.name,
                &args.engine,
                &args.rendering_method,
                &Project::format_rendering_method(&args.rendering_method),
            )
            .await?
        }
        _ => {
            return Err(format!("Unknown template: {}", args.template).into());
        }
    };

    // Determine template location
    let template_path = determine_template_path(&args.template)?;

    // Get the root directory (kebab-case name)
    let kebab_name =
        utils::naming::split_into_kebab_case(&args.name, utils::naming::Casing::SnakeCase);

    // Parse and generate all template files
    println!(
        "📝 Generating project files from '{}' template...",
        args.template
    );
    template_parser::parse_template(&template_path, Path::new(&kebab_name), context)?;

    // Success summary
    println!(
        "\n✅ Successfully generated '{}' from template '{}'!",
        args.name, args.template
    );
    println!("\n📁 Project structure:");
    println!("   {}/", kebab_name);
    println!("   ├── {}/          (Godot project)", kebab_name);
    println!("   └── {}_core/     (Rust library)", args.name);
    println!("\n💡 Next steps:");
    println!("   cd {}", kebab_name);
    println!("   cd {}_core && cargo build", args.name);
    println!("   Open {} in Godot Editor", kebab_name);

    Ok(())
}
