# EROSION GROUND-COVER AUTHORITY — DEFECT CLOSURE

Status: `EXECUTED — AWAITING CODEX REVIEW` (Claude-executed, operator-directed 2026-07-05:
"scaffold and execute the follow-on defect closure work-package").
Branch: `erosion-cover-authority-defect-closure`. Shape: defect-closure
ExecPlan — diagnose AND correct in one pass; the diagnosis is inherited
from the E.5 adjudication (`GAP-SED-009`,
`20260703-…/artifacts/increment-4-magnitude-adjudication.md`).

## Defect (inherited, E.5-attributed; entry recon COMPLETE)

The erosion daily covers (`rilcov`/`inrcov`) consume
`residue_partition.cover_fraction`, and the production builder hardcodes
the partition inputs to ZEROS
(`00d_authority_runtime_impl.rs`: `standing/flat_offset/buried/
cover_fraction: 0.0`) — ground cover is 0 by construction on EVERY run.
Forest ICs declare `inrcov`/`rilcov` (p61: 0.85/0.85); legacy honors
them; openWEPP runs erosion bare-soil → the `GAP-SED-009` ~4–6×
over-detachment.

**Entry recon (Ran static, 2026-07-05) — the E.5 open question is
RESOLVED: legacy is (a) recomputed-from-pools.**
- `init1.for:295-297`: the declared covers seed the ground-residue pools
  by the exact inverse `rilrm = ln(1−rilcov)/(−cf)`,
  `rigrm = ln(1−rigcov)/(−cf)` (`rigcov = inrcov`, `:134`), per residue
  type (`cf(iresd)`).
- `covcal.for:160-176`: covers are RECOMPUTED from the pools —
  `rilcov = 1 − exp(−Σ cf_i·rilrm_i) + strcov`,
  `inrcov = rigcov = 1 − exp(−Σ cf_i·rigrm_i) + strcov`
  (`strcov = rmagt/srmhav·basmat`, the standing-mat contribution),
  0/0.999 clamps.
- `decomp.for:977` calls `covcal` after evolving the pools (decay +
  litter additions). "No-decomp" forest scenarios hold covers at the
  declared values because the decay parameters are ~0 — the pools, and
  therefore the covers, are constant.
- openWEPP's decomposition surface pool seeds from `sumsrm_seed` (the
  legacy IC's BURIED-residue line) — a different basis than the ground
  pools `covcal` reads; the declared covers currently reach only WB16
  friction and the frost depth seeding.

## Fix design (legacy-faithful, bounded)

1. **Pool seeding (init1 lineage):** the residue-cover authority gains
   per-OFE `interrill_ground_residue_kg_m2` / `rill_ground_residue_kg_m2`
   seeded by the `init1.for` inverse from the projection's declared
   `inrcov`/`rilcov` and the residue plant's `cf` (both already parsed;
   the frost path's `legacy_initial_residue_depth_m` is the in-repo
   precedent for the same inverse).
2. **Pool evolution (decomp lineage):** the decomposition state carries
   the two ground pools; the SAME daily decay factor the surface pool
   uses applies to them directly (`decomp.for` applies its decay to
   `rilrm`/`rigrm` in the identical form), and the surface-litter input
   adds to both (per-area mass falls on rills and interrills alike).
3. **covcal port (pure):** `inrcov = clamp(1 − exp(−cf·rigrm), 0, 0.999)`
   and `rilcov` likewise on `rilrm`. `strcov` (standing-mat term) is 0 —
   the standing pool is not yet modeled (labeled limitation, recorded;
   the term is additive and can only RAISE cover, so its absence is
   conservative in the fail-direction of the original defect).
4. **Partition + erosion wiring:** the residue partition publishes the
   two covers (new fields; the existing composite `cover_fraction`
   becomes the legacy `rescov` area-weighted composite); the erosion
   daily state consumes `inrcov` for the interrill fields and `rilcov`
   for the rill field (today both read the one zero composite).
5. **Small-event day gate (rides along):** adjudicate
   `wave1_day_routes_sediment` against the `contin.for` event gate.

## Acceptance

- p61 instrument rerun: the dominant event's per-width delivery moves
  from ~6× legacy into the legacy order (the E.5 water cut bounds the
  expected residual at ~0.6–1×); recorded in the closure artifact.
- Existing conservation/closure gates unchanged (cover changes operand
  MAGNITUDE, never mass accounting).
- Contracts: SC-RESIDUE-001 amendment (ground pools + covcal covers);
  SC-SED-001 `GAP-SED-009` re-judgment.
- Gates per `docs/standards/local-ci-gate-selection.md`: focused +
  erosion fixtures in-loop; `full` at branch head; Codex review before
  merge.

## Write set

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/decomposition.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs` (if a new symbol export is needed)
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/*`
- erosion fixture tests; SC-RESIDUE-001; SC-SED-001; this package

## Line-count disposition

`00_builders_and_authority.rs` is over the 2,000-line WARN (2,258+ after
this branch's authority additions). The file is the known
seed-authority fan-out (every typed lane authority builds here); this
package adds ~60 lines to the existing shape rather than a new concern.
Disposition: WARN acknowledged; the split rides the next structural
runner refactor (the module is already sectioned by authority family) —
not this defect-closure's scope.
