const CODEX_CONFIG: &str = include_str!("../../.codex/config.toml");
const COMPARATOR_RUNNER: &str = include_str!("../../.codex/agents/comparator_suite_runner.toml");

fn contains_all(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| haystack.contains(needle))
}

#[test]
fn comparator_suite_runner_is_registered_and_uses_spark_model() {
    assert!(contains_all(
        CODEX_CONFIG,
        &[
            "[agents.comparator_suite_runner]",
            "config_file = \"agents/comparator_suite_runner.toml\"",
            "tools/owcmp",
            "compact metrics plus artifact paths",
        ],
    ));
    assert!(contains_all(
        COMPARATOR_RUNNER,
        &[
            "model = \"gpt-5.3-codex-spark\"",
            "Purpose: keep premium reasoning agents cheap",
            "Scope (execution only):",
            "Output contract (compact):",
        ],
    ));
}

#[test]
fn comparator_suite_runner_discovers_owcmp_not_retired_legacy_suite() {
    assert!(contains_all(
        COMPARATOR_RUNNER,
        &[
            "tools/owcmp/owcmp wat semantic ...",
            "tools/owcmp/owcmp pl14s run ...",
            "tools/owcmp/owcmp summarize --input <report.json> --output-root <dir>",
            "tools/owcmp/owcmp batch h1-h39-semantic",
            "summary.json",
            "summary.md",
            "command-log.json",
        ],
    ));
    assert!(
        !COMPARATOR_RUNNER.contains("tools/legacy_comparison_suite"),
        "active comparator runner config must not point agents at the retired legacy suite"
    );
}
