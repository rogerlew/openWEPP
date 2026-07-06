# Kernel Profile Compliance (D10B)

Status: executed
Evidence mode: Static (against `kernel-process-contract-profile.md`) + Ran (tests)

- Canonical `SC-*` authority: SC-OFEROUTE-001 rev 24/25 precede all
  production edits (pre-implementation gate recorded).
- Typed guards preserved: all fail-closed paths unchanged
  (`InvalidForcing`, `InvalidCellParameter`, `CflViolation`,
  `NonFiniteState`, `NegativeDepth`, `DegenerateConfiguration`); new
  Manning operand validated in `CellParameters::validate`; upstream
  integral closure validated via `is_valid_forcing`. No guard loosened.
- No surrogate physics: every numerical form traces to a named primary
  (Davis/Mingham/Tseng/Iwagaki) or the conservation invariant; the
  Manning limb is the definitional friction identity; no tuned constants
  (the `k_o` scan class remains rejected).
- Provenance: REF rows updated with primary-in-hand citations + page/eq
  anchors; evidence tags `[DIRECT][Ran]` where executed.
- Symbol continuity: `phi`, `Gr/G`, `Cf`, `Cr`, `alpha`, `n` follow
  R-63/Davis/Mingham symbols; new fields carry unit-suffixed names
  (`_m2`, `_m2_s`, `_dt_s`).
- Invariant guard map: INV-005/006/007/011 enforcement paths updated to
  the D10B test/harness surfaces.
- Shadow-first posture: the subsystem remains unwired from production
  phase spans (`INV-OFEROUTE-010`); no activation semantics changed.
