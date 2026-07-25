# Line-Count Governance

Evidence class: Ran / Static.

| File | Lines | Disposition |
|---|---:|---|
| `tools/local_ci/quality_observatory.py` | 2,286 | WARN |
| `tests/integration/quality_observatory_merged_coverage_contract.rs` | 122 | PASS |

The Python collector remains below the 3,000-line blocking ceiling. The
follow-on adds one narrow Git-metadata helper and behavioral self-test.
Splitting admission/collection/publication in the same correction would mix a
structural refactor with a proven one-path compatibility defect.

Split intent: before the next material collector extension, extract admission
and execution-snapshot identity into a dedicated module with unchanged CLI and
schema contracts. No exemption from the blocking ceiling is claimed.
