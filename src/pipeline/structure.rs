use std::{
    fs::{self},
    io::{self},
};

use crate::utils;

pub fn generate_structure(name: &String) -> io::Result<()> {
    let godot_kebab_case_name =
        utils::naming::split_into_kebab_case(name, utils::naming::Casing::SnakeCase);

    fs::create_dir(&godot_kebab_case_name)?;
    fs::create_dir(&format!(
        "{}/{}",
        &godot_kebab_case_name, &godot_kebab_case_name
    ))?;
    fs::create_dir(&format!("{}/{}_core", &godot_kebab_case_name, &name))?;
    fs::create_dir(&format!("{}/{}_core/src", &godot_kebab_case_name, &name))?;

    Ok(())
}
