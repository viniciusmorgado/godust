// TODO: Most content in this file is currently unused. Instead of trying to handle every name the user chooses,
// I now enforce snake_case on the Clap parameter. Review and reduce this file to only the functions that are currently in use.
// Attention: Some functions here still in use, be careful.

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
        Casing::SnakeCase => name
            .split('_')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect(),
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

pub fn split_into_snake_case(name: &String, detected_casing: Casing) -> String {
    if detected_casing == Casing::Undefined {
        return name.to_owned();
    }

    let words = get_words_from_name(name.as_str(), detected_casing);
    words
        .into_iter()
        .map(|word| to_all_lowercase(&word))
        .collect::<Vec<String>>()
        .join("_")
}

pub fn split_into_camel_case(name: &String, detected_casing: Casing) -> String {
    if detected_casing == Casing::Undefined {
        return name.to_owned();
    }

    let words = get_words_from_name(name.as_str(), detected_casing);
    let mut result = String::new();
    for (i, word) in words.into_iter().enumerate() {
        if i == 0 {
            result.push_str(&to_all_lowercase(&word));
        } else {
            result.push_str(&capitalize_first_char(&word));
        }
    }
    result
}

pub fn split_into_kebab_case(name: &String, detected_casing: Casing) -> String {
    if detected_casing == Casing::Undefined {
        return name.to_owned();
    }

    let words = get_words_from_name(name.as_str(), detected_casing);
    words
        .into_iter()
        .map(|word| to_all_lowercase(&word))
        .collect::<Vec<String>>()
        .join("-")
}

pub fn handle_undefined(name: &String, detected_casing: Casing) -> String {
    if detected_casing != Casing::Undefined {
        return name.to_owned();
    }

    name.as_str()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
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
    fn convert_pascal_to_snake() {
        let name = "MyPascalCaseName".to_string();
        assert_eq!(
            split_into_snake_case(&name, Casing::PascalCase),
            "my_pascal_case_name"
        );
        let name_acronym = "HTTPRequestResponse".to_string();
        assert_eq!(
            split_into_snake_case(&name_acronym, Casing::PascalCase),
            "http_request_response"
        );
    }

    #[test]
    fn convert_camel_to_snake() {
        let name = "myCamelCaseName".to_string();
        assert_eq!(
            split_into_snake_case(&name, Casing::CamelCase),
            "my_camel_case_name"
        );
    }

    #[test]
    fn convert_kebab_to_snake() {
        let name = "my-kebab-case-name".to_string();
        assert_eq!(
            split_into_snake_case(&name, Casing::KebabCase),
            "my_kebab_case_name"
        );
    }

    #[test]
    fn convert_snake_to_camel() {
        let name = "my_snake_case_name".to_string();
        assert_eq!(
            split_into_camel_case(&name, Casing::SnakeCase),
            "mySnakeCaseName"
        );
    }

    #[test]
    fn convert_kebab_to_camel() {
        let name = "my-kebab-case-name".to_string();
        assert_eq!(
            split_into_camel_case(&name, Casing::KebabCase),
            "myKebabCaseName"
        );
    }

    #[test]
    fn convert_pascal_to_camel() {
        let name = "MyPascalCaseName".to_string();
        assert_eq!(
            split_into_camel_case(&name, Casing::PascalCase),
            "myPascalCaseName"
        );
    }

    #[test]
    fn convert_camel_to_camel() {
        let name = "myCamelCaseName".to_string();
        assert_eq!(
            split_into_camel_case(&name, Casing::CamelCase),
            "myCamelCaseName"
        );
    }

    #[test]
    fn convert_snake_to_kebab() {
        let name = "my_snake_case_name".to_string();
        assert_eq!(
            split_into_kebab_case(&name, Casing::SnakeCase),
            "my-snake-case-name"
        );
    }

    #[test]
    fn convert_camel_to_kebab() {
        let name = "myCamelCaseName".to_string();
        assert_eq!(
            split_into_kebab_case(&name, Casing::CamelCase),
            "my-camel-case-name"
        );
    }

    #[test]
    fn convert_pascal_to_kebab() {
        let name = "MyPascalCaseName".to_string();
        assert_eq!(
            split_into_kebab_case(&name, Casing::PascalCase),
            "my-pascal-case-name"
        );
    }

    #[test]
    fn convert_kebab_to_kebab() {
        let name = "my-kebab-case-name".to_string();
        assert_eq!(
            split_into_kebab_case(&name, Casing::KebabCase),
            "my-kebab-case-name"
        );
    }

    // #[test]
    // fn test_handle_undefined_mixed_symbols_spaces() {
    //     let name = "Snake_caseGame-name with spaces!".to_string();
    //     assert_eq!(
    //         handle_undefined(&name, Casing::Undefined),
    //         "snakecasegamenameendswithspaces"
    //     );
    // }

    #[test]
    fn test_handle_undefined_with_symbols_and_numbers() {
        let name = "$_my@#name-123!".to_string();
        assert_eq!(handle_undefined(&name, Casing::Undefined), "myname123");
    }

    #[test]
    fn test_handle_undefined_already_clean() {
        let name = "cleanstring".to_string();
        assert_eq!(handle_undefined(&name, Casing::Undefined), "cleanstring");
    }

    #[test]
    fn test_handle_undefined_not_undefined_input() {
        let name = "MyProject".to_string();
        assert_eq!(handle_undefined(&name, Casing::PascalCase), "MyProject");
    }

    #[test]
    fn full_flow_conversion_test() {
        let input_name = "MyAwesomeProject";
        let detected = detect_casing(input_name);
        assert_eq!(detected, Casing::PascalCase);

        let input_string = input_name.to_string();
        assert_eq!(
            split_into_snake_case(&input_string, detected),
            "my_awesome_project"
        );
        assert_eq!(
            split_into_kebab_case(&input_string, detected),
            "my-awesome-project"
        );
        assert_eq!(
            split_into_camel_case(&input_string, detected),
            "myAwesomeProject"
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

    #[test]
    fn full_flow_undefined_test() {
        let input_name = "Project Name With Spaces!";
        let detected = detect_casing(input_name);
        assert_eq!(detected, Casing::Undefined);

        let input_string = input_name.to_string();
        assert_eq!(
            handle_undefined(&input_string, detected),
            "projectnamewithspaces"
        );

        assert_eq!(
            split_into_pascal_case(&input_string, detected),
            input_string
        );
        assert_eq!(split_into_snake_case(&input_string, detected), input_string);
    }
}
