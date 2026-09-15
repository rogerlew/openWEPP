# Secondary QA review — reference-input recovery

**Reviewer:** `/root/recovery_qa`, authorized replacement for unavailable `/root/wrapper_custody_qa`; independent secondary QA review with no continuity claim.

**Evidence class:** Static: inspected the authorization, package acceptance and current recovery artifacts, and reused the unchanged accepted static QA findings from `artifacts/recorder-fixture-20260915/qa-review.md`. Ran evidence inspected: reference compilation and exact Nextest discovery records. I ran no build, test, model, reader, capture, or pair command.

## Findings

### HIGH — listing post-processing misread an unfiltered aggregate as the selected-test count; required comparison never launched

**Paths:** `artifacts/reference-input-recovery-20260915/reference-list.stdout`, `artifacts/reference-input-recovery-20260915/reference-list.json`

The reference discovery command itself is PASS, exit 0, with source unchanged. Its top-level `test-count: 1470` is the crate's complete listed inventory, while the frozen filter selects exactly one case: `v9_real_consumer_shadow::tests::adaptive_production_path_coverage::native_mixed_phase_fixture_characterization_payload`. Treating 1470 as the filtered count caused the conservative failed verification 29/29 and hard stop before any reference/candidate native pair command or result artifact existed.

This blocks the required all-fields reference/candidate comparison and therefore blocks the original 540-second regression, recorder-mode controls, terminal formatting, matched owning/runner lint, and their acceptance reviews. The package remains **HOLD / NO APPROVAL**. The list record is valid discovery evidence; it is not pair or behavioral evidence.

### HIGH — selected behavioral and source-quality requirements are unrun at the enforced stop

**Paths:** `artifacts/reference-input-recovery-20260915/`, `package.md`

The restored reference `cargo check --lib --tests` is PASS, exit 0, and exact discovery is PASS, exit 0. Neither establishes the required native characterization equality or the quality workflow. No pair output/comparison, timing-regression result, recorder positive/negative-control result, `cargo fmt --check`, or matched owning/runner Clippy result is present for this recovery cut. Compilation warnings in the listing stderr also cannot be treated as a warnings-denied lint result. `cargo deny` is not a current-scope requirement: this recovery changed no manifest, lockfile, dependency, toolchain dependency, or workspace resolution, and the adopted checks do not select it.

## Non-blocking debt and follow-ups

- The recovery custody record is sufficient for the restored inputs: it preserves the prior absent-root-`tests/` receipt, supplements rather than rewrites the historical 741-entry membership, records 7,980 regular files (1,413,362,008 bytes) and 1,318 internal symlinks, and verifies all restored inputs against the producing base and candidate. All symlink targets remain under `tests/`; the additional compile-time matrix input is likewise base/candidate-equal.
- The preliminary no-symlink assertion was incompatible with the immutable base's legitimate internal symlinks and was conservatively charged as failed verification 28/29. Future copy preflights should validate internal targets rather than assert that the tree contains no symlinks.
- The accepted static oracle-maintenance debt remains unchanged: manually projected owner and accepted-segment payloads need mutation-sensitive maintenance as owner fields evolve, and dynamic-hydrology payload fields still lack an independently expected future-field oracle.

## QA disposition

**HOLD / NO APPROVAL.** Executable-input custody is restored and bound to the successful reference compiler/discovery records. The hard stop at 29/29 prevented the required matched native pair and every downstream behavioral and quality check; it provides no timing, scientific, or production-qualification acceptance.
