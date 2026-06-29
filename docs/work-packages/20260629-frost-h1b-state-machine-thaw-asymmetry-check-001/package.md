# FROST H1b State-Machine Thaw-Asymmetry Check

Status: `EXECUTED-COMPLETE-DIAGNOSTIC-NARROW-EDGE`

Date: 2026-06-29

## Objective

Determine whether the `H1b` thaw-late cells from the post-residue Sleepers
diagnostic indicate a structural top-down thaw gap in the FDHP01 fine-layer
freeze/thaw state machine, or a narrow bounded residual that should not block
`GAP-SNOWFREEZE-002` ratification.

## Scope

Included:

- Static code reading of the fine-layer top/front/bottom thaw state machine.
- Per-cell trace analysis for the two prior `H1b` cells:
  `site2_sleepers_w9_hardwood_vt:1995:thaw` and
  `site2_sleepers_w9_hardwood_vt:2010:thaw`.
- Full Sleepers generalization scan using the existing post-residue R7G frost
  traces.
- `GAP-SNOWFREEZE-002` disposition update.

Excluded:

- Frost solver or state-machine changes.
- Contract, default, fixture, schema, Qwet, or detector-threshold changes.
- Ratification of `INV-SNOWFREEZE-047/048/050`.

## Required Reading

- `docs/work-packages/20260629-frost-thaw-residual-diagnostic-001/`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md` section 11

## Evidence Plan

1. Read the top-front branch selection and branch-3 top-thaw functions.
2. Reuse the existing post-residue Sleepers traces and WAT outputs under
   `target/frost_step3_residue_parameterization/runs/`.
3. Scan `frdp` and `thdp` separately:
   - `frdp` is the bottom extent of modeled frozen soil.
   - `thdp` is the thawed cap above a frozen segment.
4. Report whether warm-surface, material-frost days lack top-front retreat or only
   lack bottom-extent (`frdp`) retreat.

## Exit Criteria

- Code path is classified as `STRUCTURALLY ABSENT`, `CONDITIONALLY BLOCKED`, or
  `PRESENT`.
- Each `H1b` cell has a trace-backed blocking-term disposition.
- Full Sleepers prevalence count is recorded.
- Final routing is one of:
  - `STRUCTURAL-GAP`: a contract-gated state-machine fix must precede ratification.
  - `NARROW-EDGE`: top retreat works generally; document the bounded residual and
    proceed to ratification.
- Documentation validation passes for the touched docs.

## Validation

- `python -m py_compile` for the package analyzer.
- Analyzer execution against existing trace/WAT artifacts.
- JSON shape validation for the emitted payload.
- Scoped `markdown-doc lint` and `markdown-doc validate`.
- `git diff --check`.

## Disposition

Executed complete as diagnostic-only. The static code read classifies the
top-down thaw path as `PRESENT`: branch 3 selects positive surface thaw over
material frost, calls `thaw_fine_top_with_resistance_feedback`, and reduces
surface fine-layer frozen depth/ice from the top downward.

The prior `H1b` cells are not structural top-thaw failures. In both W9 1995 and
W9 2010, all no-`frdp`-retreat warm/material days in the observed-to-modeled
thaw window show `thdp` growth, meaning branch 3 created a surface-thawed cap
while `frdp` stayed fixed as the bottom extent of the frozen domain. The named
blocking term is therefore metric semantics, not a branch guard: `frdp` is the
bottom frozen extent, while `thdp` records top-front retreat.

Full Sleepers prevalence scan: `570` branch-3 warm/material days, `497` with
next-day `frdp` retreat, `58` with no `frdp` retreat but `thdp` advance, and
`15` (`0.026`) with neither `frdp` retreat nor `thdp` advance. The route is
`NARROW-EDGE`; proceed to ratification with this bounded residual documented.
`GAP-SNOWFREEZE-002` remains open for the snow-persistence uncertainty and the
snow-free wet-heat/Qwet subset, not for an H1b structural state-machine gap.
