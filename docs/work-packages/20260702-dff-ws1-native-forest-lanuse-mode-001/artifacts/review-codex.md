# Codex Review — WS-1 Increment-1 Design (2026-07-02)

Evidence: Static source/doc review + doc gates. Verdict: **design revisions
needed before Increment 2** (ADR-0034 authority model sound; no kernel rewrite
implied). All five findings **accepted and dispositioned** into the design.

| # | Sev | Finding | Disposition |
|---|---|---|---|
| 1 | High | Landuse-agnostic seam overstated: kernel is symbol-oriented, but the projection path is cropland-specific — yearly projection rejects `landuse!=1` + destructures cropland (`01_management.rs:635`), growth projection accepts only `PlantCroplandData` (`05_projection_helpers.rs:158`), initial seeding pulls cropland data (`:259`). | Reworded package §"Integration seam" + schema §3 to "kernel symbol-compatible, NOT whole-seam agnostic"; Increment-2 scope widened to add the forest **projection path** (Plant + Initial + yearly). |
| 2 | High | Tier-A "forest defaults" not implementable as active physics — the growth symbols drive canopy/LAI/roots/ET/interception/runoff; unnamed defaults = masquerade rebranded. | Added Tier-A **physics-authority requirement**: name a forest authority, require explicit values, or a labeled default-off/placeholder mode with manifest warning + no fidelity claim; no rangeland numeric defaults. Elevated from "open item" to requirement. |
| 3 | Med | Carve under-specified: parser rejects non-cropland across ALL sections (plant/op/initial/surface/contour/drain/yearly), not just Plant/Initial; soil precedent transfers as a *gated pattern*, not exact shape. | Added §1 **all-section forest policy** (supported Plant/Initial/yearly + no-op op/surface/contour/drain) + blank-slot handling + per-section tests to Increment 2. |
| 4 | Med | Soil/mgmt split right, but reconciliation only "may" arrive; `DisturbedPolicy.luse` is free text. | Made it a **requirement**: Increment-2 reconciliation manifest tying `.man` class ↔ lookup ↔ `openwepp-disturbed.json` ↔ `.sol` policy, **fail-closed on mismatch**. |
| 5 | Low | Wording leaks rangeland authority ("rangeland-derived"). | Changed to "rangeland-**shaped** structural reference" in package + contract pointer; rangeland numeric defaults prohibited unless separately authorized. |
| — | Low | Contract pointer appropriate if non-normative. | No revert; pointer kept as "WS-1 design (in progress)", not ratification. |

Gates (Codex): `git diff --check` pass; markdown-doc lint/validate pass (5 files, 0 err/warn). No Rust gates (design increment).

Post-disposition state: the design no longer overclaims the seam and no longer
permits running forest mode on unnamed defaults; Increment-2 scope is widened to
the projection path + all sections + reconciliation manifest. Ready for a Codex
re-check or Increment-2 handoff.

## Re-check disposition (Codex, 2026-07-02)

Narrow residual + a wording nit, both fixed:
- **Residual (keeps High-2 partially open):** the Tier-A table's Source column
  still listed "forest defaults" / "rangeland `bbb`" / "rangeland `hmax`",
  contradicting the requirement below it (an implementer could read the table as
  authority). **Fixed:** Source column now reads `lookup` (authoritative) or
  `forest authority †` (resolve per the Tier-A requirement — never unnamed
  cropland/rangeland defaults), with a `†` footnote + a column-legend preamble.
- **Low nit:** package §carve said the soil parser implements the "exact carve
  pattern." **Fixed:** reworded to a "gated native-extension pattern, not an
  exact parser shape."

Post-fix: no Source-column entry asserts a default as authority; the High-2
disposition is fully closed. Ready for Increment-2 handoff.
