use std::collections::VecDeque;

pub struct BracelessParser;

#[derive(Clone, Copy)]
enum ScopeType {
    Regular,
    Lambda,
}

impl BracelessParser {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut indent_stack: VecDeque<(usize, ScopeType)> = VecDeque::new();

        for (_line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Insert empty lines, preprocessor directives, and comments as-is
            if trimmed.is_empty()
                || trimmed.starts_with('#')
                || trimmed.starts_with("//")
                || trimmed.starts_with("using")
                || trimmed.starts_with("typedef")
            {
                result.push(line.to_string());
                continue;
            }

            let current_indent = self.get_indent_level(line);

            while let Some(&(last_indent, scope_type)) = indent_stack.back() {
                if current_indent <= last_indent {
                    indent_stack.pop_back();
                    match scope_type {
                        ScopeType::Lambda => {
                            result.push(format!("{}}};", " ".repeat(last_indent)));
                        }
                        ScopeType::Regular => {
                            result.push(format!("{}}}", " ".repeat(last_indent)));
                        }
                    }
                } else {
                    break;
                }
            }

            if let Some(scope_type) = self.should_open_scope(trimmed) {
                let mut new_line = line.to_string();
                if !new_line.trim_end().ends_with('{') {
                    new_line.push_str(" {");
                }
                result.push(new_line);
                indent_stack.push_back((current_indent, scope_type));
            } else {
                result.push(line.to_string());
            }
        }

        while let Some((indent, scope_type)) = indent_stack.pop_back() {
            match scope_type {
                ScopeType::Lambda => {
                    result.push(format!("{}}};", " ".repeat(indent)));
                }
                ScopeType::Regular => {
                    result.push(format!("{}}}", " ".repeat(indent)));
                }
            }
        }

        Ok(result.join("\n"))
    }

    fn get_indent_level(&self, line: &str) -> usize {
        line.chars().take_while(|&c| c == ' ' || c == '\t').count()
    }

    fn should_open_scope(&self, line: &str) -> Option<ScopeType> {
        if line.ends_with('{') {
            return None;
        }

        // Check for lambda expressions
        if self.is_lambda(line) {
            return Some(ScopeType::Lambda);
        }

        let keywords = [
            "if",
            "else",
            "for",
            "while",
            "do",
            "switch",
            "try",
            "catch",
            "class",
            "struct",
            "enum",
            "namespace",
            "extern",
        ];

        let ends_with_paren = line.ends_with(')');
        let ends_with_colon = line.ends_with(':');
        let contains_keyword = keywords
            .iter()
            .any(|&kw| line.split_whitespace().any(|word| word == kw));

        let is_function = line.contains('(')
            && line.contains(')')
            && !line.contains("if")
            && !line.contains("while")
            && !line.contains("for")
            && !line.contains("switch");

        if ends_with_paren || ends_with_colon || contains_keyword || is_function {
            Some(ScopeType::Regular)
        } else {
            None
        }
    }

    fn is_lambda(&self, line: &str) -> bool {
        // Check for lambda syntax: contains [] and ends with )
        line.contains('[') && line.contains(']') && line.ends_with(')')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_processes_without_error() {
        let parser = BracelessParser::new();
        let input = "if (x > 0)\n    cout << \"positive\";";

        // Test that parser processes without error and adds braces
        let result = parser.process(input);
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("{"));
        assert!(output.contains("}"));
    }

    #[test]
    fn test_basic_control_structures() {
        let parser = BracelessParser::new();

        // Test if statement
        let if_input = "if (x > 0)\n    action();";
        let if_result = parser.process(if_input).unwrap();
        assert!(if_result.contains("if (x > 0) {"));
        assert!(if_result.contains("}"));

        // Test for loop
        let for_input = "for (int i = 0; i < 10; i++)\n    process(i);";
        let for_result = parser.process(for_input).unwrap();
        assert!(for_result.contains("for (int i = 0; i < 10; i++) {"));
        assert!(for_result.contains("}"));

        // Test while loop
        let while_input = "while (condition)\n    work();";
        let while_result = parser.process(while_input).unwrap();
        assert!(while_result.contains("while (condition) {"));
        assert!(while_result.contains("}"));
    }

    #[test]
    fn test_lambda_expressions() {
        let parser = BracelessParser::new();

        let lambda_input = "auto lambda = [&] (int m)\n    process(m);";
        let result = parser.process(lambda_input).unwrap();

        // Lambda should have opening brace and closing };
        assert!(result.contains("auto lambda = [&] (int m) {"));
        assert!(result.contains("};"));
    }

    #[test]
    fn test_function_definitions() {
        let parser = BracelessParser::new();

        let func_input = "int calculate(int x)\n    return x * 2;";
        let result = parser.process(func_input).unwrap();

        // Function should get braces added
        assert!(result.contains("int calculate(int x) {"));
        assert!(result.contains("}"));
    }

    #[test]
    fn test_class_and_struct() {
        let parser = BracelessParser::new();

        // Test class
        let class_input = "class MyClass\npublic:\n    void method();";
        let class_result = parser.process(class_input).unwrap();
        assert!(class_result.contains("class MyClass {"));
        assert!(class_result.contains("}"));

        // Test struct
        let struct_input = "struct Point\n    int x, y;";
        let struct_result = parser.process(struct_input).unwrap();
        assert!(struct_result.contains("struct Point {"));
        assert!(struct_result.contains("}"));
    }

    #[test]
    fn test_nested_structures() {
        let parser = BracelessParser::new();

        let nested_input =
            "if (a > 0)\n    if (b > 0)\n        action();\n    else\n        other_action();";
        let result = parser.process(nested_input).unwrap();

        // Should have multiple levels of braces
        let brace_count = result.matches('{').count();
        let close_brace_count = result.matches('}').count();
        assert!(brace_count >= 2); // At least 2 opening braces for nested structure
        assert_eq!(brace_count, close_brace_count); // Braces should be balanced
    }

    #[test]
    fn test_preserves_existing_braces() {
        let parser = BracelessParser::new();

        let braced_input = "if (condition) {\n    action();\n}";
        let result = parser.process(braced_input).unwrap();

        // Should not add extra braces to already braced code
        assert!(result.contains("if (condition) {"));
        assert!(result.contains("action();"));
        assert!(result.contains("}"));
    }

    #[test]
    fn test_preserves_preprocessor_and_comments() {
        let parser = BracelessParser::new();

        let input =
            "#include <iostream>\n// Comment\nusing namespace std;\nif (true)\n    action();";
        let result = parser.process(input).unwrap();

        // Preprocessor and comments should be unchanged
        assert!(result.contains("#include <iostream>"));
        assert!(result.contains("// Comment"));
        assert!(result.contains("using namespace std;"));
        assert!(result.contains("if (true) {"));
    }

    #[test]
    fn test_complex_lambda_with_control_flow() {
        let parser = BracelessParser::new();

        let complex_lambda = "auto processor = [&] (int value)\n    if (value > 10)\n        large_values.push_back(value);\n    else\n        small_values.push_back(value);";
        let result = parser.process(complex_lambda).unwrap();

        // Should handle lambda with nested control structures
        assert!(result.contains("auto processor = [&] (int value) {"));
        assert!(result.contains("if (value > 10) {"));
        assert!(result.contains("else {"));
        assert!(result.contains("};"));
    }
}
