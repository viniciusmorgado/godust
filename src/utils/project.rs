use std::{
    fs::{self},
    io::{self},
};

use crate::utils;

struct RenderingMethod {
    method: String,     // Forward Plus, GL Compatibility, Mobile
    identifier: String, //forward_plus, gl_compatibility, mobile
}

pub struct Project {
    core_project: String,   // rust project
    engine_project: String, // Godot project
    rendering_method: RenderingMethod,
}

const DEFAULT_CORE_TEMPLATE: &str = "{}/{}_core";
const DEFAULT_ENGINE_TEMPLATE: &str = "{}/{}";
const DEFAULT_RENDERING_METHOD: &str = "Forward Plus";
const DEFAULT_RENDERING_ID: &str = "forward_plus";

impl Default for Project {
    fn default() -> Self {
        Self {
            core_project: DEFAULT_CORE_TEMPLATE.to_string(),
            engine_project: DEFAULT_ENGINE_TEMPLATE.to_string(),
            rendering_method: RenderingMethod {
                method: DEFAULT_RENDERING_METHOD.to_string(),
                identifier: DEFAULT_RENDERING_ID.to_string(),
            },
        }
    }
}

impl Project {
    pub fn generate_structure(&self, name: &str) -> io::Result<()> {
        let kebab_name =
            utils::naming::split_into_kebab_case(name, utils::naming::Casing::SnakeCase);

        let core_path = self
            .core_project
            .replacen("{}", &kebab_name, 1)
            .replacen("{}", name, 1);

        let engine_path =
            self.engine_project
                .replacen("{}", &kebab_name, 1)
                .replacen("{}", &kebab_name, 1);

        fs::create_dir(&kebab_name)?;
        fs::create_dir(&engine_path)?;
        fs::create_dir(&core_path)?;
        fs::create_dir(format!("{}/src", &core_path))?;

        Ok(())
    }

    pub fn format_rendering_method(identifier: &str) -> String {
        match identifier {
            "forward_plus" => "Forward Plus".to_string(),
            "gl_compatibility" => "GL Compatibility".to_string(),
            "mobile" => "Mobile".to_string(),
            _ => "Forward Plus".to_string(),
        }
    }
}
