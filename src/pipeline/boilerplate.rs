use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::pipeline::templates;
use crate::utils;

pub fn generate_core(
    name: &str,
    template: &str,
    rendering_method: &str,
    engine_version: &str,
) -> io::Result<()> {
    let godot_kebab_case_name =
        utils::naming::split_into_kebab_case(&name.to_string(), utils::naming::Casing::SnakeCase);

    // Render templates based on template type
    let rendered = match template {
        "blank" => templates::blank::generate(name, rendering_method, engine_version)?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown template: {}", template),
            ))
        }
    };

    // Write Cargo.toml
    let cargo_file = File::create(format!(
        "{}/{}_core/Cargo.toml",
        &godot_kebab_case_name, &name
    ))?;
    let mut cargo_writer = BufWriter::new(cargo_file);
    cargo_writer.write_all(rendered.get("Cargo.toml").unwrap().as_bytes())?;
    cargo_writer.flush()?;

    // Write lib.rs
    let lib_file = File::create(format!(
        "{}/{}_core/src/lib.rs",
        &godot_kebab_case_name, &name
    ))?;
    let mut lib_writer = BufWriter::new(lib_file);
    lib_writer.write_all(rendered.get("lib.rs").unwrap().as_bytes())?;
    lib_writer.flush()?;

    Ok(())
}

pub fn generate_godot(
    name: &str,
    template: &str,
    rendering_method: &str,
    engine_version: &str,
) -> io::Result<()> {
    let godot_kebab_case_name =
        utils::naming::split_into_kebab_case(&name.to_string(), utils::naming::Casing::SnakeCase);

    // Render templates based on template type
    let rendered = match template {
        "blank" => templates::blank::generate(name, rendering_method, engine_version)?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown template: {}", template),
            ))
        }
    };

    // Write project.godot
    let project_godot = File::create(format!(
        "{}/{}/project.godot",
        &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    let mut project_writer = BufWriter::new(project_godot);
    project_writer.write_all(rendered.get("project.godot").unwrap().as_bytes())?;
    project_writer.flush()?;

    // Write icon.svg
    let icon_file = File::create(format!(
        "{}/{}/icon.svg",
        &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    let mut icon_writer = BufWriter::new(icon_file);
    icon_writer.write_all(rendered.get("icon.svg").unwrap().as_bytes())?;
    icon_writer.flush()?;

    // Write .gdextension file
    let gdextension = File::create(format!(
        "{}/{}/{}.gdextension",
        &godot_kebab_case_name, &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    let mut gdext_writer = BufWriter::new(gdextension);
    gdext_writer.write_all(rendered.get("extension.gdextension").unwrap().as_bytes())?;
    gdext_writer.flush()?;

    // Write Main.tscn
    let main_scene = File::create(format!(
        "{}/{}/Main.tscn",
        &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    let mut scene_writer = BufWriter::new(main_scene);
    scene_writer.write_all(rendered.get("Main.tscn").unwrap().as_bytes())?;
    scene_writer.flush()?;

    Ok(())
}
