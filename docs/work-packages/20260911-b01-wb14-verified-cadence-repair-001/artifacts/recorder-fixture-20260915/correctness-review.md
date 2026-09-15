# Independent correctness review — recorder fixture isolation

**Reviewer:** `/root/wrapper_correctness`, primary correctness reviewer, independent of the writer and QA reviewer

**Scope:** B01-CONT-026 test-only fixture extraction, native characterization payload, and recorder-isolation evidence. Previously accepted recorder implementation and compile fixes were not reopened.

**Final identities:** candidate tree SHA-256 `719ba2b265f5d1d132b8d31f3ac1296bf6c66e647af42e45062ee74d01ee9cd0`, patch SHA-256 `4378d34a185b7e458d520fd727ebbda23ef2276144744175f31c5581f8338989`; diagnostic reference tree SHA-256 `996e87f5a4aad77fc906691dc1d0ce466eb769767af87c32bd83b849cc593016`, patch SHA-256 `e8a700579d6dfad9bac8db2f716b5004104b3215fa2321f52aa71e950f0492b6`. These are preserved by `final-source.json` and `reference-final-source.json`.

**Evidence class:** Static: inspected the final candidate/reference test surfaces, original callers, fixture operands, and corrected comparison projections. Ran evidence inspected: candidate compilation and the failed reference compilation. The reviewer ran no Rust, model, reader, or raw-corpus command.

## Findings

### HIGH — the diagnostic reference is not compilable, so mandatory behavioral acceptance is absent

`reference-check.json` records exit 101 at 2026-09-15T05:54:04.853018Z. Its compiler output reports that `crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs:78` cannot include `../../../../tests/integration/land_surface_energy_real_hydrology_shadow_contract.rs` because the detached reference copy omitted the repository `tests/` directory. This is a reference-preparation failure rather than a defect demonstrated in the candidate source.

The failure reached the adopted 27/27 correction ceiling. Consequently, the matched reference/candidate characterization pair, compiled test listing, original 540-second regression, actual recorder producer/isolation test, and selected Clippy comparisons did not run. Static source structure cannot establish that reference and candidate native results match or that recorder-disabled, non-target, enabled, and forced-error runs preserve identical canonical custody. B01-CONT-026 acceptance is therefore unmet.

### MEDIUM — complete test oracles intentionally mirror non-serializable owner structures

`crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs` manually projects every `V11ParentCandidate` field and every `V11AcceptedSegmentCandidate` field, including f64 cumulative debits as exact bits. The recorder tests also retain separate producer-local expected projections from the recorder encoder, as reviewed in the preceding recorder scope.

This duplication is justified here: using the recorder projection as its own expected value would destroy oracle independence, while adding production serialization was outside authority. It remains a maintenance risk when either source type gains a field. Future changes must update both projections and retain mutation-sensitive equality; no completeness claim beyond the frozen types is supported.

## Static fix verification

- **PASS:** The source-backed 540-second oracle remains attached to the original regression. `exercise_complete_wb14_cadence` keeps its original name, ten-argument signature, and named forwarding of every input. The unchanged covered-physical caller file still includes all original call sites. The exact 540 assertion remains both in the extracted legacy terminal assertion and in the original `capture_terminal_failure` branch. No generic flag, recorder-mode predicate, tolerance widening, alternate tick, or panic-catching bypass was added.
- **PASS:** The native fixture is explicitly separate from the legacy absolute-time oracle. It fixes `NativeMixedPhase`, exact SWE/cold/boundary/lane and production inputs, requires native V3/V4 residents, and checks a real terminal event, accepted-event tick join, consumed terminal parcel/digest join, contiguous terminal prefix, ordinal-zero snow-free successor, owner/snow/pending-parcel lineage, and complete successor chronology. The observed native tick is emitted only in a payload labelled diagnostic, with no claim that 360 seconds is scientific authority.
- **PASS:** The earlier recorder-isolation blocker is corrected. Snow-free recorder runs retain the full `canonical_comparison_payload` and compare disabled against non-target, enabled, capture-failure, wrong-phase, and required-omission modes. The payload covers exact fixture inputs; beginning and ending parent, complete owners, clock and Stage3 owner; full finalized-parent state, accepted segments/checkpoints, debits and transfers; ordered subslabs and custody; event groups and terminal custody; terminal parcels; and snow-free successor receipts. Diagnostic recorder rows are excluded from this model equality.
- **PASS:** The native reference/candidate characterization payload uses the same shared test harness and disabled observer posture. It includes exact physical/canonical fixture results, adaptive receipt/proof work, carrier supports, complete terminal-batch fields, publication and performance topology, complete-owner comparison, and comparison audits. Elapsed timing diagnostics are kept outside equality.
- **PASS:** The common execution helper takes a private request structure, avoiding a second high-arity/bool-heavy core signature and adding no lint suppression. The thin legacy wrapper retains its pre-existing signature solely to preserve callers.
- **PASS:** All new execution observation is test-only. The snow-free successor seam is guarded by `#[cfg(test)]`, passively clones the genuine production return, and does not alter arithmetic, clamps, units, guards, physical selection, chronology, admission, owner installation, error taxonomy, dependencies, or production schemas.
- **Ran, PASS:** `candidate-check-corrected.json` reports exit 0 for the final candidate library and test targets with `persisted-restart-v1,restart-authority-evidence`; source stayed unchanged during the command. This verifies the corrected `ModelTimeNs` qualification and final candidate compilation only.

## Residual risk and missing tests

- Reference/candidate payload equality is NOT RUN; absence of a recorder-induced numerical or custody difference is unproved.
- The original legacy 540-second test and every recorder covered/snow-free, non-target, disabled/enabled/forced-error, wrong-phase, omission, selector, and exact-selection control are NOT RUN on the final cut.
- Owning-crate and runner Clippy comparisons and a terminal format check are NOT RUN. Formatting was applied before final preservation, but no final unchanged-source format check supplies acceptance.
- The prior dynamic-hydrology limitation remains: exercised rows retain exact internal reconstruction equality, but there is no independent mutation-sensitive expected dynamic-frame payload for future fields.
- The outer day 4 / interval 22 versus nested day 0 / interval 0 disagreement, authentic context acquisition, complete prefix authentication, native reader, conservation/restart/science qualification, and cadence repair remain unresolved and outside this fixture-only scope.

## Verdict

**HOLD / NO APPROVAL for recorder preparation.** The final candidate compiles and the fixture-isolation source addresses the identified static correctness gaps without production behavior changes. The incomplete detached reference prevents every required behavioral comparison, and the 27/27 hard stop leaves those obligations unexecuted.
