# Worker Handoff

Evidence class: Static.

Next recommended package: `SNOWDENSITY-06 Density Compaction`.

Carry forward:

- Use 05G representative-regime evidence, not the 05E regime-limited
  promotion-candidate result.
- `legacy_coe` remains the default/rollback melt boundary.
- `coe_shortwave_albedo_v1` may be used only as an opt-in diagnostic boundary
  unless a later activation package reopens melt.
- Do not retune melt coefficients, albedo constants, or shared radiation to
  improve density signatures.
- Density work should target Anderson-1976/SNOBAL-style overburden and
  metamorphism compaction in the `physics_bulk` path.

First SNOWDENSITY-06 action:

- Add the density compaction contract amendment and focused tests before
  production or diagnostic density edits.

Melt follow-up fork:

- Do not retire `coe_shortwave_albedo_v1` based on coniferous neutrality. Hold
  it opt-in pending a low-canopy/mixed-forest adjudication package.
- The mixed-forest package must use canopy-stratified fixtures such as
  Marcell/Harvard and must prove the open-vs-deciduous-vs-conifer regime.
- Before any mixed-forest melt verdict, replace the single representative
  canopy value with a real per-day seasonal `cancov` series. The coniferous
  `0.9` scalar was acceptable for 05G evergreen replay; it is not acceptable for
  leaf-off deciduous or mixed-forest trajectories.
- Keep the PySnobal arm as an ADR-0017 diagnostic flag profile unless/until its
  direct `net_solar` consumption is separately normalized.
- Leave Brock-2000 constant recheck against `references/copyrighted/brock2000.pdf`
  as an albedo-focused follow-up.
