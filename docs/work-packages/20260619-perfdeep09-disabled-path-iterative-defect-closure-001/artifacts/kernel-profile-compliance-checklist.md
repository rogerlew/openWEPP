# PERFDEEP09 Kernel Profile Compliance Checklist

Status: complete.
Evidence class: Static + Ran.

| Requirement | Status | Evidence |
|---|---|---|
| No physics formula changes | PASS | retained patch changes only indexed-overflow guard traversal |
| No output schema/unit/metadata meaning changes | PASS | no output writer/schema files changed |
| Typed guards preserved | PASS | new `HS-DECOMP-E-008` regression passed |
| PERFDEEP opt-ins remain explicit and fail-closed | PASS | final commands unset all PERFDEEP env vars; no env default changed |
| R2+ direct runtime not implemented | PASS | no direct executor/frame schema/publication cutover files changed |
| Protected output identity preserved | PASS | HBP/loss/WAT/plot byte checks; PASS Arrow/DuckDB row equivalence |
| Gate Evidence Non-Deferral checked | PASS | final median, identity, full Rust gates, reviews, and verification recorded |
