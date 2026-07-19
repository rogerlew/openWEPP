use serde_json::Value;

use crate::canonical::sha256_bytes;

pub(super) fn collect_testcases(value: &Value, prefix: &str, output: &mut Vec<String>) {
    match value {
        Value::Object(object) => collect_object_testcases(object, prefix, output),
        Value::Array(array) => collect_array_testcases(array, prefix, output),
        _ => {}
    }
}

fn collect_object_testcases(
    object: &serde_json::Map<String, Value>,
    prefix: &str,
    output: &mut Vec<String>,
) {
    if let Some(testcases) = object.get("testcases").and_then(Value::as_object) {
        output.extend(
            testcases
                .iter()
                .filter(|(_, testcase)| testcase["filter-match"]["status"] != "mismatch")
                .map(|(name, _)| sha256_bytes(format!("{prefix}\0{name}").as_bytes())),
        );
    }
    for (key, child) in object {
        let next = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}::{key}")
        };
        collect_testcases(child, &next, output);
    }
}

fn collect_array_testcases(array: &[Value], prefix: &str, output: &mut Vec<String>) {
    for child in array {
        collect_testcases(child, prefix, output);
    }
}

#[cfg(test)]
mod tests {
    use super::collect_testcases;
    use crate::canonical::sha256_bytes;

    #[test]
    fn excludes_explicit_filter_mismatches() {
        let listing = serde_json::json!({
            "rust-suites": {
                "suite": {
                    "testcases": {
                        "selected": {"filter-match": {"status": "matches"}},
                        "ignored": {
                            "ignored": true,
                            "filter-match": {"status": "mismatch", "reason": "ignored"}
                        },
                        "legacy-selected": {"kind": "test"}
                    }
                }
            }
        });
        let mut inventory = Vec::new();
        collect_testcases(&listing, "", &mut inventory);
        inventory.sort();
        assert_eq!(inventory.len(), 2);
        assert!(inventory.contains(&sha256_bytes(b"rust-suites::suite\0selected")));
        assert!(inventory.contains(&sha256_bytes(b"rust-suites::suite\0legacy-selected")));
    }
}
