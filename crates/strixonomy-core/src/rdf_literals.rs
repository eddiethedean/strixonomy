/// Parse Turtle/RDF boolean literals (`true`, `"true"`, `"true"^^xsd:boolean`).
pub fn parse_boolean_literal(object: &str) -> Option<bool> {
    let trimmed = object.trim();
    if trimmed == "true" {
        return Some(true);
    }
    if trimmed == "false" {
        return Some(false);
    }
    if let Some(inner) = trimmed.strip_prefix('"') {
        let (literal, rest) = inner.split_once('"')?;
        if rest.is_empty() || rest.starts_with("^^") {
            return match literal {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            };
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_untrue_as_deprecated() {
        assert_eq!(parse_boolean_literal("untrue"), None);
        assert_eq!(parse_boolean_literal("\"untrue\""), None);
    }

    #[test]
    fn accepts_typed_boolean_true() {
        assert_eq!(
            parse_boolean_literal("\"true\"^^<http://www.w3.org/2001/XMLSchema#boolean>"),
            Some(true)
        );
    }
}
