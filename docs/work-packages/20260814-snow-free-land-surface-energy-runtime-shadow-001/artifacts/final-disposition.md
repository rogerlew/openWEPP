# Final Disposition

Status: `COMPLETE / PASS / default-off shadow`

The package was reopened after commit `96c46c88e01e4faaecccd084e402ebb6dcb1e6cd`
for a bounded implementation correction. Forest-litter conductivity now uses
the immutable beginning hydrology-owned litter store under the canonical
`0.1 + 0.03*W_l/(rho_w*dz_l)` equation instead of aliasing the top-soil
conductivity. Per-tile LSE VIS/NIR ground optics are the sole E01--E03
lower-boundary owner; the single vegetation forcing albedo pair is neither a
heterogeneous-tile restriction nor a covered-ground optics input. Focused
runtime gates and fresh independent science review pass. The separate
oracle-reconciliation package is COMPLETE and closed first. The clean full
workspace gate passes 2,999/2,999, all seven corrected-litter benchmark
surfaces pass with nonzero selectors, and both terminal verifiers return PASS.

The historical custody HOLD below remains immutable evidence of the dependency
that blocked the first execution attempt. Commit `a7d692da4` lifted that
dependency, and the existing Child-3 package resumed. All accepted material
Child-3 Rust and science findings were remediated, the strict default-off public
endpoint and seven required nonzero benchmark surfaces passed, and the final
science review returned PASS.

The exact hold and rejected aliases are documented in
`real-hydrology-surface-liquid-hold-audit.md` and independently confirmed in
`review_real_hydrology_surface_liquid_hold.md`. A bare-mineral-soil-only
release remains prohibited by the campaign objective. Completion here is only
for the strict default-off shadow; no real-consumer claim is made.

The historical V3/V5 regeneration contradiction remains preserved as
provenance evidence, not rewritten or waived. Contract version 13 resolves it
prospectively through content-addressed V9 authority and explicit V8-to-V9
migration. Historical V3--V8 calculator, fixture, definition, and digest bytes
remain immutable. The previously failing full-workspace run remains retained
as historical evidence; subsequent uninterrupted exact-byte execution passes
and is preserved in the reconciliation package's terminal evidence. The
kickoff prompt is archived and the authorized `COMPLETE` disposition is now
recorded.

Production selectors, defaults, execution, state and outputs remain unchanged.
No activation, cutover, publication, calibration or empirical-validation claim
is included.
