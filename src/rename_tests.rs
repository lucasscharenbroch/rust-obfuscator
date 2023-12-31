#[cfg(test)]
use super::*;
use regex::Regex;

//function for testing
fn is_valid_rust_var_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    re.is_match(name)
}

#[test]
    fn test_variable_renamer() {
        let code = r#"
            fn calculate_sum(a: i32, b: i32) -> i32 {
                let result = a + b;
                result
            }

            fn main() {
                let mut num1 = 10;
                let num2 = 20;
                num1 = 30;
                let sum = calculate_sum(num1, num2);
                println!("The sum is: {}", sum);
            }
        "#;

        let ast = syn::parse_file(code).expect("Failed to parse code");
        let mut renamer = VariableRenamer { renamed_vars: HashMap::new() };
        let mut modified_ast = ast.clone();
        renamer.visit_file_mut(&mut modified_ast);
        
        let modified_code = quote!(#modified_ast).to_string();
        //compare the modified code with the original
        assert_ne!(modified_code, code);

        //check if names used are all valid rust variable names
        for new_name in renamer.renamed_vars.values() {
            assert!(is_valid_rust_var_name(new_name), "Invalid variable name: {}", new_name);
        }
        println!("{}", modified_code);
        //original names should not be found in modified code (except for sum TO DO: remove when string encryption is implemented)
        let original_names = vec!["calculate_sum", "result", "num1", "num2"];
        for name in original_names {
            assert!(
                !modified_code.contains(name),
                "Original name '{}' still found in modified code", 
                name
            );
        }

    }