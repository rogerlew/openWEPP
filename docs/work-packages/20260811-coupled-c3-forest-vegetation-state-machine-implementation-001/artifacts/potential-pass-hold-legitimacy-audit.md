# Potential-Pass HOLD Legitimacy Audit

Status: `PASS / canonical omissions confirmed before constitutive edit`

Evidence mode: `Static + Ran + two independent code/authority audits`

## Exact blocked mechanisms

Increment 2B cannot truthfully construct the exact V2 potential
occupancy-column solve from the released authority. The implementation stopped
before producing an `OccupancyRadiation` or a potential physiological result.

1. E01 accepts one plant-area coordinate and one `(chi,rho,tau)` pair per
   stratum and band. V2 supplies separate conditional LAI/WAI, leaf/stem
   VIS/NIR optics, and clumping, but no canonical rule defines:

   - how leaf area, stem area, and clumping form E01 plant area;
   - how leaf and stem optical pairs form E01's single `rho/tau`; or
   - how solved absorption partitions back to leaf, stem, sunlit leaf, and
     shaded leaf.

   The historical implementation used combined-area clumping, area-weighted
   optics, and an absorptivity-weighted post-partition. Those are not stated in
   the digest-bound contract and therefore cannot be reinstated as inference.

2. The energy authority requires distinct positive `u`, `u_leaf`, and `u_wet`.
   `SnowFreeForcing` supplies only one reference `wind_m_s`; the executable
   schema does not authorize aliasing it to both local surface winds.

3. E14 requires explicit stem path length `z2` and stem gravity. Configuration
   supplies height and crown base, but no canonical mapping from those fields to
   either hydraulic operand.

4. V2 persistent state owns `root_potential_mm_by_layer[]`. The admitted E14
   kernel solves one common root-node potential. No accepted-state rule maps
   that scalar to the per-layer state vector; copying or broadcasting it is
   explicitly unavailable as a general V2 state rule.

5. The potential-column instruction fixes `beta_hyd=1`, while the inherited
   coupled rule requires gas/energy and hydraulic transpiration equality through
   a solved `beta_hyd`. Leaf vulnerability generally makes the hydraulic result
   differ from the beta-one gas maximum. The potential-stage acceptance
   semantics are not specified precisely enough to choose one residual system.

6. No committed independent V2 E01--E03 mixed leaf/stem column fixture or exact
   potential E11--E15 fixture exists. The V2 topology fixture intentionally uses
   controlled, non-constitutive vapor values. The Python calculator's coupled
   iteration is not the frozen Rust numerical algorithm and cannot silently
   become the missing `STAGE_B_E11_E15_EXACT_ORACLE` fixture.

## In-scope correction routes attempted

- Reuse the exact `radiation::solve_column` matrix-exponential kernel: rejected
  because constructing its layer and partitioning its result requires the
  missing rules above.
- Restore the historical V1 radiation preparation: rejected because its
  mixing/partition formulas are not explicit canonical authority.
- Compose `solve_canopy_energy(beta=1)` followed by `solve_hydraulics`: rejected
  because it is the prohibited disconnected/one-pass endpoint and does not
  resolve the beta/equality or V2 root-state mapping.
- Treat reference wind as all local winds and derive hydraulic length/gravity
  from crown geometry: rejected because both are hidden operand substitutions.
- Broadcast the common root-node solution into every layer lane: rejected
  because V2 requires occupancy/root-layer state identity and forbids silent
  copied numerical lanes during migration; no accepted-update exception exists.

All draft inferred radiation and potential-solver bytes were removed. The
existing public transaction remains fail-closed before E04 and emits no
candidate or request.

## Safe completed work

The non-constitutive typed water boundary is implemented in
`occupancy_solver/resources.rs`. It validates complete occupancy/root-layer key
sets, transaction and owner identity, stand-ground amount basis, finite
nonnegative amounts, duplicate requests/authorizations, exact request-to-
authorization correspondence, and tolerance-bounded final amount comparison.
It does not calculate water demand or make Increment 2B executable.

## First concrete lift action

Authorize a contract-first correction of `SC-VEGETATION-001` and a successor
digest-bound model definition that explicitly freezes the six mechanisms above,
then generate independent mixed leaf/stem whole-column radiation and beta-one
potential E11--E15 vectors using the selected algorithms. Resume this same
implementation package only after that authority release. The authority work
must also adjudicate the existing `Rd25` temperature-response mismatch and
failure-diagnostic payload requirements exposed by the audit.
