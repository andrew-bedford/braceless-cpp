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
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.starts_with("using") || trimmed.starts_with("typedef") {
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
            "if", "else", "for", "while", "do", "switch", "try", "catch",
            "class", "struct", "enum", "namespace", "extern"
        ];

        let ends_with_paren = line.ends_with(')');
        let ends_with_colon = line.ends_with(':');
        let contains_keyword = keywords.iter().any(|&kw| {
            line.split_whitespace().any(|word| word == kw)
        });

        let is_function = line.contains('(') && line.contains(')') &&
                         !line.contains("if") && !line.contains("while") &&
                         !line.contains("for") && !line.contains("switch");

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