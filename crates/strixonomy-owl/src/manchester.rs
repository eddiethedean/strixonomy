use crate::error::{OwlError, Result};
use horned_owl::model::{
    Build, ClassExpression, DataRange, FacetRestriction, Individual, Literal,
    ObjectPropertyExpression, RcStr,
};
use horned_owl::vocab::Facet;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Debug, Clone, Serialize)]
pub struct ManchesterDiagnostic {
    pub message: String,
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct ManchesterParseOutput {
    pub normalized: String,
    pub expression: ClassExpression<RcStr>,
    pub tree: serde_json::Value,
    pub diagnostics: Vec<ManchesterDiagnostic>,
}

pub fn parse_class_expression(
    input: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<ManchesterParseOutput> {
    parse_class_expression_with_datatypes(input, namespaces, &std::collections::BTreeSet::new())
}

/// Parse Manchester with an optional set of known datatype IRIs (DeclareDatatype / definitions).
pub fn parse_class_expression_with_datatypes(
    input: &str,
    namespaces: &BTreeMap<String, String>,
    known_datatypes: &std::collections::BTreeSet<String>,
) -> Result<ManchesterParseOutput> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(OwlError::ManchesterInvalid("empty expression".to_string()));
    }

    let tokens = tokenize(trimmed).map_err(OwlError::ManchesterInvalid)?;
    let mut parser = ManchesterParser { tokens, pos: 0 };
    let ast = parser.parse_expression().map_err(OwlError::ManchesterInvalid)?;
    if !parser.is_at_end() {
        return Err(OwlError::ManchesterInvalid(format!("unexpected token: {:?}", parser.peek())));
    }

    let build = Build::default();
    let expression = ast_to_class_expression(&ast, &build, namespaces, known_datatypes)?;
    let normalized = class_expression_to_manchester(&expression, namespaces);
    let tree = expression_tree_json(&expression, namespaces);
    Ok(ManchesterParseOutput { normalized, expression, tree, diagnostics: Vec::new() })
}

pub fn class_expression_to_turtle_fragment(
    expr: &ClassExpression<RcStr>,
    predicate: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    let value = class_expression_to_turtle_value(expr, namespaces, 0)?;
    Ok(format!("    {predicate} {value} ;\n"))
}

pub fn class_expression_to_manchester(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> String {
    match expr {
        ClassExpression::Class(c) => iri_to_manchester_term(&c.to_string(), namespaces),
        ClassExpression::ObjectIntersectionOf(v) => {
            let parts: Vec<String> =
                v.iter().map(|e| class_expression_to_manchester(e, namespaces)).collect();
            if parts.len() == 1 {
                parts[0].clone()
            } else {
                parts.join(" and ")
            }
        }
        ClassExpression::ObjectUnionOf(v) => {
            let parts: Vec<String> =
                v.iter().map(|e| class_expression_to_manchester(e, namespaces)).collect();
            if parts.len() == 1 {
                parts[0].clone()
            } else {
                format!("({})", parts.join(" or "))
            }
        }
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
            let prop = ope_to_iri(ope);
            let filler = class_expression_to_manchester(bce, namespaces);
            format!("{} some {}", iri_to_manchester_term(&prop, namespaces), filler)
        }
        ClassExpression::ObjectAllValuesFrom { ope, bce } => {
            let prop = ope_to_iri(ope);
            let filler = class_expression_to_manchester(bce, namespaces);
            format!("{} only {}", iri_to_manchester_term(&prop, namespaces), filler)
        }
        ClassExpression::ObjectMinCardinality { n, ope, bce } => cardinality_manchester(
            &iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "min",
            *n,
            bce,
            namespaces,
        ),
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => cardinality_manchester(
            &iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "max",
            *n,
            bce,
            namespaces,
        ),
        ClassExpression::ObjectExactCardinality { n, ope, bce } => cardinality_manchester(
            &iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "exactly",
            *n,
            bce,
            namespaces,
        ),
        ClassExpression::ObjectComplementOf(inner) => {
            let body = class_expression_to_manchester(inner, namespaces);
            format!("not ({body})")
        }
        ClassExpression::ObjectHasValue { ope, i } => {
            format!(
                "{} value {}",
                iri_to_manchester_term(&ope_to_iri(ope), namespaces),
                iri_to_manchester_term(i, namespaces)
            )
        }
        ClassExpression::ObjectHasSelf(ope) => {
            format!("{} Self", iri_to_manchester_term(&ope_to_iri(ope), namespaces))
        }
        ClassExpression::ObjectOneOf(inds) => {
            let parts: Vec<String> =
                inds.iter().map(|i| iri_to_manchester_term(i, namespaces)).collect();
            format!("{{ {} }}", parts.join(" "))
        }
        ClassExpression::DataSomeValuesFrom { dp, dr } => {
            format!(
                "{} some {}",
                iri_to_manchester_term(dp.0.as_ref(), namespaces),
                data_range_to_manchester(dr, namespaces)
            )
        }
        ClassExpression::DataAllValuesFrom { dp, dr } => {
            format!(
                "{} only {}",
                iri_to_manchester_term(dp.0.as_ref(), namespaces),
                data_range_to_manchester(dr, namespaces)
            )
        }
        ClassExpression::DataHasValue { dp, l } => {
            format!(
                "{} value \"{}\"",
                iri_to_manchester_term(dp.0.as_ref(), namespaces),
                literal_lexical(l)
            )
        }
        ClassExpression::DataMinCardinality { n, dp, dr } => {
            data_cardinality_manchester(dp.0.as_ref(), "min", *n, dr, namespaces)
        }
        ClassExpression::DataMaxCardinality { n, dp, dr } => {
            data_cardinality_manchester(dp.0.as_ref(), "max", *n, dr, namespaces)
        }
        ClassExpression::DataExactCardinality { n, dp, dr } => {
            data_cardinality_manchester(dp.0.as_ref(), "exactly", *n, dr, namespaces)
        }
    }
}

pub fn expression_tree_json(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> serde_json::Value {
    match expr {
        ClassExpression::Class(c) => serde_json::json!({
            "kind": "Class",
            "label": iri_to_manchester_term(&c.to_string(), namespaces),
        }),
        ClassExpression::ObjectIntersectionOf(v) => serde_json::json!({
            "kind": "ObjectIntersectionOf",
            "children": v.iter().map(|e| expression_tree_json(e, namespaces)).collect::<Vec<_>>(),
        }),
        ClassExpression::ObjectUnionOf(v) => serde_json::json!({
            "kind": "ObjectUnionOf",
            "children": v.iter().map(|e| expression_tree_json(e, namespaces)).collect::<Vec<_>>(),
        }),
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => serde_json::json!({
            "kind": "ObjectSomeValuesFrom",
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectAllValuesFrom { ope, bce } => serde_json::json!({
            "kind": "ObjectAllValuesFrom",
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectMinCardinality { n, ope, bce } => serde_json::json!({
            "kind": "ObjectMinCardinality",
            "cardinality": n,
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => serde_json::json!({
            "kind": "ObjectMaxCardinality",
            "cardinality": n,
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectExactCardinality { n, ope, bce } => serde_json::json!({
            "kind": "ObjectExactCardinality",
            "cardinality": n,
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "filler": expression_tree_json(bce, namespaces),
        }),
        ClassExpression::ObjectComplementOf(inner) => serde_json::json!({
            "kind": "ObjectComplementOf",
            "filler": expression_tree_json(inner, namespaces),
        }),
        ClassExpression::ObjectHasValue { ope, i } => serde_json::json!({
            "kind": "ObjectHasValue",
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
            "individual": iri_to_manchester_term(i, namespaces),
        }),
        ClassExpression::ObjectHasSelf(ope) => serde_json::json!({
            "kind": "ObjectHasSelf",
            "property": iri_to_manchester_term(&ope_to_iri(ope), namespaces),
        }),
        ClassExpression::ObjectOneOf(inds) => serde_json::json!({
            "kind": "ObjectOneOf",
            "individuals": inds.iter().map(|i| iri_to_manchester_term(i, namespaces)).collect::<Vec<_>>(),
        }),
        ClassExpression::DataSomeValuesFrom { dp, dr } => serde_json::json!({
            "kind": "DataSomeValuesFrom",
            "property": iri_to_manchester_term(dp.0.as_ref(), namespaces),
            "range": data_range_to_manchester(dr, namespaces),
        }),
        ClassExpression::DataAllValuesFrom { dp, dr } => serde_json::json!({
            "kind": "DataAllValuesFrom",
            "property": iri_to_manchester_term(dp.0.as_ref(), namespaces),
            "range": data_range_to_manchester(dr, namespaces),
        }),
        ClassExpression::DataHasValue { dp, l } => serde_json::json!({
            "kind": "DataHasValue",
            "property": iri_to_manchester_term(dp.0.as_ref(), namespaces),
            "literal": literal_lexical(l),
        }),
        other => serde_json::json!({ "kind": format!("{other:?}") }),
    }
}

#[derive(Debug, Clone)]
enum ManchesterAst {
    Class(String),
    Some { property: String, filler: Box<ManchesterAst> },
    Only { property: String, filler: Box<ManchesterAst> },
    And(Vec<ManchesterAst>),
    Or(Vec<ManchesterAst>),
    Min { n: u32, property: String, filler: Box<ManchesterAst> },
    Max { n: u32, property: String, filler: Box<ManchesterAst> },
    Exactly { n: u32, property: String, filler: Box<ManchesterAst> },
    Not(Box<ManchesterAst>),
    HasValue { property: String, individual: String },
    HasSelf { property: String },
    OneOf(Vec<String>),
    DataHasValue { property: String, literal: String },
}

fn ast_to_class_expression(
    ast: &ManchesterAst,
    build: &Build<RcStr>,
    namespaces: &BTreeMap<String, String>,
    known_datatypes: &std::collections::BTreeSet<String>,
) -> Result<ClassExpression<RcStr>> {
    match ast {
        ManchesterAst::Class(iri) => {
            let resolved = resolve_term_iri(iri, namespaces)?;
            Ok(ClassExpression::Class(build.class(resolved)))
        }
        ManchesterAst::Some { property, filler } => {
            let prop_iri = resolve_term_iri(property, namespaces)?;
            if let Some(dr) = filler_as_data_range(filler, build, namespaces, known_datatypes)? {
                Ok(ClassExpression::DataSomeValuesFrom { dp: build.data_property(prop_iri), dr })
            } else {
                let bce =
                    Box::new(ast_to_class_expression(filler, build, namespaces, known_datatypes)?);
                Ok(ClassExpression::ObjectSomeValuesFrom {
                    ope: ObjectPropertyExpression::ObjectProperty(build.object_property(prop_iri)),
                    bce,
                })
            }
        }
        ManchesterAst::Only { property, filler } => {
            let prop_iri = resolve_term_iri(property, namespaces)?;
            if let Some(dr) = filler_as_data_range(filler, build, namespaces, known_datatypes)? {
                Ok(ClassExpression::DataAllValuesFrom { dp: build.data_property(prop_iri), dr })
            } else {
                let bce =
                    Box::new(ast_to_class_expression(filler, build, namespaces, known_datatypes)?);
                Ok(ClassExpression::ObjectAllValuesFrom {
                    ope: ObjectPropertyExpression::ObjectProperty(build.object_property(prop_iri)),
                    bce,
                })
            }
        }
        ManchesterAst::And(items) => {
            let exprs: Result<Vec<_>> = items
                .iter()
                .map(|i| ast_to_class_expression(i, build, namespaces, known_datatypes))
                .collect();
            Ok(ClassExpression::ObjectIntersectionOf(exprs?))
        }
        ManchesterAst::Or(items) => {
            let exprs: Result<Vec<_>> = items
                .iter()
                .map(|i| ast_to_class_expression(i, build, namespaces, known_datatypes))
                .collect();
            Ok(ClassExpression::ObjectUnionOf(exprs?))
        }
        ManchesterAst::Min { n, property, filler } => {
            let prop_iri = resolve_term_iri(property, namespaces)?;
            if let Some(dr) = filler_as_data_range(filler, build, namespaces, known_datatypes)? {
                Ok(ClassExpression::DataMinCardinality {
                    n: *n,
                    dp: build.data_property(prop_iri),
                    dr,
                })
            } else {
                let bce =
                    Box::new(ast_to_class_expression(filler, build, namespaces, known_datatypes)?);
                Ok(ClassExpression::ObjectMinCardinality {
                    n: *n,
                    ope: ObjectPropertyExpression::ObjectProperty(build.object_property(prop_iri)),
                    bce,
                })
            }
        }
        ManchesterAst::Max { n, property, filler } => {
            let prop_iri = resolve_term_iri(property, namespaces)?;
            if let Some(dr) = filler_as_data_range(filler, build, namespaces, known_datatypes)? {
                Ok(ClassExpression::DataMaxCardinality {
                    n: *n,
                    dp: build.data_property(prop_iri),
                    dr,
                })
            } else {
                let bce =
                    Box::new(ast_to_class_expression(filler, build, namespaces, known_datatypes)?);
                Ok(ClassExpression::ObjectMaxCardinality {
                    n: *n,
                    ope: ObjectPropertyExpression::ObjectProperty(build.object_property(prop_iri)),
                    bce,
                })
            }
        }
        ManchesterAst::Exactly { n, property, filler } => {
            let prop_iri = resolve_term_iri(property, namespaces)?;
            if let Some(dr) = filler_as_data_range(filler, build, namespaces, known_datatypes)? {
                Ok(ClassExpression::DataExactCardinality {
                    n: *n,
                    dp: build.data_property(prop_iri),
                    dr,
                })
            } else {
                let bce =
                    Box::new(ast_to_class_expression(filler, build, namespaces, known_datatypes)?);
                Ok(ClassExpression::ObjectExactCardinality {
                    n: *n,
                    ope: ObjectPropertyExpression::ObjectProperty(build.object_property(prop_iri)),
                    bce,
                })
            }
        }
        ManchesterAst::Not(inner) => {
            let ce = ast_to_class_expression(inner, build, namespaces, known_datatypes)?;
            Ok(ClassExpression::ObjectComplementOf(Box::new(ce)))
        }
        ManchesterAst::HasValue { property, individual } => {
            let prop = build.object_property(resolve_term_iri(property, namespaces)?);
            let ind: Individual<RcStr> =
                build.named_individual(resolve_term_iri(individual, namespaces)?).into();
            Ok(ClassExpression::ObjectHasValue {
                ope: ObjectPropertyExpression::ObjectProperty(prop),
                i: ind,
            })
        }
        ManchesterAst::HasSelf { property } => {
            let prop = build.object_property(resolve_term_iri(property, namespaces)?);
            Ok(ClassExpression::ObjectHasSelf(ObjectPropertyExpression::ObjectProperty(prop)))
        }
        ManchesterAst::OneOf(inds) => {
            let individuals: Result<Vec<_>> = inds
                .iter()
                .map(|i| {
                    Ok(Individual::from(build.named_individual(resolve_term_iri(i, namespaces)?)))
                })
                .collect();
            Ok(ClassExpression::ObjectOneOf(individuals?))
        }
        ManchesterAst::DataHasValue { property, literal } => Ok(ClassExpression::DataHasValue {
            dp: build.data_property(resolve_term_iri(property, namespaces)?),
            l: Literal::Simple { literal: literal.clone() },
        }),
    }
}

fn filler_as_data_range(
    filler: &ManchesterAst,
    _build: &Build<RcStr>,
    namespaces: &BTreeMap<String, String>,
    known_datatypes: &std::collections::BTreeSet<String>,
) -> Result<Option<DataRange<RcStr>>> {
    match filler {
        ManchesterAst::Class(term) => {
            // Only interpret as a data range when the base term looks like a datatype
            // (avoid treating class fillers such as owl:Thing as DataRanges).
            let base = term.split('[').next().unwrap_or(term).trim();
            let iri = resolve_term_iri(base, namespaces)?;
            if !looks_like_datatype_iri(&iri, base, known_datatypes) {
                return Ok(None);
            }
            // Facet/parse failures must surface (#335) — do not silently degrade to bare Datatype.
            let dr = parse_data_range(term, namespaces)?;
            Ok(Some(dr))
        }
        _ => Ok(None),
    }
}

/// Parse a Manchester-style data range (datatype, facets, oneOf, and/or/not).
pub fn parse_data_range(
    input: &str,
    namespaces: &BTreeMap<String, String>,
) -> Result<DataRange<RcStr>> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(OwlError::ManchesterInvalid("empty data range".to_string()));
    }
    let mut parser = DataRangeParser { input: trimmed, pos: 0, namespaces };
    let dr = parser.parse_or()?;
    parser.skip_ws();
    if parser.pos < parser.input.len() {
        return Err(OwlError::ManchesterInvalid(format!(
            "unexpected trailing input in data range: '{}'",
            &parser.input[parser.pos..]
        )));
    }
    Ok(dr)
}

struct DataRangeParser<'a> {
    input: &'a str,
    pos: usize,
    namespaces: &'a BTreeMap<String, String>,
}

impl<'a> DataRangeParser<'a> {
    fn skip_ws(&mut self) {
        while let Some(c) = self.input[self.pos..].chars().next() {
            if !c.is_whitespace() {
                break;
            }
            self.pos += c.len_utf8();
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn parse_or(&mut self) -> Result<DataRange<RcStr>> {
        let mut left = self.parse_and()?;
        loop {
            self.skip_ws();
            if self.consume_keyword("or") {
                let right = self.parse_and()?;
                left = match left {
                    DataRange::DataUnionOf(mut v) => {
                        v.push(right);
                        DataRange::DataUnionOf(v)
                    }
                    other => DataRange::DataUnionOf(vec![other, right]),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<DataRange<RcStr>> {
        let mut left = self.parse_unary()?;
        loop {
            self.skip_ws();
            if self.consume_keyword("and") {
                let right = self.parse_unary()?;
                left = match left {
                    DataRange::DataIntersectionOf(mut v) => {
                        v.push(right);
                        DataRange::DataIntersectionOf(v)
                    }
                    other => DataRange::DataIntersectionOf(vec![other, right]),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<DataRange<RcStr>> {
        self.skip_ws();
        if self.consume_keyword("not") {
            let inner = self.parse_unary()?;
            return Ok(DataRange::DataComplementOf(Box::new(inner)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<DataRange<RcStr>> {
        self.skip_ws();
        if self.peek() == Some('{') {
            return self.parse_one_of();
        }
        if self.peek() == Some('(') {
            self.bump();
            let inner = self.parse_or()?;
            self.skip_ws();
            if self.bump() != Some(')') {
                return Err(OwlError::ManchesterInvalid("expected ')' in data range".to_string()));
            }
            return Ok(inner);
        }
        let name = self.parse_name()?;
        let iri = resolve_term_iri(&name, self.namespaces)?;
        let build = Build::new();
        let datatype = build.datatype(iri.as_str());
        let mut facets = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() != Some('[') {
                break;
            }
            facets.push(self.parse_facet_bracket()?);
        }
        if facets.is_empty() {
            Ok(DataRange::Datatype(datatype))
        } else {
            Ok(DataRange::DatatypeRestriction(datatype, facets))
        }
    }

    fn parse_one_of(&mut self) -> Result<DataRange<RcStr>> {
        self.bump(); // {
        let mut lits = Vec::new();
        loop {
            self.skip_ws();
            if self.peek() == Some('}') {
                self.bump();
                break;
            }
            lits.push(self.parse_literal_value()?);
            self.skip_ws();
            if self.peek() == Some(',') {
                self.bump();
                continue;
            }
            if self.peek() == Some('}') {
                self.bump();
                break;
            }
            return Err(OwlError::ManchesterInvalid(
                "expected ',' or '}' in data oneOf".to_string(),
            ));
        }
        Ok(DataRange::DataOneOf(lits))
    }

    fn parse_literal_value(&mut self) -> Result<Literal<RcStr>> {
        self.skip_ws();
        if self.peek() == Some('"') {
            self.bump();
            let mut lit = String::new();
            while let Some(c) = self.bump() {
                if c == '"' {
                    break;
                }
                if c == '\\' {
                    if let Some(n) = self.bump() {
                        lit.push(n);
                    }
                } else {
                    lit.push(c);
                }
            }
            return Ok(Literal::Simple { literal: lit });
        }
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == ',' || c == '}' || c == ')' || c == ']' {
                break;
            }
            self.bump();
        }
        let raw = self.input[start..self.pos].trim();
        if raw.is_empty() {
            return Err(OwlError::ManchesterInvalid("empty literal in data oneOf".into()));
        }
        Ok(Literal::Simple { literal: raw.to_string() })
    }

    fn parse_facet_bracket(&mut self) -> Result<FacetRestriction<RcStr>> {
        self.bump(); // [
        self.skip_ws();
        let (facet, value) = if self.consume_symbol(">=") {
            (Facet::MinInclusive, self.parse_facet_value()?)
        } else if self.consume_symbol("<=") {
            (Facet::MaxInclusive, self.parse_facet_value()?)
        } else if self.consume_symbol(">") {
            (Facet::MinExclusive, self.parse_facet_value()?)
        } else if self.consume_symbol("<") {
            (Facet::MaxExclusive, self.parse_facet_value()?)
        } else if self.consume_keyword("length") {
            (Facet::Length, self.parse_facet_value()?)
        } else if self.consume_keyword("minLength") {
            (Facet::MinLength, self.parse_facet_value()?)
        } else if self.consume_keyword("maxLength") {
            (Facet::MaxLength, self.parse_facet_value()?)
        } else if self.consume_keyword("pattern") {
            (Facet::Pattern, self.parse_facet_value()?)
        } else if self.consume_keyword("totalDigits") {
            (Facet::TotalDigits, self.parse_facet_value()?)
        } else if self.consume_keyword("fractionDigits") {
            (Facet::FractionDigits, self.parse_facet_value()?)
        } else {
            return Err(OwlError::ManchesterInvalid(
                "unknown facet in datatype restriction".to_string(),
            ));
        };
        self.skip_ws();
        if self.bump() != Some(']') {
            return Err(OwlError::ManchesterInvalid("expected ']' after facet".to_string()));
        }
        Ok(FacetRestriction { f: facet, l: value })
    }

    fn parse_facet_value(&mut self) -> Result<Literal<RcStr>> {
        self.skip_ws();
        self.parse_literal_value()
    }

    fn parse_name(&mut self) -> Result<String> {
        self.skip_ws();
        if self.peek() == Some('<') {
            self.bump();
            let start = self.pos;
            while let Some(c) = self.bump() {
                if c == '>' {
                    return Ok(format!("<{}>", &self.input[start..self.pos - 1]));
                }
            }
            return Err(OwlError::ManchesterInvalid("unclosed IRI in data range".into()));
        }
        let start = self.pos;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == ':' || c == '_' || c == '-' || c == '.' {
                self.bump();
            } else {
                break;
            }
        }
        let name = self.input[start..self.pos].to_string();
        if name.is_empty() {
            return Err(OwlError::ManchesterInvalid("expected datatype name".into()));
        }
        Ok(name)
    }

    fn consume_keyword(&mut self, kw: &str) -> bool {
        self.skip_ws();
        let rest = &self.input[self.pos..];
        if rest.len() >= kw.len()
            && rest[..kw.len()].eq_ignore_ascii_case(kw)
            && rest
                .get(kw.len()..)
                .and_then(|s| s.chars().next())
                .map(|c| !c.is_alphanumeric() && c != '_')
                .unwrap_or(true)
        {
            self.pos += kw.len();
            true
        } else {
            false
        }
    }

    fn consume_symbol(&mut self, sym: &str) -> bool {
        self.skip_ws();
        if self.input[self.pos..].starts_with(sym) {
            self.pos += sym.len();
            true
        } else {
            false
        }
    }
}

/// Pretty-print a data range in Manchester-ish form for Inspector / PatchOp payloads.
pub fn data_range_to_manchester(
    dr: &DataRange<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> String {
    match dr {
        DataRange::Datatype(dt) => iri_to_curie_or_full(dt.0.as_ref(), namespaces),
        DataRange::DatatypeRestriction(dt, facets) => {
            let mut s = iri_to_curie_or_full(dt.0.as_ref(), namespaces);
            for f in facets {
                s.push_str(&format!("[{} {}]", facet_symbol(&f.f), literal_lexical(&f.l)));
            }
            s
        }
        DataRange::DataOneOf(lits) => {
            let inner = lits
                .iter()
                .map(|l| format!("\"{}\"", escape_turtle_string_local(literal_lexical(l))))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{inner}}}")
        }
        DataRange::DataComplementOf(inner) => {
            format!("not ({})", data_range_to_manchester(inner, namespaces))
        }
        DataRange::DataIntersectionOf(parts) => parts
            .iter()
            .map(|p| data_range_to_manchester(p, namespaces))
            .collect::<Vec<_>>()
            .join(" and "),
        DataRange::DataUnionOf(parts) => parts
            .iter()
            .map(|p| data_range_to_manchester(p, namespaces))
            .collect::<Vec<_>>()
            .join(" or "),
    }
}

fn facet_symbol(f: &Facet) -> &'static str {
    match f {
        Facet::MinInclusive => ">=",
        Facet::MaxInclusive => "<=",
        Facet::MinExclusive => ">",
        Facet::MaxExclusive => "<",
        Facet::Length => "length",
        Facet::MinLength => "minLength",
        Facet::MaxLength => "maxLength",
        Facet::Pattern => "pattern",
        Facet::TotalDigits => "totalDigits",
        Facet::FractionDigits => "fractionDigits",
        _ => "facet",
    }
}

fn iri_to_curie_or_full(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    for (prefix, base) in namespaces {
        if !prefix.is_empty() && iri.starts_with(base.as_str()) {
            return format!("{prefix}:{}", &iri[base.len()..]);
        }
    }
    format!("<{iri}>")
}

fn looks_like_datatype_iri(
    iri: &str,
    original: &str,
    known_datatypes: &std::collections::BTreeSet<String>,
) -> bool {
    iri.starts_with("http://www.w3.org/2001/XMLSchema#")
        || iri.starts_with("http://www.w3.org/1999/02/22-rdf-syntax-ns#")
            && (iri.ends_with("PlainLiteral")
                || iri.ends_with("langString")
                || iri.ends_with("HTML")
                || iri.ends_with("XMLLiteral"))
        || iri == "http://www.w3.org/2000/01/rdf-schema#Literal"
        || iri == "http://www.w3.org/2002/07/owl#real"
        || iri == "http://www.w3.org/2002/07/owl#rational"
        || original.starts_with("xsd:")
        || known_datatypes.contains(iri)
}

fn resolve_term_iri(term: &str, namespaces: &BTreeMap<String, String>) -> Result<String> {
    if term.starts_with("http://") || term.starts_with("https://") {
        return Ok(term.to_string());
    }
    if let Some(stripped) = term.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
        return Ok(stripped.to_string());
    }
    if let Some((prefix, local)) = term.split_once(':') {
        if prefix.is_empty() {
            // Default prefix `:Local` only when bound.
            if let Some(ns) = namespaces.get("") {
                return Ok(format!("{ns}{local}"));
            }
            return Err(OwlError::ManchesterInvalid(format!(
                "empty prefix in QName '{term}' (no default prefix declared)"
            )));
        }
        if let Some(ns) = namespaces.get(prefix) {
            return Ok(format!("{ns}{local}"));
        }
        // OWL Thing/Nothing are builtin defaults (e.g. unqualified cardinality fillers).
        if prefix == "owl" && (local == "Thing" || local == "Nothing") {
            return Ok(format!("http://www.w3.org/2002/07/owl#{local}"));
        }
        return Err(OwlError::ManchesterInvalid(format!("unknown prefix '{prefix}' in '{term}'")));
    }
    Err(OwlError::ManchesterInvalid(format!(
        "bare name '{term}' is not an IRI; use prefix:local or <absolute-iri>"
    )))
}

fn tokenize(input: &str) -> std::result::Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            '"' => {
                let mut lit = String::new();
                let mut closed = false;
                while let Some((_, c)) = chars.next() {
                    if c == '"' {
                        closed = true;
                        break;
                    }
                    if c == '\\' {
                        if let Some((_, escaped)) = chars.next() {
                            lit.push(escaped);
                        }
                    } else {
                        lit.push(c);
                    }
                }
                if !closed {
                    return Err(format!("unclosed string starting at {start}"));
                }
                tokens.push(Token::StringLit(lit));
            }
            '<' => {
                let mut iri = String::new();
                let mut closed = false;
                for (_, c) in chars.by_ref() {
                    if c == '>' {
                        closed = true;
                        break;
                    }
                    iri.push(c);
                }
                if !closed {
                    return Err(format!("unclosed IRI starting at {start}"));
                }
                tokens.push(Token::Iri(iri));
            }
            '0'..='9' => {
                let mut num = ch.to_string();
                while chars.peek().is_some_and(|(_, c)| c.is_ascii_digit()) {
                    num.push(chars.next().unwrap().1);
                }
                let n: u32 = num.parse().map_err(|_| format!("invalid number at {start}"))?;
                tokens.push(Token::Number(n));
            }
            _ if ch.is_alphabetic() || ch == '_' || ch == ':' => {
                let mut ident = ch.to_string();
                while chars.peek().is_some_and(|(_, c)| {
                    c.is_alphanumeric() || *c == '_' || *c == ':' || *c == '-'
                }) {
                    ident.push(chars.next().unwrap().1);
                }
                let lower = ident.to_ascii_lowercase();
                let kw = match lower.as_str() {
                    "and" => Some(Keyword::And),
                    "or" => Some(Keyword::Or),
                    "some" => Some(Keyword::Some),
                    "only" => Some(Keyword::Only),
                    "min" => Some(Keyword::Min),
                    "max" => Some(Keyword::Max),
                    "exactly" => Some(Keyword::Exactly),
                    "not" => Some(Keyword::Not),
                    "value" => Some(Keyword::Value),
                    "self" => Some(Keyword::SelfKw),
                    _ => None,
                };
                if let Some(k) = kw {
                    tokens.push(Token::Keyword(k));
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            _ => return Err(format!("unexpected character '{ch}' at {start}")),
        }
    }
    tokens.push(Token::Eof);
    Ok(tokens)
}

#[derive(Debug, Clone)]
enum Token {
    Ident(String),
    Iri(String),
    StringLit(String),
    Keyword(Keyword),
    Number(u32),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eof,
}

#[derive(Debug, Clone, Copy)]
enum Keyword {
    And,
    Or,
    Some,
    Only,
    Min,
    Max,
    Exactly,
    Not,
    Value,
    SelfKw,
}

struct ManchesterParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl ManchesterParser {
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if !matches!(tok, Token::Eof) {
            self.pos += 1;
        }
        tok
    }

    fn parse_expression(&mut self) -> std::result::Result<ManchesterAst, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> std::result::Result<ManchesterAst, String> {
        let mut parts = vec![self.parse_and()?];
        while matches!(self.peek(), Token::Keyword(Keyword::Or)) {
            self.advance();
            parts.push(self.parse_and()?);
        }
        if parts.len() == 1 {
            Ok(parts.into_iter().next().unwrap())
        } else {
            Ok(ManchesterAst::Or(parts))
        }
    }

    fn parse_and(&mut self) -> std::result::Result<ManchesterAst, String> {
        let mut parts = vec![self.parse_unary()?];
        while matches!(self.peek(), Token::Keyword(Keyword::And)) {
            self.advance();
            parts.push(self.parse_unary()?);
        }
        if parts.len() == 1 {
            Ok(parts.into_iter().next().unwrap())
        } else {
            Ok(ManchesterAst::And(parts))
        }
    }

    fn parse_unary(&mut self) -> std::result::Result<ManchesterAst, String> {
        if matches!(self.peek(), Token::Keyword(Keyword::Not)) {
            self.advance();
            let inner = self.parse_unary()?;
            return Ok(ManchesterAst::Not(Box::new(inner)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> std::result::Result<ManchesterAst, String> {
        if matches!(self.peek(), Token::Keyword(Keyword::Min | Keyword::Max | Keyword::Exactly)) {
            return self.parse_cardinality();
        }
        if matches!(self.peek(), Token::LBrace) {
            return self.parse_one_of();
        }
        if matches!(self.peek(), Token::LParen) {
            self.advance();
            let inner = self.parse_expression()?;
            self.expect_paren_r()?;
            return Ok(inner);
        }
        let name = self.parse_name()?;
        if matches!(self.peek(), Token::Keyword(Keyword::SelfKw)) {
            self.advance();
            return Ok(ManchesterAst::HasSelf { property: name });
        }
        if matches!(self.peek(), Token::Keyword(Keyword::Value)) {
            self.advance();
            return match self.peek().clone() {
                Token::StringLit(lit) => {
                    self.advance();
                    Ok(ManchesterAst::DataHasValue { property: name, literal: lit })
                }
                _ => {
                    let individual = self.parse_name()?;
                    Ok(ManchesterAst::HasValue { property: name, individual })
                }
            };
        }
        if matches!(self.peek(), Token::Keyword(Keyword::Min | Keyword::Max | Keyword::Exactly)) {
            let kind = self.advance();
            let Token::Number(n) = self.advance() else {
                return Err("expected cardinality number".to_string());
            };
            let filler = if Self::starts_class_term(self.peek()) {
                self.parse_primary()?
            } else {
                // Absolute IRI so unqualified cardinality works without a declared owl: prefix.
                ManchesterAst::Class("http://www.w3.org/2002/07/owl#Thing".to_string())
            };
            return Ok(match kind {
                Token::Keyword(Keyword::Min) => {
                    ManchesterAst::Min { n, property: name, filler: Box::new(filler) }
                }
                Token::Keyword(Keyword::Max) => {
                    ManchesterAst::Max { n, property: name, filler: Box::new(filler) }
                }
                Token::Keyword(Keyword::Exactly) => {
                    ManchesterAst::Exactly { n, property: name, filler: Box::new(filler) }
                }
                _ => return Err("expected min, max, or exactly".to_string()),
            });
        }
        if matches!(self.peek(), Token::Keyword(Keyword::Some | Keyword::Only)) {
            let quant = self.advance();
            let filler = self.parse_primary()?;
            return match quant {
                Token::Keyword(Keyword::Some) => {
                    Ok(ManchesterAst::Some { property: name, filler: Box::new(filler) })
                }
                Token::Keyword(Keyword::Only) => {
                    Ok(ManchesterAst::Only { property: name, filler: Box::new(filler) })
                }
                _ => return Err("internal parser error after some/only keyword".to_string()),
            };
        }
        Ok(ManchesterAst::Class(name))
    }

    fn parse_one_of(&mut self) -> std::result::Result<ManchesterAst, String> {
        if !matches!(self.advance(), Token::LBrace) {
            return Err("expected '{'".to_string());
        }
        let mut inds = Vec::new();
        while !matches!(self.peek(), Token::RBrace | Token::Eof) {
            inds.push(self.parse_name()?);
        }
        if !matches!(self.advance(), Token::RBrace) {
            return Err("expected '}'".to_string());
        }
        if inds.is_empty() {
            return Err("ObjectOneOf must contain at least one individual".to_string());
        }
        Ok(ManchesterAst::OneOf(inds))
    }

    fn parse_cardinality(&mut self) -> std::result::Result<ManchesterAst, String> {
        let kind = self.advance();
        let Token::Number(n) = self.advance() else {
            return Err("expected cardinality number".to_string());
        };
        let prop = self.parse_name()?;
        let filler = if matches!(self.peek(), Token::Keyword(Keyword::Some)) {
            self.advance();
            self.parse_primary()?
        } else if Self::starts_class_term(self.peek()) {
            self.parse_primary()?
        } else {
            // Absolute IRI so unqualified cardinality works without a declared owl: prefix.
            ManchesterAst::Class("http://www.w3.org/2002/07/owl#Thing".to_string())
        };
        Ok(match kind {
            Token::Keyword(Keyword::Min) => {
                ManchesterAst::Min { n, property: prop, filler: Box::new(filler) }
            }
            Token::Keyword(Keyword::Max) => {
                ManchesterAst::Max { n, property: prop, filler: Box::new(filler) }
            }
            Token::Keyword(Keyword::Exactly) => {
                ManchesterAst::Exactly { n, property: prop, filler: Box::new(filler) }
            }
            _ => return Err("expected min, max, or exactly".to_string()),
        })
    }

    fn starts_class_term(tok: &Token) -> bool {
        matches!(tok, Token::Ident(_) | Token::Iri(_) | Token::LParen | Token::LBrace)
    }

    fn parse_name(&mut self) -> std::result::Result<String, String> {
        match self.advance() {
            Token::Ident(s) => Ok(s),
            Token::Iri(iri) => Ok(format!("<{iri}>")),
            other => Err(format!("expected name, got {other:?}")),
        }
    }

    fn expect_paren_r(&mut self) -> std::result::Result<(), String> {
        if !matches!(self.advance(), Token::RParen) {
            return Err("expected ')'".to_string());
        }
        Ok(())
    }
}

pub fn class_expression_to_turtle_value(
    expr: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
    indent: usize,
) -> Result<String> {
    let pad = "    ".repeat(indent);
    let inner_pad = "    ".repeat(indent + 1);
    match expr {
        ClassExpression::Class(c) => iri_to_turtle_term(&c.to_string(), namespaces),
        ClassExpression::ObjectIntersectionOf(v) if v.len() == 1 => {
            class_expression_to_turtle_value(&v[0], namespaces, indent)
        }
        ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
            let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces)?;
            let filler = class_expression_to_turtle_value(bce, namespaces, indent + 1)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:someValuesFrom {filler}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectAllValuesFrom { ope, bce } => {
            let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces)?;
            let filler = class_expression_to_turtle_value(bce, namespaces, indent + 1)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:allValuesFrom {filler}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectMinCardinality { n, ope, bce } => {
            cardinality_turtle("owl:minQualifiedCardinality", *n, ope, bce, namespaces, indent)
        }
        ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
            cardinality_turtle("owl:maxQualifiedCardinality", *n, ope, bce, namespaces, indent)
        }
        ClassExpression::ObjectExactCardinality { n, ope, bce } => {
            cardinality_turtle("owl:qualifiedCardinality", *n, ope, bce, namespaces, indent)
        }
        ClassExpression::ObjectIntersectionOf(v) if v.len() > 1 => {
            let terms: Vec<String> = v
                .iter()
                .map(|e| class_expression_to_turtle_value(e, namespaces, indent + 2))
                .collect::<Result<_>>()?;
            let list = terms.join(" ");
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Class ;").ok();
            writeln!(out, "{inner_pad}owl:intersectionOf ( {list} )").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectUnionOf(v) if v.len() > 1 => {
            let terms: Vec<String> = v
                .iter()
                .map(|e| class_expression_to_turtle_value(e, namespaces, indent + 2))
                .collect::<Result<_>>()?;
            let list = terms.join(" ");
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Class ;").ok();
            writeln!(out, "{inner_pad}owl:unionOf ( {list} )").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectIntersectionOf(v) => {
            let Some(first) = v.first() else {
                return Err(OwlError::ManchesterInvalid(
                    "empty intersection expression".to_string(),
                ));
            };
            class_expression_to_turtle_value(first, namespaces, indent)
        }
        ClassExpression::ObjectUnionOf(v) => {
            let Some(first) = v.first() else {
                return Err(OwlError::ManchesterInvalid("empty union expression".to_string()));
            };
            class_expression_to_turtle_value(first, namespaces, indent)
        }
        ClassExpression::ObjectComplementOf(inner) => {
            let ce = class_expression_to_turtle_value(inner, namespaces, indent + 1)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Class ;").ok();
            writeln!(out, "{inner_pad}owl:complementOf {ce}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectHasValue { ope, i } => {
            let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces)?;
            let ind = iri_to_turtle_term(i, namespaces)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:hasValue {ind}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectHasSelf(ope) => {
            let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:hasSelf true").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::ObjectOneOf(inds) => {
            let terms: Result<Vec<_>> =
                inds.iter().map(|i| iri_to_turtle_term(i, namespaces)).collect();
            let list = terms?.join(" ");
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Class ;").ok();
            writeln!(out, "{inner_pad}owl:oneOf ( {list} )").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::DataSomeValuesFrom { dp, dr } => {
            let prop = iri_to_turtle_term(dp.0.as_ref(), namespaces)?;
            let range = data_range_to_turtle(dr, namespaces)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:someValuesFrom {range}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::DataAllValuesFrom { dp, dr } => {
            let prop = iri_to_turtle_term(dp.0.as_ref(), namespaces)?;
            let range = data_range_to_turtle(dr, namespaces)?;
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:allValuesFrom {range}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::DataHasValue { dp, l } => {
            let prop = iri_to_turtle_term(dp.0.as_ref(), namespaces)?;
            let lit = format!("\"{}\"", escape_turtle_string_local(literal_lexical(l)));
            let mut out = String::new();
            writeln!(out, "[").ok();
            writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
            writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
            writeln!(out, "{inner_pad}owl:hasValue {lit}").ok();
            write!(out, "{pad}]").ok();
            Ok(out)
        }
        ClassExpression::DataMinCardinality { n, dp, dr } => data_cardinality_turtle(
            "owl:minQualifiedCardinality",
            *n,
            dp.0.as_ref(),
            dr,
            namespaces,
            indent,
        ),
        ClassExpression::DataMaxCardinality { n, dp, dr } => data_cardinality_turtle(
            "owl:maxQualifiedCardinality",
            *n,
            dp.0.as_ref(),
            dr,
            namespaces,
            indent,
        ),
        ClassExpression::DataExactCardinality { n, dp, dr } => data_cardinality_turtle(
            "owl:qualifiedCardinality",
            *n,
            dp.0.as_ref(),
            dr,
            namespaces,
            indent,
        ),
    }
}

fn cardinality_turtle(
    pred: &str,
    n: u32,
    ope: &ObjectPropertyExpression<RcStr>,
    bce: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
    indent: usize,
) -> Result<String> {
    let pad = "    ".repeat(indent);
    let inner_pad = "    ".repeat(indent + 1);
    let prop = iri_to_turtle_term(&ope_to_iri(ope), namespaces)?;
    let filler = class_expression_to_turtle_value(bce, namespaces, indent + 1)?;
    let mut out = String::new();
    writeln!(out, "[").ok();
    writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
    writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
    writeln!(
        out,
        "{inner_pad}{pred} \"{n}\"^^<http://www.w3.org/2001/XMLSchema#nonNegativeInteger> ;"
    )
    .ok();
    writeln!(out, "{inner_pad}owl:onClass {filler}").ok();
    write!(out, "{pad}]").ok();
    Ok(out)
}

fn data_cardinality_turtle(
    pred: &str,
    n: u32,
    property_iri: &str,
    dr: &DataRange<RcStr>,
    namespaces: &BTreeMap<String, String>,
    indent: usize,
) -> Result<String> {
    let pad = "    ".repeat(indent);
    let inner_pad = "    ".repeat(indent + 1);
    let prop = iri_to_turtle_term(property_iri, namespaces)?;
    let range = data_range_to_turtle(dr, namespaces)?;
    let mut out = String::new();
    writeln!(out, "[").ok();
    writeln!(out, "{inner_pad}a owl:Restriction ;").ok();
    writeln!(out, "{inner_pad}owl:onProperty {prop} ;").ok();
    writeln!(
        out,
        "{inner_pad}{pred} \"{n}\"^^<http://www.w3.org/2001/XMLSchema#nonNegativeInteger> ;"
    )
    .ok();
    writeln!(out, "{inner_pad}owl:onDataRange {range}").ok();
    write!(out, "{pad}]").ok();
    Ok(out)
}

fn data_range_to_turtle(
    dr: &DataRange<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    match dr {
        DataRange::Datatype(dt) => iri_to_turtle_term(dt.0.as_ref(), namespaces),
        DataRange::DatatypeRestriction(dt, facets) => {
            let on = iri_to_turtle_term(dt.0.as_ref(), namespaces)?;
            let mut restrictions = String::new();
            for f in facets {
                let facet_iri = f.f.as_ref();
                let facet_term = iri_to_turtle_term(facet_iri, namespaces)?;
                let lit = format!("\"{}\"", escape_turtle_string_local(literal_lexical(&f.l)));
                write!(restrictions, " [ {facet_term} {lit} ]").ok();
            }
            Ok(format!(
                "[ a rdfs:Datatype ; owl:onDatatype {on} ; owl:withRestrictions ({restrictions} ) ]"
            ))
        }
        DataRange::DataOneOf(lits) => {
            let members = lits
                .iter()
                .map(|l| format!("\"{}\"", escape_turtle_string_local(literal_lexical(l))))
                .collect::<Vec<_>>()
                .join(" ");
            Ok(format!("[ a rdfs:Datatype ; owl:oneOf ( {members} ) ]"))
        }
        DataRange::DataComplementOf(inner) => {
            let inner_t = data_range_to_turtle(inner, namespaces)?;
            Ok(format!("[ a rdfs:Datatype ; owl:datatypeComplementOf {inner_t} ]"))
        }
        DataRange::DataIntersectionOf(parts) => {
            let mut members = String::new();
            for p in parts {
                let t = data_range_to_turtle(p, namespaces)?;
                write!(members, " {t}").ok();
            }
            Ok(format!("[ a rdfs:Datatype ; owl:intersectionOf ({members} ) ]"))
        }
        DataRange::DataUnionOf(parts) => {
            let mut members = String::new();
            for p in parts {
                let t = data_range_to_turtle(p, namespaces)?;
                write!(members, " {t}").ok();
            }
            Ok(format!("[ a rdfs:Datatype ; owl:unionOf ({members} ) ]"))
        }
    }
}

/// Public Turtle emitter for data ranges (DatatypeDefinition write-back).
pub fn data_range_to_turtle_term(
    dr: &DataRange<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> Result<String> {
    data_range_to_turtle(dr, namespaces)
}

fn data_cardinality_manchester(
    prop: &str,
    keyword: &str,
    n: u32,
    dr: &DataRange<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> String {
    format!(
        "{} {keyword} {n} {}",
        iri_to_manchester_term(prop, namespaces),
        data_range_to_manchester(dr, namespaces)
    )
}

fn literal_lexical(l: &Literal<RcStr>) -> &str {
    match l {
        Literal::Simple { literal } => literal.as_str(),
        Literal::Language { literal, .. } => literal.as_str(),
        Literal::Datatype { literal, .. } => literal.as_str(),
    }
}

fn escape_turtle_string_local(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn is_owl_thing(expr: &ClassExpression<RcStr>) -> bool {
    matches!(
        expr,
        ClassExpression::Class(c) if c.as_ref() == "http://www.w3.org/2002/07/owl#Thing"
    )
}

fn cardinality_manchester(
    prop: &str,
    keyword: &str,
    n: u32,
    bce: &ClassExpression<RcStr>,
    namespaces: &BTreeMap<String, String>,
) -> String {
    if is_owl_thing(bce) {
        format!("{prop} {keyword} {n}")
    } else {
        let filler = class_expression_to_manchester(bce, namespaces);
        format!("{prop} {keyword} {n} {filler}")
    }
}

fn ope_to_iri(ope: &ObjectPropertyExpression<RcStr>) -> String {
    match ope {
        ObjectPropertyExpression::ObjectProperty(p) => p.0.as_ref().to_string(),
        ObjectPropertyExpression::InverseObjectProperty(p) => {
            format!("inverse {}", p.0.as_ref())
        }
    }
}

fn iri_to_manchester_term(iri: &str, namespaces: &BTreeMap<String, String>) -> String {
    if !crate::patch::is_safe_iri(iri) {
        // Display-only path: never emit angle brackets for unsafe IRIs.
        return iri.chars().filter(|c| !c.is_control() && !c.is_whitespace()).collect();
    }
    if let Some((prefix, ns)) = crate::patch::best_namespace_match(iri, namespaces) {
        let local = &iri[ns.len()..];
        return format!("{prefix}:{local}");
    }
    if iri.starts_with("http://") || iri.starts_with("https://") {
        format!("<{iri}>")
    } else {
        iri.to_string()
    }
}

fn iri_to_turtle_term(iri: &str, namespaces: &BTreeMap<String, String>) -> Result<String> {
    crate::patch::iri_to_turtle_term_impl(iri, namespaces)
        .map_err(|e| OwlError::ManchesterInvalid(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/people#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
            ("rdfs".to_string(), "http://www.w3.org/2000/01/rdf-schema#".to_string()),
        ])
    }

    fn clinic_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/clinic#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ])
    }

    fn anatomy_ns() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("ex".to_string(), "http://example.org/anatomy#".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ])
    }

    #[test]
    fn parse_property_first_min_cardinality() {
        let ns = anatomy_ns();
        let out = parse_class_expression("ex:hasPart min 1 ex:Organ", &ns).expect("parse");
        assert!(out.normalized.contains("ex:hasPart min 1 ex:Organ"));
    }

    #[test]
    fn parse_property_first_without_filler() {
        let ns = anatomy_ns();
        let out = parse_class_expression("ex:hasPart min 1", &ns).expect("parse");
        assert_eq!(out.normalized, "ex:hasPart min 1");
        match &out.expression {
            ClassExpression::ObjectMinCardinality { bce, .. } => {
                assert!(is_owl_thing(bce));
            }
            other => panic!("expected min cardinality, got {other:?}"),
        }
    }

    #[test]
    fn unqualified_cardinality_without_owl_prefix() {
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org/".to_string())]);
        let out = parse_class_expression("ex:p min 1", &ns).expect("parse without owl prefix");
        match &out.expression {
            ClassExpression::ObjectMinCardinality { bce, .. } => {
                assert!(is_owl_thing(bce), "default filler must be owl:Thing");
            }
            other => panic!("expected min cardinality, got {other:?}"),
        }
    }

    #[test]
    fn parse_keyword_first_cardinality_without_some() {
        let ns = anatomy_ns();
        let out = parse_class_expression("min 1 ex:hasPart ex:Organ", &ns).expect("parse");
        assert!(out.normalized.contains("ex:hasPart min 1 ex:Organ"));
    }

    #[test]
    fn cardinality_turtle_emits_typed_literal() {
        let ns = anatomy_ns();
        for input in [
            "ex:hasPart min 1 ex:Organ",
            "ex:hasPart max 2 ex:Organ",
            "ex:hasPart exactly 3 ex:Organ",
        ] {
            let out = parse_class_expression(input, &ns).expect("parse");
            let turtle = class_expression_to_turtle_value(&out.expression, &ns, 0).expect("turtle");
            assert!(
                turtle.contains("^^<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"),
                "expected typed cardinality literal in: {turtle}"
            );
            assert!(!turtle.contains("minQualifiedCardinality 1 ;"));
            assert!(!turtle.contains("maxQualifiedCardinality 2 ;"));
            assert!(!turtle.contains("qualifiedCardinality 3 ;"));
        }
    }

    #[test]
    fn cardinality_round_trip_property_first() {
        let ns = anatomy_ns();
        let out = parse_class_expression("ex:hasPart min 1 ex:Organ", &ns).expect("parse");
        let turtle = class_expression_to_turtle_fragment(&out.expression, "rdfs:subClassOf", &ns)
            .expect("turtle");
        assert!(turtle.contains("owl:Restriction"));
        assert!(turtle.contains("owl:minQualifiedCardinality"));
        assert!(turtle.contains("owl:onClass"));
        assert!(turtle.contains("^^<http://www.w3.org/2001/XMLSchema#nonNegativeInteger>"));
    }

    #[test]
    fn parse_some_values_from() {
        let ns = clinic_ns();
        let out = parse_class_expression("ex:hasRecord some ex:MedicalRecord", &ns).expect("parse");
        assert!(out.normalized.contains("some"));
    }

    #[test]
    fn parse_and_expression() {
        let ns = ex_ns();
        let out = parse_class_expression("ex:Person and ex:Organization", &ns).expect("parse");
        assert!(out.normalized.contains("and"));
    }

    #[test]
    fn turtle_fragment_for_restriction() {
        let ns = clinic_ns();
        let out = parse_class_expression("ex:hasRecord some ex:MedicalRecord", &ns).expect("parse");
        let turtle = class_expression_to_turtle_fragment(&out.expression, "rdfs:subClassOf", &ns)
            .expect("turtle");
        assert!(turtle.contains("owl:Restriction"));
        assert!(turtle.contains("owl:someValuesFrom"));
    }

    #[test]
    fn turtle_intersection_includes_all_operands() {
        let ns = ex_ns();
        let out = parse_class_expression("ex:Person and ex:Organization", &ns).expect("parse");
        let turtle = class_expression_to_turtle_value(&out.expression, &ns, 0).expect("turtle");
        assert!(turtle.contains("ex:Person"));
        assert!(turtle.contains("ex:Organization"));
        assert!(turtle.contains("owl:intersectionOf"));
    }

    #[test]
    fn turtle_term_longest_namespace_prefix_wins() {
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/".to_string()),
            ("exfoo".to_string(), "http://example.org/foo/".to_string()),
            ("owl".to_string(), "http://www.w3.org/2002/07/owl#".to_string()),
        ]);
        let out = parse_class_expression("exfoo:Bar", &ns).expect("parse");
        let turtle = class_expression_to_turtle_fragment(&out.expression, "rdfs:subClassOf", &ns)
            .expect("turtle");
        assert!(turtle.contains("exfoo:Bar"));
        assert!(!turtle.contains("ex:foo/Bar"));
    }

    #[test]
    fn parse_not_complement() {
        let ns = ex_ns();
        let out = parse_class_expression("not ex:Person", &ns).expect("parse not");
        assert!(matches!(out.expression, ClassExpression::ObjectComplementOf(_)));
        let turtle = class_expression_to_turtle_value(&out.expression, &ns, 0).expect("turtle");
        assert!(turtle.contains("owl:complementOf"));
    }

    #[test]
    fn parse_has_value_and_has_self() {
        let ns = clinic_ns();
        let out = parse_class_expression("ex:hasRecord value ex:rec1", &ns).expect("has value");
        assert!(matches!(out.expression, ClassExpression::ObjectHasValue { .. }));
        let out = parse_class_expression("ex:likes Self", &ns).expect("has self");
        assert!(matches!(out.expression, ClassExpression::ObjectHasSelf(_)));
    }

    #[test]
    fn parse_one_of() {
        let ns = ex_ns();
        let out = parse_class_expression("{ ex:Person ex:Organization }", &ns).expect("one of");
        match out.expression {
            ClassExpression::ObjectOneOf(inds) => assert_eq!(inds.len(), 2),
            other => panic!("expected oneOf, got {other:?}"),
        }
    }

    #[test]
    fn parse_data_some_with_xsd() {
        let ns = BTreeMap::from([
            ("ex".to_string(), "http://example.org/".to_string()),
            ("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string()),
        ]);
        let out = parse_class_expression("ex:age some xsd:integer", &ns).expect("data some");
        assert!(matches!(out.expression, ClassExpression::DataSomeValuesFrom { .. }));
    }

    #[test]
    fn parse_data_some_with_known_custom_datatype() {
        // #335
        let ns = BTreeMap::from([("ex".to_string(), "http://example.org/".to_string())]);
        let known = std::collections::BTreeSet::from(["http://example.org/SSN".to_string()]);
        let out = parse_class_expression_with_datatypes("ex:hasSSN some ex:SSN", &ns, &known)
            .expect("custom datatype filler");
        assert!(matches!(out.expression, ClassExpression::DataSomeValuesFrom { .. }));
        let as_class = parse_class_expression("ex:hasSSN some ex:SSN", &ns).expect("no known");
        assert!(matches!(as_class.expression, ClassExpression::ObjectSomeValuesFrom { .. }));
    }

    #[test]
    fn parse_data_range_facets_and_one_of() {
        let ns =
            BTreeMap::from([("xsd".to_string(), "http://www.w3.org/2001/XMLSchema#".to_string())]);
        let restricted = parse_data_range("xsd:integer[>= 0][<= 10]", &ns).expect("facets");
        match restricted {
            DataRange::DatatypeRestriction(_, facets) => assert_eq!(facets.len(), 2),
            other => panic!("expected DatatypeRestriction, got {other:?}"),
        }
        let one_of = parse_data_range("{\"a\", \"b\"}", &ns).expect("oneOf");
        assert!(matches!(one_of, DataRange::DataOneOf(_)));
        let complement = parse_data_range("not xsd:string", &ns).expect("not");
        assert!(matches!(complement, DataRange::DataComplementOf(_)));
    }
}
