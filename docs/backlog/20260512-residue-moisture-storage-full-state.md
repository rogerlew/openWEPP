# Full Residue-Moisture Storage State (Concept Backlog)

## Status
- `state`: backlog
- `maturity`: concept / planning only
- `default_path`: not eligible
- `date`: 2026-05-12

## Why this exists
Legacy WEPP-style hydrology paths model residue interception as a scalar loss
term but do not carry a full residue-layer moisture state through time. This
limits process realism for:
- event-to-event wetting memory in residue;
- evaporation partition between canopy, residue, and soil;
- explicit residue drainage/release timing into soil/runoff pathways.

## Scope
Define an optional residue-hydrology kernel family that introduces an explicit
residue-layer moisture storage state and conservative flux accounting.

The implementation target is:
- physically explicit residue storage (`S_res`) with bounded capacity;
- timestep-to-timestep memory with wetting, evaporation, and drainage fluxes;
- compatibility with existing overland-flow/routing orchestration surfaces.

## Non-goals
- No immediate default replacement for baseline production kernels.
- No ad hoc tuning constants without provenance and contract authority.
- No implicit/clamped "magic recovery" behavior that masks conservation error.

## Governing constraints (openWEPP policy)
- openWEPP default kernels are governed by openWEPP science contracts.
- This is a new physics surface and must remain experimental until:
  - top-down science-contract authority exists, and
  - parity/validation gates are accepted.
- No ABI break for baseline hillslope/watershed CLIs.

## Proposed state and flux surfaces
Per overland element and timestep:
- `S_res` : residue-layer water storage depth-equivalent `[m]`
- `S_res_max` : residue storage capacity `[m]`
- `I_res` : incoming water available to residue interception `[m/s]`
- `E_res` : evaporation from residue storage `[m/s]`
- `D_res` : drainage/drip from residue to soil surface `[m/s]`
- `X_res` : overflow/bypass when storage is full `[m/s]`

State update:
- `dS_res/dt = I_res - E_res - D_res - X_res`
- `0 <= S_res <= S_res_max`

Water available to infiltration/runoff partition then uses:
- `I_net = precipitation + snowmelt + runon - canopy_terms - residue_terms`
- where residue_terms are represented explicitly via `I_res`, `D_res`, `X_res`
  rather than only a static interception loss scalar.

## Capacity parameterization candidates
Phase-1 should support conservative, bounded forms only:
- `S_res_max = f(residue_mass, residue_cover, residue_architecture)`
- keep form minimal and monotone with respect to residue mass/cover.

Any new constitutive constants must have citation-backed provenance before
promotion from backlog to active implementation.

## Integration shape
1. Add residue-moisture state to hillslope process state model (experimental).
2. Replace scalar residue-interception shortcut in experimental path with
   explicit state update + flux accounting.
3. Preserve baseline-mode outputs/contracts unchanged.
4. Emit optional diagnostic traces for residue storage closure:
   `S_res_prev`, `S_res_curr`, `I_res`, `E_res`, `D_res`, `X_res`, residual.

## Numerical plan (phased)
Phase A: isolated core
- implement standalone residue-storage update routine;
- enforce hard bounds with explicit status signaling, not silent coercion;
- verify local mass closure under synthetic forcing sequences.

Phase B: hillslope coupling
- couple residue update before infiltration/runoff partition;
- preserve process ordering deterministically across timesteps;
- validate no regression in baseline mode.

Phase C: orchestration and reporting
- expose optional residue diagnostics in experimental outputs;
- verify publication-routing integrity for new diagnostics;
- evaluate if existing pass surfaces need optional extension.

## Acceptance criteria to promote from backlog
1. Local closure:
   - `|dS_res - (I_res - E_res - D_res - X_res)*dt| <= tolerance`.
2. Run closure:
   - no unexplained water creation/loss attributable to residue state.
3. Baseline safety:
   - baseline mode byte/semantic behavior unchanged.
4. Boundedness:
   - no negative storage; no storage above capacity.
5. Determinism:
   - repeated runs under fixed config produce bit-identical residue diagnostics.
6. Governance:
   - contract/provenance path documented.

## Key risks
- identifiability/confounding between residue storage and soil infiltration
  parameters;
- overfitting via unconstrained empirical capacity curves;
- hidden mass leakage if residue fluxes are not included in closure accounting;
- output-surface drift if diagnostics are added without contract pinning.

## Open questions
1. Should `S_res_max` be mass-based only, or mass + cover + architecture?
2. Should residue drainage be instantaneous, thresholded, or time-lagged?
3. How should snow-covered residue behavior be represented in the explicit state
   path?
4. Which experimental outputs are mandatory for operator-grade diagnostics?
5. What minimum comparator dataset is acceptable before contract promotion?
