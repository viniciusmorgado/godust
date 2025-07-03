use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::utils;

pub fn generate_core(name: &String) -> io::Result<()> {
    let godot_kebab_case_name =
        utils::naming::split_into_kebab_case(name, utils::naming::Casing::SnakeCase);
    let cargo_file = File::create(format!(
        "{}/{}_core/Cargo.toml",
        &godot_kebab_case_name, &name
    ))?;
    let mut cargo_file_writer = BufWriter::new(cargo_file);
    let godot_lib_version = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(utils::packages_version::get_crate_version("godot"))
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to get godot crate version: {}", e),
            )
        })?;

    cargo_file_writer.write_all(b"[package]\n")?;
    cargo_file_writer.write_all(format!("name = \"{}\"\n", name).as_bytes())?;
    cargo_file_writer.write_all(b"version = \"0.1.0\"\n")?;
    cargo_file_writer.write_all(b"edition = \"2024\"\n")?;
    cargo_file_writer.write_all(b"\n")?;
    cargo_file_writer.write_all(b"[dependencies]\n")?;
    cargo_file_writer.write_all(format!("godot = \"{}\"\n", godot_lib_version).as_bytes())?;
    cargo_file_writer.write_all(b"\n")?;
    cargo_file_writer.write_all(b"[lib]\n")?;
    cargo_file_writer.write_all(b"crate-type = [\"cdylib\"]\n")?;
    cargo_file_writer.flush()?;

    let lib_file = File::create(format!(
        "{}/{}_core/src/lib.rs",
        &godot_kebab_case_name, &name
    ))?;
    let mut lib_file_writer = BufWriter::new(lib_file);
    let struct_pascal_case_name =
        utils::naming::split_into_pascal_case(name, utils::naming::Casing::SnakeCase);

    lib_file_writer.write_all(b"use godot::prelude::*;\n")?;
    lib_file_writer.write_all(b"\n")?;
    lib_file_writer.write_all(format!("struct {};\n", &struct_pascal_case_name).as_bytes())?;
    lib_file_writer.write_all(b"\n")?;
    lib_file_writer.write_all(b"#[gdextension]\n")?;
    lib_file_writer.write_all(
        format!(
            "unsafe impl ExtensionLibrary for {} {{}}\n",
            &struct_pascal_case_name
        )
        .as_bytes(),
    )?;
    lib_file_writer.flush()?;

    Ok(())
}

pub fn generate_godot(
    name: &String,
    rendering_method: &String,
    engine_version: &String,
) -> io::Result<()> {
    let godot_kebab_case_name =
        utils::naming::split_into_kebab_case(name, utils::naming::Casing::SnakeCase);
    let project_godot = File::create(format!(
        "{}/{}/project.godot",
        &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    let mut project_godot_writer = BufWriter::new(project_godot);

    project_godot_writer.write_all(b"config_version=5\n")?;
    project_godot_writer.write_all(b"\n")?;
    project_godot_writer.write_all(b"[application]\n")?;
    project_godot_writer.write_all(b"\n")?;
    project_godot_writer
        .write_all(format!("config/name=\"{}\"\n", &godot_kebab_case_name).as_bytes())?;

    let formated_rendering_method =
        utils::rendering_method::format_rendering_method(&rendering_method);

    project_godot_writer.write_all(
        format!(
            "config/features=PackedStringArray(\"{}\", \"{}\") \n",
            &engine_version, formated_rendering_method
        )
        .as_bytes(),
    )?;

    project_godot_writer.write_all(b"config/icon=\"res://icon.svg\" \n")?;
    project_godot_writer.write_all(b"\n")?;
    project_godot_writer.write_all(b"[rendering]\n")?;
    project_godot_writer.write_all(b"\n")?;
    project_godot_writer
        .write_all(format!("renderer/rendering_method=\"{}\" \n", &rendering_method).as_bytes())?;
    const DEFAULT_ICON_SVG: &str = r###"
        <svg xmlns="http://www.w3.org/2000/svg" width="128" height="128"><rect width="124" height="124" x="2" y="2" fill="#363d52" stroke="#212532" stroke-width="4" rx="14"/><g fill="#fff" transform="translate(12.322 12.322)scale(.101)"><path d="M105 673v33q407 354 814 0v-33z"/><path fill="#478cbf" d="m105 673 152 14q12 1 15 14l4 67 132 10 8-61q2-11 15-15h162q13 4 15 15l8 61 132-10 4-67q3-13 15-14l152-14V427q30-39 56-81-35-59-83-108-43 20-82 47-40-37-88-64 7-51 8-102-59-28-123-42-26 43-46 89-49-7-98 0-20-46-46-89-64 14-123 42 1 51 8 102-48 27-88 64-39-27-82-47-48 49-83 108 26 42 56 81zm0 33v39c0 276 813 276 814 0v-39l-134 12-5 69q-2 10-14 13l-162 11q-12 0-16-11l-10-65H446l-10 65q-4 11-16 11l-162-11q-12-3-14-13l-5-69z"/><path d="M483 600c0 34 58 34 58 0v-86c0-34-58-34-58 0z"/><circle cx="725" cy="526" r="90"/><circle cx="299" cy="526" r="90"/></g><g fill="#414042" transform="translate(12.322 12.322)scale(.101)"><circle cx="307" cy="532" r="60"/><circle cx="717" cy="532" r="60"/></g></svg>
        "###;

    let godot_icon = File::create(format!(
        "{}/{}/icon.svg",
        &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    let mut godot_icon_writer = BufWriter::new(godot_icon);
    godot_icon_writer.write_all(format!("{}", DEFAULT_ICON_SVG).as_bytes())?;

    let gdextension = File::create(format!(
        "{}/{}/{}.gdextension",
        &godot_kebab_case_name, &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    let mut gdextension_writer = BufWriter::new(gdextension);

    gdextension_writer.write_all(b"[configuration]\n")?;
    gdextension_writer.write_all(b"entry_symbol = \"gdext_rust_init\"\n")?;
    gdextension_writer.write_all(b"compatibility_minimum = 4.1\n")?;
    gdextension_writer.write_all(b"reloadable = true\n")?;
    gdextension_writer.write_all(b"\n")?;
    gdextension_writer.write_all(b"[libraries]\n")?;
    gdextension_writer.write_all(
        format!(
            "linux.debug.x86_64 =     \"res://../{}_core/target/debug/lib{}.so\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;
    gdextension_writer.write_all(
        format!(
            "linux.release.x86_64 =   \"res://../{}_core/target/release/lib{}.so\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;
    gdextension_writer.write_all(
        format!(
            "windows.debug.x86_64 =   \"res://../{}_core/target/debug/{}.dll\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;
    gdextension_writer.write_all(
        format!(
            "windows.release.x86_64 = \"res://../{}_core/target/release/{}.dll\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;
    gdextension_writer.write_all(
        format!(
            "macos.debug =            \"res://../{}_core/target/debug/lib{}.dylib\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;
    gdextension_writer.write_all(
        format!(
            "macos.release =          \"res://../{}_core/target/release/lib{}.dylib\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;
    gdextension_writer.write_all(
        format!(
            "macos.debug.arm64 =      \"res://../{}_core/target/debug/lib{}.dylib\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;
    gdextension_writer.write_all(
        format!(
            "macos.release.arm64 =    \"res://../{}_core/target/release/lib{}.dylib\"\n",
            &name, &name
        )
        .as_bytes(),
    )?;

    Ok(())
}
