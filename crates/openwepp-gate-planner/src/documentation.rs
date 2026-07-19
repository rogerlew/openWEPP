use std::collections::BTreeSet;
use std::path::Path;

use crate::repository::ObservedChange;

pub(crate) fn changed_markdown_paths(changes: &[ObservedChange]) -> Vec<String> {
    changes
        .iter()
        .filter(|change| change.change_kind != "DELETE" && is_markdown_file(&change.path))
        .map(|change| change.path.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn append_lint_paths(arguments: &mut Vec<String>, definition: &str, paths: &[String]) {
    if definition == "documentation-lint-v1" {
        for path in paths {
            arguments.extend(["--path".to_owned(), path.clone()]);
        }
    }
}

fn is_markdown_file(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
}

#[cfg(test)]
mod tests {
    use super::{append_lint_paths, changed_markdown_paths};
    use crate::repository::ObservedChange;

    fn change(path: &str, kind: &str) -> ObservedChange {
        ObservedChange {
            path: path.to_owned(),
            change_kind: kind.to_owned(),
            object_kind: "REGULAR".to_owned(),
            old_mode: None,
            new_mode: None,
        }
    }

    #[test]
    fn changed_markdown_is_strict_sorted_and_deduplicated() {
        let changes = [
            change("docs/z.md", "MODIFY"),
            change("docs/schema.json", "MODIFY"),
            change("docs/deleted.md", "DELETE"),
            change("README.MD", "ADD"),
            change("docs/z.md", "MODIFY"),
            change("docs/image.png", "ADD"),
        ];
        assert_eq!(changed_markdown_paths(&changes), ["README.MD", "docs/z.md"]);
        let mut arguments = vec!["markdown-doc".to_owned(), "lint".to_owned()];
        append_lint_paths(
            &mut arguments,
            "documentation-lint-v1",
            &["README.MD".to_owned(), "docs/z.md".to_owned()],
        );
        assert_eq!(
            arguments,
            [
                "markdown-doc",
                "lint",
                "--path",
                "README.MD",
                "--path",
                "docs/z.md"
            ]
        );
    }
}
