# Reviewer finding matrix

Status: `COMPLETE / ALL EXACT-COMMIT REVIEWS AND VERIFICATIONS PASS`

| ID | Severity | Source location | Authority obligation | Selected correction | Required executable evidence | Invalidated gates | Disposition |
|---|---|---|---|---|---|---|---|
| RA-HYD-001 | blocker | `tools/restart_authority/generate_vectors.py`, direct-hydrology object | Every continuation field has an explicit DTO mapping | Replace invented/truncated dictionaries with one typed reference projector/restorer using exhaustive destructuring | Real runtime projection/restore equality and source-field coverage guard | hydrology review, authority release | PASS |
| RA-HYD-002 | blocker | `artifacts/direct-run-frame-field-classification.md` | Exact units, domain, owner, order, reconstruction, validation, poison, omission consequence per field | Generate the ledger from typed mapping metadata, including nested continuation owners | Metadata completeness test plus reviewer inspection | hydrology review | PASS |
| RA-SER-001 | blocker | former generic `serde_json::Value` authority path | Strict duplicate rejection and exact canonical bytes | One typed parser/serializer with primitive wrappers and exact reserialization comparison | duplicate/reorder/whitespace/escaping/case poisons | serialization review | PASS |
| RA-SER-002 | blocker | inferred `checkpoint-schema.json` | Schema is derivative evidence, not authority | Generate schema and vectors from the typed reference implementation | deterministic regeneration and manifest equality | serialization review | PASS |
| RA-SER-003 | blocker | nested `sha(label)` identities | Every digest is canonical-content-derived and cross-joined | Compute all nested digests bottom-up in the typed source | one-bit/recomputed-outer-digest poisons | serialization and GSI reviews | PASS |
| RA-PHASE-001 | blocker | in-progress owner envelope | Exactly two owner postures with no duplicated GSI/cursor state | Separate committed scientific owners from explicit transactional GSI/cursor fields and validate equality joins | boundary/in-progress round trips and substitution poisons | all authority reviews | PASS |
| RA-FORCING-001 | blocker | forcing vector projection | Persist actual full day/interval/parcel receipt shape | Typed fixed-width wire projection of runtime forcing receipts | two destinations × 48 intervals, carry and digest poisons | GSI/forcing review | PASS |
| RA-POISON-001 | blocker | authority integration target | Every named typed category executes | Generate mutations from typed fixtures and assert exact category | full poison matrix plus live-byte no-mutation | all release gates | PASS |
| RA-DTO-001 | blocker | package authority tools | Actual typed DTOs/projector/restorer exist outside production | Create package-local Rust reference crate/module | compile, projection, restoration, regeneration | terminal verification | PASS |
| RA-REVIEW-001 | blocker | exact bytes after `bb3cc3a0e` | Fresh independent review and two terminal PASS verdicts | Rerun only after all above evidence passes | three reviews and dual verification on exact commit | authority release | PASS |
| RA-OWNER-001 | blocker | `checkpoint.rs` generic envelope | Scientific payload is actual typed owner state, never descriptive strings or blobs | Delete generic owner types and define typed scientific/committed sets | complete owner round trips and domain/join/omission poisons | all reviews, release | PASS |
| RA-LINEAGE-001 | blocker | `OwnerSetV1::validate` | Daily GSI, daily provider, immutable configuration, and interval science use distinct lineage domains | Encode domain-specific joins in the phase DTOs | substitution, rewind/skip, and interval transaction poisons | state and ownership reviews | PASS |
| RA-HYD-003 | blocker | checkpoint top-level hydrology | Committed and staged scientific sets each contain direct hydrology | Remove top-level hydrology and persist it in both phase owner postures | interval-24 continuation and exact abort equality | hydrology/state review | PASS |
| RA-SURFACE-001 | blocker | hydrology restore | Surface-liquid state restoration is configuration-bound | Restore through expected configuration and invoke owner validation/digest comparison | configuration, record/OFE/layer/lineage poisons | hydrology review | PASS |
| RA-CONT-001 | blocker | synthetic `advance()` fixture | Continuation executes real V10/LSE/hydrology/surface/soil/BGC owners | Replace byte arithmetic with repository-backed interval execution | fresh-object interval 24→47 equivalence and real late-failure abort | all reviews, release | PASS |
| RA-SCHEMA-001 | blocker | descriptor schema | Complete derivative JSON Schema covers every nested DTO and bound | Generate schema from typed hierarchy and test drift/cardinality | schema validation plus deterministic regeneration | serialization review | PASS |

## Exact-current disposition

All rows are PASS. The historical failed reviews remain preserved in the gate
record and commit history. Three authority reviewers and two terminal verifiers
reported PASS with no material findings on exact commit
`684477022b1a801a405c0ddd23c6166673339e75` after the complete executable gate
suite passed.
