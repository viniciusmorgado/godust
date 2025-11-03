mod generators;
mod utils;

use clap::Parser;
use include_dir::{include_dir, Dir};
use std::io;
use std::path::Path;

use crate::utils::project::Project;
use crate::utils::template_parser;

// Embed templates at compile time
static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates");

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

/// Gets the embedded template directory by name
///
/// Templates are embedded in the binary at compile time.
fn get_embedded_template(template_name: &str) -> Result<&'static Dir<'static>, Box<dyn std::error::Error>> {
    TEMPLATES
        .get_dir(template_name)
        .ok_or_else(|| format!("Template '{}' not found in embedded templates", template_name).into())
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

    // Get embedded template
    let template_dir = get_embedded_template(&args.template)?;

    // Get the root directory (kebab-case name)
    let kebab_name =
        utils::naming::split_into_kebab_case(&args.name, utils::naming::Casing::SnakeCase);

    // Parse and generate all template files
    println!(
        "📝 Generating project files from '{}' template...",
        args.template
    );
    template_parser::parse_template(template_dir, Path::new(&kebab_name), context)?;

    // Build the Rust library automatically
    println!("🔨 Building Rust library (this may take a moment)...");
    let core_dir = Path::new(&kebab_name).join(format!("{}_core", args.name));

    let build_result = tokio::process::Command::new("cargo")
        .arg("build")
        .current_dir(&core_dir)
        .output()
        .await;

    match build_result {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Rust library built successfully!");
            } else {
                eprintln!("⚠️  Warning: Cargo build failed. You may need to run 'cargo build' manually.");
                if !output.stderr.is_empty() {
                    eprintln!("   Error: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️  Warning: Could not run cargo build: {}", e);
            eprintln!("   Please run 'cargo build' manually in {}_core/", args.name);
        }
    }

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
