# Defect Ledger

Status: `CORRECTION-CANDIDATE-PASS`

| Defect | Reproduction | Mechanism | Authority/owner | Regression | Disposition |
| --- | --- | --- | --- | --- | --- |
| `INTVAL-AUTH-BIND-001` | Exact release required-authority lane exits 1 | Seven active required suites bind five targets deleted together by `a381702b` | openWEPP authority tests/Cargo; current contracts and deleted-test assertion provenance | all-active-required-bindings source guard plus restored targets | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-TEST-LINT-001` | Exact release candidate 1 exits 101 in Clippy before authority/stability | Two restored tests use strict `f64` equality, violating denied `clippy::float_cmp` | openWEPP integration tests; suite absolute tolerance `1.0e-12` | release Clippy red; replace equality with declared absolute-tolerance checks | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-TEST-LINT-002` | Post-candidate workspace Clippy exits 101 | AUTH11 extension comparison and predecessor AUTH06 provenance-test length violate denied lints | openWEPP authority integration tests; assertions unchanged | workspace Clippy red/green | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-EROSION-TOE-001` | Exact release candidate 2: first failure WB05C-CO-H0034 lane 3 day 404, `erosion.wave1.segment_toe`; 36 cases share the signature | Wave-1 slope stations are divided by declared `slplen_m`; parser-compatible terminal stations 0.9995..0.9999 therefore do not reach normalized toe 1 | `SC-SED-001` EROD16 rev 54 and pinned `profil.for:37,54` terminal-station normalization | focused `profil.for` vector and exact p34 red/green; candidate 3 has zero segment-toe failures | `CORRECTED-FAMILY-GREEN` |
| `INTVAL-GROWTH-RTMMAX-001` | Candidate 3 first ordered failure WB05C-CO-H0259; H0327 shares `growth.rtmmax` | Rust rejects perennial zero cap and omitted baseline pre-increment saturated-cap branch; merely relaxing guard would divide by zero | `SC-PLANT-001` rev 20; pinned `grow.for:529-601`, `init1.for`, `infile.for` | zero-cap saturated branch state vector red/green; exact H0259/H0327 green | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-PERC-INGRESS-001` | Candidate 3: 210 `percolation.soil_water_after_m` failures; representative MO-H0001 day 724 | positive daily ingress near 2e-11 is split into 24 increments <=1e-12 and each producer increment is incorrectly dropped | existing `SC-PERC-001#INV-PERC-017`; pinned hourly `xfin` ingress | tiny-positive 24-substep storage/closure vector red/green; exact MO-H0001 green | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-PERC-RESTRICT-001` | Candidate 3: 21 `percolation.restrictive_layer_conductivity_m_s` failures, 13 cohort + 8 watchlist | runtime requires positive `kslast`; valid exact zero is the baseline impermeable restrictive boundary | `SC-PERC-001` rev 30, `SC-INFILE-SOIL-001` rev 0.1.12; pinned `input.for`/`perc.for` | daily/hourly exact-zero vector red/green; exact cohort/watchlist cases progress beyond restriction | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-FROST-THAW-CLEAR-001` | all eight zero-restrictive watchlist replays progress to `no-final-frost clear cannot debit` at 1.29-1.31 mm | runner clears prior frozen depth and restores residual water before R4W, then incorrectly rebalances to the pre-`frwatc` scalar | `SC-SNOWFREEZE-001` rev 117, pinned `frostn.for:686`/`frwatc.for:80-137`, `SC-WATBAL-001#INV-WATBAL-095` | exact 1.303248764 mm thaw-complete vector red/green; all eight real CLI replays green | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-CONTRACT-VERSION-BIND-001` | candidate 4 full workspace: 32 failures requiring missing `contract_version: 115` | snow/paradigm integration marker guards lag the contract header, whose history already contains revision 116 and whose current amendment is 117 | test-only marker bindings to canonical `SC-SNOWFREEZE-001` header | candidate 4 family red; all 32 target binaries 120/120 green after exact marker update | `CORRECTED-RELEASE-GREEN` |
| `INTVAL-EROSION-CLASS-FRACTION-001` | candidate 5: OR-H0081 and OR-H0204 fail `erosion.wave1.publication.class_fraction must be nonnegative` | pinned do-40 raises every class to an absolute `1e-15`; at trace `ldbot` the floored sum exceeds total and label 50 redistributes a negative shortfall | `SC-SED-001#INV-SED-017` rev 55; pinned `enrich.for:341-377` with deliberate correction of its degenerate trace-load behavior | direct trace-load negative-fraction vector red/green; exact OR-H0081 and OR-H0204 real CLI green | `CORRECTED-RELEASE-GREEN` |

## INTVAL-AUTH-BIND-001 Seven-Gate Record

Evidence class: **Ran + Static**.

1. Reproduction: AUTH11 generic binding guard failed before restoration on the
   first missing AUTH05 path (`logs/01-auth11-red.time`, exit 100).
2. Mechanism: active registry paths and root Cargo registrations survived the
   joint deletion of their five owned source targets in `a381702b`.
3. Ownership: root Cargo and integration tests are inside the declared first
   write set.
4. Authority: the seven suite documents bind `SC-SOIL-001`,
   `SC-WATBAL-001`, and `SC-SUBHYD-001#INV-SUBHYD-016..019`; deleted tests
   supply assertion provenance only.
5. Safety: the correction changes tests/registration only and does not restore
   the deleted symbol-map runtime, loosen a guard, or alter fixtures/tolerances.
6. Testability: the generic all-active-required binding guard was red before
   the five files and registrations existed and is green afterward.
7. Validation: all 11 focused tests across AUTH11 and the five restored
   current typed-runtime targets passed; anti-evasion and AUTH11 also passed.

## INTVAL-TEST-LINT-001 Seven-Gate Record

Evidence class: **Ran + Static**.

1. Reproduction: exact release candidate 1 exited 101 at workspace Clippy
   (`logs/02-release-candidate.log`, `.time`).
2. Mechanism: strict `f64` equality in HPHYS0224 and HPHYS0225 triggered the
   workspace-denied `clippy::float_cmp` lint.
3. Ownership: both assertions are in the declared restored-test write set.
4. Authority: both owning suites specify absolute tolerance `1.0e-12` for
   withdrawal/storage checks.
5. Safety: the correction changes only test expression form.
6. Testability: release Clippy is the direct red/green regression surface.
7. Validation: workspace Clippy and 16/16 focused authority tests pass after
   both lint corrections; exact release restart is in progress and no pre-fix
   result is reused.

## INTVAL-TEST-LINT-002 Seven-Gate Record

Evidence class: **Ran + Static**.

1. Reproduction: workspace all-target Clippy reported both named findings.
2. Mechanism: lint-incompatible expression shape and missing function-scoped
   complexity annotation; neither finding is semantic.
3. Ownership: authority integration tests are inside the package envelope; the
   intended write set was amended before editing AUTH06.
4. Authority: AUTH06 retains its provenance assertions and AUTH11 retains its
   exact active-binding path/registration checks.
5. Safety: use `Path::extension` and annotate only the unchanged provenance
   test; no guard, threshold, fixture, or runtime behavior changes.
6. Testability: workspace all-target Clippy is the direct regression.
7. Validation: workspace Clippy passes and 16/16 focused authority tests pass;
   exact release restart is in progress.

## INTVAL-EROSION-TOE-001 Seven-Gate Record

Evidence class: **Ran + Static**.

1. Reproduction: candidate 2 reached stability and failed first at
   WB05C-CO-H0034 lane 3 day 404 with `erosion.wave1.segment_toe`; an exact
   focused CLI rerun reproduced it.
2. Mechanism: the accepted lane-3 terminal station is `0.9997`; current Wave-1
   derivation divides dimensional stations by declared length and emits that
   value as the normalized toe. All 36 same-signature cases terminate from
   `0.9995` through `0.9999`.
3. Ownership: SC-SED EROD16, the Wave-1 derivation, and its crate test are in
   intended-write-set revision 2.
4. Authority: pinned baseline `profil.for:37,54` sets `slen` to the terminal
   input station and divides every `xstar` by it; SC-SED-001 rev 54 now binds
   this behavior before production edits.
5. Safety: retain declared physical length and the hard toe guard; do not
   clamp, relax tolerance, or modify erosion equations.
6. Testability: the near-terminal contract vector fails against current code
   because interior and terminal coordinates use the wrong denominator
   (`logs/04-erod16-terminal-normalization-red.log`, exit 100).
7. Validation: the focused vector passes, focused crate Clippy passes, and the
   rebuilt exact release CLI completes p34 cleanly with the real runner/runtime
   consumer (`logs/05-*`, `logs/06-*`). Exact release candidate 3 is the
   family-wide and next-ordered-defect gate; no candidate-2 result is reused.

Candidate 3 result: all 1,955 workspace tests and all required authority lanes
passed; stability contained zero `erosion.wave1.segment_toe` failures. This
closes the full 36-case signature family at the post-fix source.

## INTVAL-EROSION-CLASS-FRACTION-001 Seven-Gate Record

Evidence class: **Ran + Static**.

1. Reproduction: candidate 5 passed every pre-stability gate and failed only
   OR-H0081 and OR-H0204 at the hard nonnegative publication guard.
2. Mechanism: the pinned do-40 absolute floors make `sum(gend) > ldbot` for a
   trace routed load; label 50 then computes a negative shortfall and creates
   negative class mass. H0081's captured final loads include
   `-3.642639932051419e-15`.
3. Ownership: SC-SED, the enrichment producer, its internal test, and package
   evidence are in intended-write-set revision 6.
4. Authority: `SC-SED-001#INV-SED-017` revision 55 retains the pinned
   dimensionless `1e-15` floor but restores total-load authority before caps;
   the pinned baseline's invalid negative-mass degeneracy is not a target.
5. Safety: renormalization runs only when the existing floor fires. Ordinary
   no-floor vectors, total erosion mass, publication guards, tolerances, and
   fixtures are unchanged.
6. Testability: the direct trace vector failed with one fraction
   `3143.153396...` and four `-785.538349...`, then passed after correction.
7. Validation: all ten HB04 characterizations and crate Clippy pass; rebuilt
   release CLI replays of exact OR-H0081 and OR-H0204 both exit zero. Candidate
   6 is the full family and next-ordered-defect gate.

## Candidate-3 Three-Family Seven-Gate Summary

Evidence class: **Ran + Static**.

### INTVAL-GROWTH-RTMMAX-001

1. Reproduction: first ordered candidate-3 failure is H0259; H0327 is the only
   other identical failure.
2. Mechanism: valid Bare perennial records carry `rtmmax=0`; Rust requires
   positive and lacks the legacy cap-already-reached branch.
3. Ownership: PL contract and growth runtime/test paths are in write-set rev 3.
4. Authority: pinned `grow.for` checks `rtmass >= rtmmax` before increment or
   division, sets root mass to the cap, and root depth to the soil/`rdmax`
   envelope. Parser/input lineage reads zero directly.
5. Safety: restore branch ordering; do not only relax the guard.
6. Testability: existing zero-invalid assertion supplies the red guard and a
   state vector will prove finite zero mass plus bounded root depth.
7. Acceptance: focused tests, exact H0259/H0327, and candidate 4.

### INTVAL-PERC-INGRESS-001

1. Reproduction: 210 candidate-3 failures share the closure signature; traced
   representative daily ingress is `2.0002385869594824e-11`, whose `/24`
   increment is `8.33432744566451e-13`.
2. Mechanism: `apply_same_pass_infiltration` drops each positive increment at
   the generic zero threshold while the scalar ledger retains the daily input.
3. Ownership: subsurface producer and R4MO tests are in write-set rev 3.
4. Authority: existing `INV-PERC-017` requires every positive hourly `xfin`
   before percolation; pinned baseline has no positive epsilon gate.
5. Safety: change only the producer no-op predicate to exact non-positive after
   nonnegative validation; retain the closure tolerance and guard.
6. Testability: a 24-substep tiny-positive vector is red on current producer.
7. Acceptance: focused closure vector, exact MO-H0001, and candidate 4.

### INTVAL-PERC-RESTRICT-001

1. Reproduction: 21 candidate-3 cases all have active restriction,
   `ui_bdrkth=10000`, and `kslast=0`.
2. Mechanism: runtime positive validation contradicts parser and baseline
   exact-zero impermeable-boundary semantics.
3. Ownership: PERC/soil-input contracts, subsurface runtime, and R4MO tests are
   in write-set rev 3.
4. Authority: pinned `input.for` accepts zero and `perc.for` explicitly emits
   zero effective conductivity when either operand is non-positive.
5. Safety: preserve active restriction and exact zero; negative/non-finite
   still fail. Disabling restriction is prohibited because it would leak.
6. Testability: daily/hourly exact-zero vectors are red on current validation.
7. Acceptance: focused vectors, exact cohort p80 and watchlist p13, and
   candidate 4.

## INTVAL-FROST-THAW-CLEAR-001 Seven-Gate Record

Evidence class: **Ran + Static**.

1. Reproduction: p13/p14/p25/p27/p40/p43/p45/p49 all expose the same
   no-final-frost debit after the exact-zero restrictive correction.
2. Mechanism: clearing prior frozen depth restores `thetdr * frozen_depth`
   before R4W, but the helper targets the pre-exchange scalar that excludes it.
3. Ownership: SC-SNOWFREEZE, the runner helper/test, and evidence are in
   intended-write-set revision 4.
4. Authority: pinned `frostn` calls `frwatc(0)` at day end/thaw completion;
   pinned `frwatc` reconstructs coarse liquid and frozen stores together.
   Revision 117 binds that single-owner ordering and cross-links WATBAL-095.
5. Safety: preserve the original layer basis only for an already-computed
   material outcome; retain the nonmaterial stale-clear path unchanged.
6. Testability: the exact `0.0013032487641802763 m` residual-thaw vector fails
   before correction with the production signature (`logs/11-*`).
7. Validation: the focused material and nonmaterial tests pass, runner Clippy
   passes, and all eight real release-CLI watchlist cases complete cleanly
   (`logs/12-*`, `logs/13-*`). Candidate 4 is the full-family gate.

## INTVAL-CONTRACT-VERSION-BIND-001 Seven-Gate Record

Evidence class: **Ran + Static**.

1. Reproduction: candidate 4 runs 1,959 tests and reports 32 failures, all
   requiring the absent literal `contract_version: 115`.
2. Mechanism: the guards pin a stale header revision; the contract history
   already contains v116 and the current canonical header is v117.
3. Ownership: the exact 32 integration files are authorized in write-set
   revision 5.
4. Authority: the canonical contract header and revision history own the
   current version; individual historical package assertions remain unchanged.
5. Safety: mechanically replace only the expected header marker, with no
   science, tolerance, fixture, suite, or production changes.
6. Testability: candidate 4 is the family red gate; each failure names the
   same missing marker.
7. Validation: all 32 target binaries run 120/120 tests green after correction
   (`logs/15-contract-version-bind-green.*`). Candidate 5 is the full gate.
