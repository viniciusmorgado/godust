use clap::Parser;
mod models;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String,
}

fn main() {
    let args = Args::parse();

    let project = models::project::Project::new(args.name);
    match project.generate_structure() {
        Ok(_) => println!("Project structure created successfully."),
        Err(e) => eprintln!("Failed to create project structure: {}", e),
    }
}
