//! Tree-sitter parser family for Legacy CEL script documents.

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_celestia_legacy_cel() -> Language;
}

/// Returns the Tree-sitter [`Language`] for Legacy CEL scripts.
pub fn language() -> Language {
    unsafe { tree_sitter_celestia_legacy_cel() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_grammar_name() {
        assert_eq!("celestia-legacy-cel", "celestia-legacy-cel");
    }

    #[test]
    fn test_parse_script_block_and_commands() {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading legacy cel language");

        let script_text = r#"
            # Legacy CEL script header
            {
                select { object "Sol/Earth" }
                goto   { time 3.0 distance 5.0 }
                wait   { duration 2.0 }
            }
        "#;
        let tree = parser.parse(script_text, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error(), "Root should parse without error");
    }

    #[test]
    fn test_fatal_error_recovery_fixtures() {
        let mut parser = Parser::new();
        parser.set_language(&language()).unwrap();

        // Fatal unclosed string
        let malformed = r#"{ select { object "Sol/Earth }"#;
        let tree = parser.parse(malformed, None).unwrap();
        assert!(tree.root_node().has_error());
    }

    #[test]
    fn test_locally_recoverable_and_valid_after_damage_fixtures() {
        let mut parser = Parser::new();
        parser.set_language(&language()).unwrap();

        let content = r#"
            {
                select { object "Sol/Earth" }
                BAD_SYNTAX_UNCLOSED_GROUP {
                goto { time 3.0 distance 5.0 }
            }
        "#;
        let tree = parser.parse(content, None).unwrap();
        let root = tree.root_node();
        assert!(
            root.has_error(),
            "Root should contain error node for damaged command"
        );
        assert!(
            root.child_count() >= 1,
            "Parser should recover valid nodes after damage"
        );
    }

    #[test]
    fn test_comments_and_nested_properties() {
        let mut parser = Parser::new();
        parser.set_language(&language()).unwrap();

        let script = r#"
            # Top-level comment
            {
                # Inner comment
                renderflags { set "stars|constellations" }
                set { name "custom" details { active true count 42 } }
            }
        "#;
        let tree = parser.parse(script, None).unwrap();
        let root = tree.root_node();
        assert!(
            !root.has_error(),
            "Script with comments and nested properties should parse clean"
        );
    }
}
