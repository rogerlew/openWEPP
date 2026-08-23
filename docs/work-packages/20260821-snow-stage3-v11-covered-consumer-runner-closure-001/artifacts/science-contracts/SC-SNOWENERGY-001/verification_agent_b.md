# SC-SNOWENERGY-001 v15 verification — Agent B, second pass

Evidence: **Static + Ran.** I independently inspected the amended contract,
registry, disposition, receipt constructor/validator, covered runtime topology
projection, poison tests, expanded contract test, and exact-worktree gate
record. I ran `git diff --check`, which passed. I did not independently rerun
the Nix/Rust, strict-binding, or assurance commands; their results below are
verification of the command evidence recorded in `gate-results.md`, not a claim
that this verifier executed them.

## Accepted-finding verification

| Accepted finding | Status | Evidence |
|---|---|---|
| `A-001 / B-01` | **closed** | Released terminal-tolerance semantics remain `INV-SNOWENERGY-041`; OFE-ground authority is uniquely assigned `INV-SNOWENERGY-042`. Canonical, Child 2C, Binding Exposure, and test references agree (`SC-SNOWENERGY-001.md:809,1107,1117,1373`; `snow_stage3_shared_carrier_authority_contract.rs:163-179`). |
| `A-002 / B-02 / B-03` | **closed** | `REF-SNOWENERGY-USER-OFE-GROUND-V15` records the prospective authority and evidence classes (`SC-SNOWENERGY-001.md:160`). `INV-042`, `OBL-SNOWENERGY-C-018`, their guard posture, and the current package's active Binding Exposure row are present at `:809,840,1117`. Binding Exposure conservation is restored. |
| `A-003 / B-04` | **closed** | `TOL-SNOWENERGY-002` canonically admits only a dimensionless `1e-12` summation residual and prohibits normalization (`SC-SNOWENERGY-001.md:960`). Runtime uses the named `STAGE3_OFE_TILE_FRACTION_CLOSURE_TOLERANCE` at `snow_stage3_terminal_handoff.rs:641,813` and the covered runtime uses the same constant. |
| `A-004 / B-05` | **closed** | Contributions bind a common beginning Stage 3 digest, bit-identical snow temperature, and bit-identical latent heat (`snow_stage3_terminal_handoff.rs:657-675,779-786`). Aggregate state operands are copied from the common state, while only fluxes are weighted (`:797-819`). The threshold-dependent effective latent heat is removed, and opposing-vapor/common-state poison coverage is present (`:2117-2203`). |
| `A-005 / B-06` | **closed** | `LaneBoundaryTopologyExpectationV1` supplies an independent ordered tile ID, exact fraction bits, closed boundary class, and admitted model digest (`snow_stage3_terminal_handoff.rs:649-655`). `try_new` requires this expectation and joins it before sealing (`:699-710`); `validate_topology` compares cardinality and every field exactly (`:869-889`). Runtime builds the expectation directly from configured destination/fraction and the admitted covered model before constructing each contribution (`v9_real_consumer_shadow.rs:639-677`), so the authority is no longer mapped back from self-declared receipt fields. All four source sets are reconstructed independently (`snow_stage3_terminal_handoff.rs:856-897`). Fresh-seal class and model substitutions are rejected at `:2205-2213`. Runtime still cannot authorize an open-snow claim and correctly fails closed until that producer exists. |
| `A-006` | **closed for v15 scope** | The deterministic adopter wire, source-set wire, ordering, widths, endianness, and encodings are specified at `SC-SNOWENERGY-001.md:1328-1351`. The contract explicitly prohibits this wire from additive restart or coupled-parent authority until a future canonical-framed amendment with fixed vectors. No stronger identity claim is made here. |
| `A-007 / B-07` | **closed** | The expanded contract test checks corrected ID counts, the unique canonical C-018 row, Binding Exposure/tolerance/reference presence, prohibited old basis/normalization source, topology expectation, authority-join failure, fresh-seal class/model poisons, and source-set reconstruction (`snow_stage3_shared_carrier_authority_contract.rs:153-204`). It now detects the reviewed v15 schema and binding defects. |
| `A-008` | **closed** | `gate-results.md` now records a final in-review worktree qualification: formatting PASS; orchestrator test compilation PASS; focused v15 contract test 1/1 PASS; focused receipt/runtime tests 3/3 PASS; strict Binding Exposure PASS with 14 rows; typed assurance validation PASS; assurance generation verification PASS across 86 transitions; and diff checking PASS. The explicitly reported Clippy failure is pre-existing and is not misrepresented as a pass. Dual re-verification, rather than missing execution evidence, is the remaining promotion-cycle step. |

## No-regression audit

- Contract frontmatter and the lifecycle registry remain truthfully aligned at
  `v15 / in_review / draft / pending` during review.
- The complete OFE-ground sum remains `sum(f_i X_i)` with no covered-fraction
  normalization. Incomplete mixed-surface runtime execution still fails closed.
- The new topology expectation does not authorize an open-snow producer; it
  admits only the configured covered destination/model currently constructed by
  this runtime.
- No rejected review finding exists; every accepted finding is now implemented
  consistently with `disposition.md`.
- The missing real open-snow producer, component-resolved carrier, precipitation,
  soil heat, outcome ledger, terminal chronology, and additive restart remain
  visible package-level blockers. This contract-cycle result does not close the
  implementation package.

## Verdict

**PASS-WITH-NOTES.** All accepted v15 review findings are closed and the
recorded final-worktree qualification supports contract-cycle promotion after
the other independent verifier passes and lifecycle metadata/assurance identity
are finalized on the promotion bytes.

Retain the overall work package at `EXECUTING / HOLD`; this verification admits
the Option-A authority revision, not the unfinished covered-branch physical
implementation or restart/cutover.
