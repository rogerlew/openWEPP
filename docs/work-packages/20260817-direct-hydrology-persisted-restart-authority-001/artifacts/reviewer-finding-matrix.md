# Reviewer finding matrix

Status: `OPEN / executable remediation required`

| ID | Severity | Source location | Authority obligation | Selected correction | Required executable evidence | Invalidated gates | Disposition |
|---|---|---|---|---|---|---|---|
| RA-HYD-001 | blocker | `tools/restart_authority/generate_vectors.py`, direct-hydrology object | Every continuation field has an explicit DTO mapping | Replace invented/truncated dictionaries with one typed reference projector/restorer using exhaustive destructuring | Real runtime projection/restore equality and source-field coverage guard | hydrology review, authority release | OPEN |
| RA-HYD-002 | blocker | `artifacts/direct-run-frame-field-classification.md` | Exact units, domain, owner, order, reconstruction, validation, poison, omission consequence per field | Generate the ledger from typed mapping metadata, including nested continuation owners | Metadata completeness test plus reviewer inspection | hydrology review | OPEN |
| RA-SER-001 | blocker | former generic `serde_json::Value` authority path | Strict duplicate rejection and exact canonical bytes | One typed parser/serializer with primitive wrappers and exact reserialization comparison | duplicate/reorder/whitespace/escaping/case poisons | serialization review | OPEN |
| RA-SER-002 | blocker | inferred `checkpoint-schema.json` | Schema is derivative evidence, not authority | Generate schema and vectors from the typed reference implementation | deterministic regeneration and manifest equality | serialization review | OPEN |
| RA-SER-003 | blocker | nested `sha(label)` identities | Every digest is canonical-content-derived and cross-joined | Compute all nested digests bottom-up in the typed source | one-bit/recomputed-outer-digest poisons | serialization and GSI reviews | OPEN |
| RA-PHASE-001 | blocker | in-progress owner envelope | Exactly two owner postures with no duplicated GSI/cursor state | Separate committed scientific owners from explicit transactional GSI/cursor fields and validate equality joins | boundary/in-progress round trips and substitution poisons | all authority reviews | OPEN |
| RA-FORCING-001 | blocker | forcing vector projection | Persist actual full day/interval/parcel receipt shape | Typed fixed-width wire projection of runtime forcing receipts | two destinations × 48 intervals, carry and digest poisons | GSI/forcing review | OPEN |
| RA-POISON-001 | blocker | authority integration target | Every named typed category executes | Generate mutations from typed fixtures and assert exact category | full poison matrix plus live-byte no-mutation | all release gates | OPEN |
| RA-DTO-001 | blocker | package authority tools | Actual typed DTOs/projector/restorer exist outside production | Create package-local Rust reference crate/module | compile, projection, restoration, regeneration | terminal verification | OPEN |
| RA-REVIEW-001 | blocker | exact bytes after `bb3cc3a0e` | Fresh independent review and two terminal PASS verdicts | Rerun only after all above evidence passes | three reviews and dual verification on exact commit | authority release | OPEN |

