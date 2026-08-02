# EB-04V Review Agent A

Evidence class: `[Static] + [Ran]`.

Verdict: `APPROVE IMPLEMENTATION / HOLD PACKAGE CLOSURE`.

No implementation or science-contract finding remains open. Package closure is
still blocked because `artifacts/gate-results.md` remains `queued / not-run`;
the package's selected warnings-denied Clippy, quick, frost, full-workspace,
documentation, schema, security, and exact-terminal-diff gates therefore do
not yet have terminal evidence. This review does not substitute for those
gates or for Review B and terminal verification.

## Scope reviewed

Static review covered the current EB-04V diff and its governing
`SC-SNOWFREEZE-001` revision 120, especially `INV-SNOWFREEZE-087`,
`OBL-SNOWFREEZE-P-061`, and `TOL-SNOWFREEZE-012`. The reviewed implementation
path was:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`:
  typed operands, process attribution, cap handling, closure, finite guards,
  snow-free/legacy applicability, and the centralized wet/dry compaction
  mutation arithmetic;
- `SnowDensityRuntimeOutcome` and `DirectSnowLiquidPartition`: typed handoff;
- `runoff_reconciliation.rs`: Stage-3 adjustment and typed error mapping;
- `00c_day_input_builder_impl.rs`: the real
  `openwepp-r7h-direct-production-snow-trace-v2` consumer;
- contract and regression tests, the EB-04V analyzer, operand lineage,
  execution receipt, result object, synthesis, and invalidated-evidence
  custody records.

The review checked arithmetic order, units, clamp attribution, non-finite
handling, typed error reachability, real-consumer publication, duplication,
anti-alias evidence, observation-operator continuity, and claim calibration.

## Checks run

The reviewer ran the following against the current worktree:

- `cargo fmt --all -- --check`: passed;
- `cargo test -p openwepp-hillslope-orchestrator snow_density -- --nocapture`:
  12 passed;
- `cargo test --test snow_surface_eb04v_density_process_diagnostics_contract -- --nocapture`:
  2 passed;
- `cargo test --test hphys0296_snow_rm_acceptance_authority_contract -- --nocapture`:
  3 passed;
- `cargo test --test snowdensity10_3_22_climate_class_density_specialization -- --nocapture`:
  5 passed;
- `git diff --check`: passed;
- exact receipt/result inspection: 36 of 36 cells returned zero from release
  binary
  `fb670d086937a7785a2549339832f71b96fc98f3c8992ec8d24961123b33826f`;
  the receipt's binary hash matches the current release binary;
- independent all-cell common-trace comparison against EB-04R: 36 files and
  574,196 JSONL rows were identical after removing only the schema identifier
  and the newly authorized `density_process_*` fields;
- independent retained-row reconstruction on Harvard hardwood B row 16,435:
  wet uncompensated bulk movement was `517.3211460002838 kg m^-3` both
  independently and in the emitted ledger; independently reconstructed
  internal cap movement was `-90.3100535308107 kg m^-3` versus emitted
  `-90.31005353081076 kg m^-3`, with emitted closure `5.684e-14 kg m^-3`;
- result-object audit: maximum independent ledger closure was
  `3.410605131648481e-13 kg m^-3`; maximum emitted-versus-reconstructed
  difference was `5.686e-14 kg m^-3`; 100,824 rows reject final density as a
  fresh-density alias; all nine retained B-cell paired counts and KGE
  components reproduce EB-04R, with maximum component difference
  `4.441e-16`.

The current evidence binds execution receipt
`f2cc806de485cdbc00bc4c5b9e0e778ccb62fd6e1582d511fe5ea2b47f7fb1be`
and analyzer
`e8c608af74b4cb9747ff16011484782edc612e6e7da97c604449c5daef21b3e9`.

## Finding chronology and disposition

| ID | Severity | Finding | Disposition |
|---|---|---|---|
| EB04V-A-001 | High | Initial multilayer wet/internal-cap attribution summed layer-local density deltas instead of reconstructing the exact bulk-density movement. | `ACCEPTED / CORRECTED`. The implementation now evaluates sequential capped and uncapped bulk states, and the independent two-layer unit vector plus retained Harvard row reproduce the ledger. The original cohort is explicitly invalidated. |
| EB04V-A-002 | High | The first ledger could publish non-finite driver/process fields and failed closure without a typed hard error. | `ACCEPTED / CORRECTED`. Every floating field is checked for finiteness, closure above `1e-9 kg m^-3` returns `DiagnosticClosureViolation`, and downstream mapping preserves a typed guard error. NaN, infinity, overflow, and failed-closure tests pass. |
| EB04V-A-003 | Medium | Legacy, snow-free, climate-fallback, structural, cap, and Stage-3 applicability could be ambiguous. | `ACCEPTED / CORRECTED`. Legacy and snow-free ledgers are the exact default/inapplicable record; fallback use has a separate Boolean and signed delta; structural, internal/runtime cap, and Stage-3 deltas are distinct. The snow-free climate-class case is explicitly tested. |
| EB04V-A-004 | Medium | Initial tests/evidence did not independently separate all process terms or reject enough plausible aliases. | `ACCEPTED / CORRECTED`. Isolated fresh, wet, PTM, POC, cap, fallback, structural, Stage-3, omitted-term, and non-finite vectors now pass. Fresh density is evaluated before mixing and materially differs from final density in 100,824 retained rows. |
| EB04V-A-005 | Medium | Dry-compaction arithmetic was duplicated between bulk helpers, risking silent science divergence. | `ACCEPTED / CORRECTED`. State mutation is centralized in `apply_time_compaction_scaled_with_overburden`; the wrapper forwards to it, and diagnostics consume its returned tendencies without reimplementing mutation. |
| EB04V-A-006 | High | Corrected source postdated the retained release executable, so the cohort was not exact-terminal evidence. | `ACCEPTED / CORRECTED`. The intermediate cohort is retained as invalidated evidence. The terminal cohort was rebuilt and rerun from the later release binary `fb670d...`; 36 of 36 cells passed and receipt provenance matches. |
| EB04V-A-007 | Medium | Advancing `SC-SNOWFREEZE-001` to revision 120 left 33 integration contracts bound to literal revision 119; the climate-class target failed 4/5. | `ACCEPTED / CORRECTED`. All 33 bindings now name revision 120; the previously failing climate-class target passes 5/5. |
| EB04V-A-008 | Medium | The first terminal analyzer indexed `rubric_profile` as a list, but the retained EB-04R schema stores its rows under `rubric_profile.cells`; analysis failed after the 36 executions. | `ACCEPTED / CORRECTED`. The authority lookup now follows the retained schema. Analysis-only execution passed, hash-binds the tool and receipt, and exactly reproduces all nine B-cell observation-operator anchors. |

## Residual risk and missing validation

The implementation evidence is strong for the diagnostic claim: mutation
arithmetic is centralized, the ledger closes far inside tolerance, exact
common-field traces are unchanged, and the analysis remains explicitly
diagnostic-only. The snow-free climate-class early return intentionally makes
an inapplicable update neutral rather than requiring unused class/day operands;
the new unit test and `INV-SNOWFREEZE-087` support that interpretation. Because
the 36-cell population does not isolate this opt-in edge against a predecessor
runtime, terminal full-workspace validation remains important regression
coverage.

The sole current closure blocker is missing terminal gate evidence. Review A
may become `PASS / NO BLOCKERS` only if the selected gates are run at the exact
terminal source state, `artifacts/gate-results.md` records explicit passing
results, no later production or analysis change invalidates the retained
hashes, and Review B plus both verification artifacts independently close.
