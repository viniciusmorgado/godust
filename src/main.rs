mod pipeline;
mod utils;
use crate::pipeline::{
    boilerplate::{generate_core, generate_godot},
    structure::generate_structure,
};
use clap::Parser;
use std::io::{self};

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
    /// Template (available: blank)
    #[arg(short, long, default_value = "blank")]
    template: String,
}

fn main() {
    let args = Args::parse();
    let detected_casing = utils::naming::detect_casing(args.name.as_str());

    if detected_casing != utils::naming::Casing::SnakeCase {
        eprintln!(
            "Error: The project name '{}' is not in snake_case.",
            args.name
        );
        eprintln!("Please provide a name in snake_case (e.g., 'my_project_name').");
        return;
    }

    // Validate template exists
    let available_templates = vec!["blank"];
    if !available_templates.contains(&args.template.as_str()) {
        eprintln!(
            "Error: Template '{}' is not supported.",
            args.template
        );
        eprintln!("Available templates: {}", available_templates.join(", "));
        return;
    }

    match generate_structure(&args.name) {
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
        }
    }

    match generate_core(&args.name, &args.template, &args.rendering_method, &args.engine) {
        Ok(()) => {
            println!(
                "✅ Successfully created Rust project '{}_core'...",
                args.name
            );
        }
        Err(e) => {
            eprintln!("Error creating Rust project '{}': {}", args.name, e);
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied. Check your user permissions.");
                }
                io::ErrorKind::AlreadyExists => {
                    eprintln!(
                        "Project already exists. Please choose a different name or remove the existing directory."
                    );
                }
                _ => {
                    eprintln!("An unexpected I/O error occurred.");
                }
            }
        }
    }

    match generate_godot(&args.name, &args.template, &args.rendering_method, &args.engine) {
        Ok(()) => {
            println!("✅ Successfully created Godot project '{}'...", args.name);
        }
        Err(e) => {
            eprintln!("Error creating Godot project '{}': {}", args.name, e);
            match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied. Check your user permissions.");
                }
                io::ErrorKind::AlreadyExists => {
                    eprintln!(
                        "Project already exists. Please choose a different name or remove the existing directory."
                    );
                }
                _ => {
                    eprintln!("An unexpected I/O error occurred.");
                }
            }
        }
    }

    println!("- Godot project name: {}", args.name);
    println!("- Rust project name: {}_core", args.name);
    println!("- Engine version: {}", args.engine);
    println!("- Rendering method: {}", args.rendering_method);
}
