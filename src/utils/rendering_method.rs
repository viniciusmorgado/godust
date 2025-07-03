pub fn format_rendering_method(rendering_method: &String) -> String {
    match rendering_method.as_str() {
        "forward_plus" => "Forward Plus".to_string(),
        "gl_compatibility" => "GL Compatibility".to_string(),
        "mobile" => "Mobile".to_string(),
        _ => "Forward Plus".to_string(),
    }
}
