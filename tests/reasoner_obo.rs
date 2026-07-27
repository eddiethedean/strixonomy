mod support;

use std::path::PathBuf;
use strixonomy_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

fn obo_workspace() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let workspace = dir.path().to_path_buf();
    std::fs::write(
        workspace.join("test.obo"),
        "format-version: 1.2\nontology: test\n\n\
[Term]\n\
id: TEST:0000001\n\
name: child\n\
is_a: TEST:0000002 ! parent\n\n\
[Term]\n\
id: TEST:0000002\n\
name: parent\n",
    )
    .expect("write obo");
    (dir, workspace)
}

#[test]
fn el_classify_obo_workspace() {
    let (_dir, workspace) = obo_workspace();
    let input = WorkspaceInputLoader::new(&workspace).load().expect("load");
    let result = classify(ReasonerId::El, &input, false).expect("classify");

    assert_eq!(result.profile_used, "el");
    assert!(result.consistent);
}
