#[cfg(test)]
use super::*;

#[test]
fn test_replacement_in_expr() {
    let code = r#"
        fn main() {
            let b = "Hello, world!";
        }
    "#;
    let string_config = StringConfig::default();
    let mut string_obfuscator = StringObfuscator::new(string_config);
    let obfuscated_code = string_obfuscator.obfuscate_strings(code);
    assert_ne!(code, obfuscated_code);
    assert!(obfuscated_code.contains("encrypt_string"));

    let parse_result = syn::parse_file(&obfuscated_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}

#[test]
fn test_no_macro() {
    let code =
        r#"
        fn main() {
            println!("Hello, world!");
            let word_re = Regex::new(r"\b\w+\b").unwrap();
        }
    "#;
    let string_config = StringConfig::default();
    let mut string_obfuscator = StringObfuscator::new(string_config);
    let obfuscated_code = string_obfuscator.obfuscate_strings(code);
    assert!(!obfuscated_code.contains("encrypt_string"));
    let parse_result = syn::parse_file(&obfuscated_code);
    assert!(parse_result.is_ok(), "Modified code is not valid Rust code");
}
