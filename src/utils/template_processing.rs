use crate::utils::uid_generator::generate_godot_uid;
use std::collections::HashMap;

pub fn process_template_file(content: &str, variables: &HashMap<String, String>) -> String {
    let mut result = content.to_string();

    while result.contains("{{generate_uid}}") {
        result = result.replacen("{{generate_uid}}", &generate_godot_uid(), 1);
    }

    for (key, value) in variables {
        result = result.replace(&format!("{{{{{}}}}}", key), value);
    }

    result
}
