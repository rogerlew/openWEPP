# Gate Results

Status: executed-hold.
Evidence mode: Static + Ran.

| Gate | Status | Evidence |
|---|---|---|
| R6 prerequisite hold evidence | PASS | R6 executed-hold exists at `HOLD-R6-DIRECT-PUBLICATION-PARITY-AND-MANIFEST-CUTOVER`. |
| R6A direct frame prerequisite | PASS | R6A completed `DirectRunPublicationFrame` and direct projection consumers. |
| Operand lineage table | FAIL | Current field lineage is recorded, but accepted authoritative direct producers are absent at the cutover boundary. |
| Contract-first gate | PASS | No output meaning, metadata, schema, provenance, guard, or physics semantics changed. |
| Parity-grade direct frame population | FAIL | Production cutover still creates `DirectRunFrame::skeleton` and captures zero/default direct state. |
| Anti-alias fixtures | BLOCKED | No nonzero parity-grade typed operands exist to distinguish from rejected aliases. |
| Independent reconstruction | BLOCKED | No accepted direct publication operands exist to reconstruct. |
| Manifest direct provenance cutover | BLOCKED | Production manifest still uses compatibility provenance; direct frame is not authoritative. |
| HBP byte identity | FAIL | CLI/focused tests report `R6-DIRECT-PUBLICATION-PARITY R6B-DIRECT-PUBLICATION-TYPED-OPERANDS-ABSENT`, direct `1654` bytes vs compatibility `1654` bytes. |
| WAT Arrow/metadata parity | BLOCKED | Not accepted because item 1 and HBP identity failed first. |
| PASS Arrow/metadata parity | BLOCKED | Not accepted because item 1 and HBP identity failed first. |
| Loss JSON parity | BLOCKED | Not accepted because item 1 and HBP identity failed first. |
| Manifest parity | BLOCKED | Not wired to production direct projection. |
| No-compatibility proof | BLOCKED | Direct consumers are helper-only until direct frame population is authoritative; manifest still compatibility-fed. |
| Default-disabled H2637 gate | BLOCKED | Not run because no valid direct-publication cutover endpoint exists in this package. |
| Endpoint/RSS benchmark | BLOCKED | Not run because no valid direct-publication cutover endpoint exists in this package. |
| Focused Rust gates | PASS | `cargo fmt --check`, `cargo test -p openwepp-runner r6_ -- --nocapture`, the focused CLI contract, and `cargo clippy -p openwepp-runner --all-targets -- -D warnings` passed. |
| Workspace Rust static gates | PASS | `cargo clippy --workspace --all-targets -- -D warnings` and `cargo deny check` passed. |
| Full workspace tests | PASS | Fresh `cargo test --workspace` passed after the R6B negative-test edit. |
| Scoped markdown lint | PASS | Scoped `markdown-doc lint` passed with 28 files scanned, 0 errors, 0 warnings after final artifact edits. |
| `git diff --check` | PASS | Passed after final Rust and artifact edits. |
| Dual review and verification | PASS | Review A/B and Verification A/B completed; all findings accepted and dispositioned. |

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` status prevents completion.
