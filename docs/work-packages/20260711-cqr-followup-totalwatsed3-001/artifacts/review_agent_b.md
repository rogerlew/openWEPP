# Review agent B

Status: FINAL REVIEW — GO

Evidence mode: Static review plus focused execution

## Scope reviewed

- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs`
- Terminal LCOV, JSON coverage, and CRAP evidence
- Numeric/output equivalence, operand lineage, coverage closure, and obligation-to-test evidence

The production decomposition remains mechanically credible. Column lookup and row-evaluation order are preserved, eager override fallback behavior is unchanged, and floating-point accumulation paths were not reordered.

## Prior finding dispositions

### B-001 — RESOLVED by reviewed exclusion

`for_batch` remains the sole source-named function below the ordinary 75% floor: 66.667% coverage, cyclomatic complexity 7, and CRAP 8.815. Reviewer B independently accepts Reviewer A's closed-list infrastructure exclusion.

The exercised surface includes valid reading, callback success and error propagation, row-offset handling, file-open error mapping, malformed-Parquet reader construction, and stable public error behavior. The remaining reader-build/page-read failures originate inside the Parquet dependency and cannot be selected deterministically through the public API without encoding-specific corruption or a test-only production seam. `for_batch` performs no aggregation, area selection, normalization, conservation calculation, or output mapping. With target-wide line and region coverage above 90% and low function CRAP, the exclusion is technically narrower and safer than altering production solely for test injection.

Terminal evidence is internally consistent:

- Lines: 1,019/1,048, 97.233%.
- Regions: 1,596/1,717, 92.953%.
- Functions: 67/73, 91.781%.
- Maximum target CRAP: 23.00075; no target function exceeds 30.
- Raw hashes in `coverage-after.md` and `crap-after.md` match the retained files.

### B-002 — RESOLVED by executable independent oracle

`two_day_water_storage_and_sediment_oracle_rejects_wrong_aliases` now uses literal source cells for two days with different total areas and independently verifies:

- exact day-key output order plus relevant schema types, nullability, and finite values;
- every published WAT water, storage, profile, interception, and optional-default field;
- PASS `runvol`, `sbrunv`, `tdet`, and `tdep` without deriving runoff from emitted output;
- all five sediment class masses, their fixed-order `sed_del` sum, and class-density `sed_vol_conc`;
- daily storage delta and a deliberately nonzero primary water residual; and
- rejection of Q/QOFE runoff, all-OFE lateral, concentration-sum, common-density, storage, interception, and adjacent-column aliases.

The separate optional reconstruction proves nonzero matched-area TSMF/QRain/QSnow aggregation. Final vectors also cover non-finite Interception, TSMF, QRain, and QSnow behavior. `optional_join_partial_coverage_uses_last_duplicate_wat_key_area` now proves the matched-area denominator, partial optional coverage, unmatched-row exclusion, and current ordered last-duplicate WAT key behavior with deliberately unequal alternatives. The accepted WSHED01 real-consumer cohort is explicitly bound across the mechanical refactor by the unchanged operand, ordering, formula, schema, and output identities established by the current-source oracle.

The numeric-equivalence and exact A-H obligation map now match executable assertions and no longer overclaim the earlier partial oracle.

## Focused validation

- Ran: `cargo nextest run -p openwepp-runner --test totalwatsed3_cli_contract` — PASS, 17/17; run ID `888d5f12-c4c2-43a0-940c-124baec363aa`.
- Static: current source SHA-256 is `c31512f697a5867ae089b599a9131de1247069fa50da03dfaa96248f748530e0`.
- Static: current focused test SHA-256 is `9b33fdfcaa29d4559205c28e1dfb1f83467395d33b1d411c398ea3837ed0f519`.
- Ran: targeted diff check — PASS.
- Ran on the final 17-test worktree: `cargo fmt --check` — PASS.
- Ran on the final 17-test worktree: `cargo clippy --workspace --all-targets -- -D warnings` — PASS.
- Ran on the final 17-test worktree: `cargo nextest run --workspace --profile full` — PASS, 1,776/1,776; run ID `fb1f0fd0-96aa-49b3-b92b-587ee3d446d4`.
- Ran on the final 17-test worktree: `cargo deny check` — PASS.

The exact A-H map binds named tests for every family, including the final optional duplicate-key vector. No workspace-gate failure remains.

## Recommendation

GO. B-001 has a legitimate, explicitly bounded non-science exclusion, and B-002 is closed by executable all-operand, multi-day conservation/publication evidence, exact optional-join coverage, and the bound real-consumer cohort. The final focused suite and all required workspace gates pass. Reviewer B finds no remaining material defect in FQ-04's implementation, evidence, metric disposition, or terminal validation.
