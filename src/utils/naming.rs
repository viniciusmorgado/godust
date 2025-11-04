#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Casing {
    PascalCase = 1,
    CamelCase = 2,
    KebabCase = 3,
    SnakeCase = 4,
    Undefined = 5,
}

pub fn detect_casing(input: &str) -> Casing {
    if input.is_empty() {
        return Casing::Undefined;
    }

    let has_underscore = input.contains('_');
    let has_hyphen = input.contains('-');

    let has_other_chars = input
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'));

    if (has_underscore && has_hyphen) || has_other_chars {
        return Casing::Undefined;
    }

    if has_underscore && !has_hyphen {
        let all_chars_valid_for_snake = input.chars().all(|c| c.is_ascii_lowercase() || c == '_');
        if all_chars_valid_for_snake {
            return Casing::SnakeCase;
        } else {
            return Casing::Undefined;
        }
    }

    if has_hyphen && !has_underscore {
        let all_chars_valid_for_kebab = input.chars().all(|c| c.is_ascii_lowercase() || c == '-');
        if all_chars_valid_for_kebab {
            return Casing::KebabCase;
        } else {
            return Casing::Undefined;
        }
    }

    let first_char = input.chars().next().unwrap();
    let has_uppercase_internal = input.chars().skip(1).any(|c| c.is_ascii_uppercase());
    let has_lowercase_internal = input.chars().skip(1).any(|c| c.is_ascii_lowercase());

    if first_char.is_ascii_uppercase() {
        if has_lowercase_internal || !input.chars().skip(1).any(|c| c.is_ascii_uppercase()) {
            return Casing::PascalCase;
        }
    } else if first_char.is_ascii_lowercase() {
        if has_uppercase_internal {
            return Casing::CamelCase;
        }
    }

    if input.chars().all(|c| c.is_ascii_lowercase()) {
        return Casing::SnakeCase;
    }

    Casing::Undefined
}

fn capitalize_first_char(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase(),
    }
}

fn to_all_lowercase(s: &str) -> String {
    s.to_ascii_lowercase()
}

fn get_words_from_name(name: &str, casing: Casing) -> Vec<String> {
    match casing {
        Casing::SnakeCase => {
            if name.contains('_') {
                name.split('_')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_owned())
                    .collect()
            } else {
                vec![name.to_owned()]
            }
        }
        Casing::KebabCase => name
            .split('-')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect(),
        Casing::PascalCase | Casing::CamelCase => {
            let mut words = Vec::new();
            let chars: Vec<char> = name.chars().collect();
            if chars.is_empty() {
                return words;
            }

            let mut current_word_start_idx = 0;

            for i in 1..chars.len() {
                let prev_char = chars[i - 1];
                let current_char = chars[i];

                if prev_char.is_ascii_lowercase() && current_char.is_ascii_uppercase() {
                    words.push(chars[current_word_start_idx..i].iter().collect());
                    current_word_start_idx = i;
                } else if prev_char.is_ascii_uppercase()
                    && current_char.is_ascii_uppercase()
                    && i + 1 < chars.len()
                    && chars[i + 1].is_ascii_lowercase()
                {
                    words.push(chars[current_word_start_idx..i].iter().collect());
                    current_word_start_idx = i;
                }
            }
            words.push(chars[current_word_start_idx..].iter().collect());
            words
        }
        Casing::Undefined => {
            vec![name.to_owned()]
        }
    }
}

pub fn split_into_pascal_case(name: &String, detected_casing: Casing) -> String {
    if detected_casing == Casing::Undefined {
        return name.to_owned();
    }

    let words = get_words_from_name(name.as_str(), detected_casing);
    words
        .into_iter()
        .map(|word| capitalize_first_char(&word))
        .collect()
}

pub fn split_into_kebab_case(name: &str, detected_casing: Casing) -> String {
    if detected_casing == Casing::Undefined {
        return name.to_owned();
    }

    let words = get_words_from_name(name, detected_casing);
    words
        .into_iter()
        .map(|word| to_all_lowercase(&word))
        .collect::<Vec<String>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_casing_snake() {
        assert_eq!(detect_casing("my_project_name"), Casing::SnakeCase);
        assert_eq!(detect_casing("a_b_c"), Casing::SnakeCase);
        assert_eq!(detect_casing("just_one"), Casing::SnakeCase);
    }

    #[test]
    fn test_detect_casing_kebab() {
        assert_eq!(detect_casing("my-project-name"), Casing::KebabCase);
        assert_eq!(detect_casing("a-b-c"), Casing::KebabCase);
        assert_eq!(detect_casing("just-one"), Casing::KebabCase);
    }

    #[test]
    fn test_detect_casing_pascal() {
        assert_eq!(detect_casing("MyProjectName"), Casing::PascalCase);
        assert_eq!(detect_casing("FooBar"), Casing::PascalCase);
        assert_eq!(detect_casing("Project"), Casing::PascalCase);
        assert_eq!(detect_casing("HTTPRequestResponse"), Casing::PascalCase);
    }

    #[test]
    fn test_detect_casing_camel() {
        assert_eq!(detect_casing("myProjectName"), Casing::CamelCase);
        assert_eq!(detect_casing("fooBar"), Casing::CamelCase);
        assert_eq!(detect_casing("httpReqResp"), Casing::CamelCase);
    }

    #[test]
    fn test_detect_casing_undefined_mixed_delimiters() {
        assert_eq!(detect_casing("my_project-name"), Casing::Undefined);
        assert_eq!(detect_casing("my-project_name"), Casing::Undefined);
        assert_eq!(detect_casing("my_project-name_v2"), Casing::Undefined);
    }

    #[test]
    fn test_detect_casing_undefined_invalid_chars() {
        assert_eq!(detect_casing("my!project"), Casing::Undefined);
        assert_eq!(detect_casing("my project"), Casing::Undefined);
        assert_eq!(detect_casing("my@project"), Casing::Undefined);
        assert_eq!(detect_casing("my-project!"), Casing::Undefined);
    }

    #[test]
    fn test_detect_casing_undefined_all_caps() {
        assert_eq!(detect_casing("MYPROJECT"), Casing::Undefined);
        assert_eq!(detect_casing("ANOTHER_ONE"), Casing::Undefined);
        assert_eq!(detect_casing("SOME-THING"), Casing::Undefined);
    }

    #[test]
    fn test_detect_casing_undefined_all_lowercase_no_delimiter() {
        assert_eq!(detect_casing("myproject"), Casing::Undefined);
    }

    #[test]
    fn test_detect_casing_undefined_starts_with_number() {
        assert_eq!(detect_casing("123Project"), Casing::Undefined);
        assert_eq!(detect_casing("1_test"), Casing::Undefined);
    }

    #[test]
    fn test_detect_casing_undefined_snake_with_uppercase() {
        assert_eq!(detect_casing("My_project"), Casing::Undefined);
        assert_eq!(detect_casing("my_Project"), Casing::Undefined);
    }

    #[test]
    fn test_detect_casing_undefined_kebab_with_uppercase() {
        assert_eq!(detect_casing("My-project"), Casing::Undefined);
        assert_eq!(detect_casing("my-Project"), Casing::Undefined);
    }

    #[test]
    fn test_detect_casing_empty_string() {
        assert_eq!(detect_casing(""), Casing::Undefined);
    }

    #[test]
    fn convert_snake_to_pascal() {
        let name = "my_snake_case_name".to_string();
        assert_eq!(
            split_into_pascal_case(&name, Casing::SnakeCase),
            "MySnakeCaseName"
        );
    }

    #[test]
    fn convert_kebab_to_pascal() {
        let name = "my-kebab-case-name".to_string();
        assert_eq!(
            split_into_pascal_case(&name, Casing::KebabCase),
            "MyKebabCaseName"
        );
    }

    #[test]
    fn convert_camel_to_pascal() {
        let name = "myCamelCaseName".to_string();
        assert_eq!(
            split_into_pascal_case(&name, Casing::CamelCase),
            "MyCamelCaseName"
        );
    }

    #[test]
    fn convert_pascal_to_pascal() {
        let name = "MyPascalCaseName".to_string();
        assert_eq!(
            split_into_pascal_case(&name, Casing::PascalCase),
            "MyPascalCaseName"
        );
        let name_acronym = "HTTPRequestResponse".to_string();
        assert_eq!(
            split_into_pascal_case(&name_acronym, Casing::PascalCase),
            "HttpRequestResponse"
        );
    }

    #[test]
    fn full_flow_conversion_test() {
        let input_name = "MyAwesomeProject";
        let detected = detect_casing(input_name);
        assert_eq!(detected, Casing::PascalCase);

        let input_string = input_name.to_string();

        assert_eq!(
            split_into_kebab_case(&input_string, detected),
            "my-awesome-project"
        );

        assert_eq!(
            split_into_pascal_case(&input_string, detected),
            "MyAwesomeProject"
        );

        let input_name_snake = "another_game_thing";
        let detected_snake = detect_casing(input_name_snake);
        assert_eq!(detected_snake, Casing::SnakeCase);
        let input_snake_string = input_name_snake.to_string();
        assert_eq!(
            split_into_pascal_case(&input_snake_string, detected_snake),
            "AnotherGameThing"
        );
    }
}
