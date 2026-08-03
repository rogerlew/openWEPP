# Verification Agent B

Status: **PASS for executed / `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE`**

Evidence mode: **Static + Ran**

## Independent Verification

The terminal HOLD is truthful. The in-envelope snow correction satisfies the
canonical activation and conservation boundary, but the required downstream
correctness gate deterministically fails. This verification does **not** admit
package completion, a terminal W2A scientific result, the post-correction full
profile, promotion, or EB-04X.

- Canonical authority is aligned: `INV-SNOWFREEZE-089`,
  `TOL-SNOWFREEZE-006`, `TOL-SNOWFREEZE-014`, and `INV-RUNOFFPART-033` require
  typed snowfall greater than `1e-12 m` to be sufficient for activation,
  preserve exactly `1e-12 m` as sub-resolution, preserve existing cold/pack/
  frost triggers, and require absolute daily SWE closure at `1e-9 m`.
- The shared partition independently sums `hourly snowfall_m * 0.1` and checks
  `SWE_before + typed_snow + rain_retained - snowpack_loss - sublimation -
  SWE_after`; it rejects non-finite and material residuals through typed
  `HKERNEL-WB14-RUNOFF-E-002/E-003` errors. The snowbench consumer separately
  reconstructs the same physical ledger and preserves structured kernel errors
  as `SnowKernel`, with consumer-only disagreement as `SnowStorageClosure`.
- The real production test loads the production fixture, creates typed seed
  authority and a production frame, invokes `DirectProductionDayInputBuilder`,
  and proves a warm-mean, zero-pack snowfall reaches the shared partition and
  publishes storage gain, after-day SWE, and hydrology-projection SWE. Static
  tracing confirms the production SIMIMPL28 row builder resolves material
  precipitation before the shared decision. No wrapper, shadow, skeleton, or
  compatibility-only path carries the claim.
- Warm all-rain/no-pack inactivity, exact/just-over activation thresholds,
  mixed-event closure, noncanonical-density rejection, and exact/just-over/
  non-finite closure boundaries are covered by the passing focused tests.
- The assurance identity lock is at terminal generation
  `9e64c4c70ed9a5e77d1d9f1de373ef1ad11b27058d23ff030ec140ecdff36cea`.
  Its `SC-SNOWFREEZE-001` hash matches the current contract; all three checked
  adoption transactions form a continuous generation chain and report no
  invalidated authority. Current assurance validation passes all three reports.
- The retained owning-crate (`653/653`) and frost (`345/345`) evidence is
  consistent with the terminal executable implementation. Later corrections
  were contract wording, evidence, and focused real-consumer/error-code test
  strengthening; their focused reruns pass. Earlier quick/full and first W2A
  evidence are correctly marked superseded or prerequisite-ineligible.
- Independent isolated execution reproduces EROD16 with `231` storm days,
  `170` clean solves, and `61` named flux-closure refusals: `61/231 = 26.4%`,
  violating the retained hard `<=20%` bound. The production fixture completing
  does not waive this assertion. Erosion solver mechanics and any prospective
  gate rebaselining require erosion authority and writes outside this package;
  reverting the canonical snow correction or weakening EROD16 here is not an
  admissible in-envelope route. The unretained `37/227` reversal is correctly
  limited to supporting observation and carries no closure claim.
- Consequently, withholding the post-correction critical full profile and a
  fresh frozen W2A rerun is required after the deterministic quick prerequisite
  failure. The existing eight-cell artifacts remain audit-only and have no
  terminal albedo authority. No full, terminal rerun, EB-04X, or EB-04W2C
  execution is represented as complete.
- `docs/ROADMAP.md`, the snow campaign roadmap, package catalog, package
  status, disposition, scientific synthesis, and archived prompt consistently
  record the cross-domain HOLD, prerequisite-ineligible rerun, authorization-
  needed W2C boundary, and EB-04X block. The tracked/untracked write set is
  confined to the declared contracts, production/test bindings, assurance
  adoption, roadmap/catalog, and package tree. Line-count WARN dispositions are
  present for both 2,000+ line Rust files; no nonexempt file reaches 3,000 lines.
  Non-blocking reconciliation note: current `wc -l` reports
  `runoff_reconciliation.rs` at 2,598 lines rather than the checklist's 2,596;
  both counts retain the same WARN/below-3,000 disposition. The package owner
  should refresh that exact count during finalization.

## Commands Run From `/home/workdir/openWEPP`

| Command | Result |
|---|---|
| Orchestrator focused two-test Nextest expression recorded in `gate-results.md` | PASS, `2/2` |
| Runner focused three-test Nextest expression recorded in `gate-results.md` | PASS, `3/3` |
| `cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract` | PASS, `6/6` |
| `cargo nextest run --test erod16_wave1_continuity_fixture_conservation` | expected blocker reproduced, exit `100`, `61/231` |
| `cargo fmt --check` | PASS |
| `cargo run --quiet -p openwepp-assurance -- validate --all` | PASS, `3/3` reports |
| `git diff --check` | PASS |

## Verdict

No remaining closure-blocking in-envelope finding prevents the executed/HOLD
disposition. The
EROD16 failure is an exact, reproducible, different-process-family hard blocker.
Resume EB-04W2B only after a separately authorized erosion-governed hold-lift
restores the quick gate, then run the still-required terminal full profile,
fresh frozen W2A rerun, and refreshed terminal verification. Until then,
retain `HOLD_CROSS_DOMAIN_CORRECTNESS_GATE` and keep EB-04X blocked.
