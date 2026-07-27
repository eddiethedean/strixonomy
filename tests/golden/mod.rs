use std::fs;
use std::path::Path;
use strixonomy_catalog::IndexBuilder;
use strixonomy_query::query_catalog;

pub fn assert_golden_snapshot(workspace: &str, query: &str, golden_path: &Path) {
    let catalog = IndexBuilder::new().workspace(workspace).build().expect("index workspace");

    let result = query_catalog(&catalog, query).expect("run query");
    let mut rows = result.rows.clone();
    if let Some(sort_col) = result.columns.first() {
        rows.sort_by(|a, b| a.get(sort_col).cmp(&b.get(sort_col)));
    }
    let mut lines = vec![result.columns.join("\t")];
    for row in &rows {
        let values: Vec<String> =
            result.columns.iter().map(|c| row.get(c).cloned().unwrap_or_default()).collect();
        lines.push(values.join("\t"));
    }
    let actual = format!("{}\n", lines.join("\n"));

    if std::env::var("ONTOINDEX_UPDATE_GOLDEN").is_ok() {
        if let Some(parent) = golden_path.parent() {
            fs::create_dir_all(parent).expect("create golden dir");
        }
        fs::write(golden_path, &actual).expect("write golden");
        return;
    }

    let expected = fs::read_to_string(golden_path)
        .unwrap_or_else(|_| panic!("missing golden file: {}", golden_path.display()));
    assert_eq!(expected, actual, "golden snapshot mismatch for {}", query);
}
