# HOLD Legitimacy Audit

Status: `PASS / canonical omission confirmed`

Evidence mode: `Static + Ran + independent review`

## Exact blocked mechanism

`OPENWEPP_C3_WOODY_V1` E04 cannot be implemented for a stratum occupying a
proper subset of the stand or multiple topology tiles with heterogeneous
upstream columns. SC-VEGETATION-001 defines `C_s` and permits multi-tile
membership (lines 214--220), defines `LAI_s`, `WAI_s`, `S_liq,s`, and
`P_liq,s` on the horizontal stand/OFE basis (lines 125--143), and freezes the
nonlinear interception/store sequence (lines 315--330). It does not define:

- how one stand-ground `S_liq,s` is distributed among occupied tiles;
- whether E04 is evaluated before or after tile aggregation;
- how tilewise nonlinear store/drainage results return to one persistent
  stratum store; or
- how releases are disaggregated to differently structured descendant columns.

The independent science adjudication is preserved in
`review_agent_b_remediation_repeat.md` under “Targeted authority adjudication.”

## In-scope correction routes attempted

Radiation was corrected using the explicit E03 rule: derive tile-local optical
area from stand-ground area and `C_s`, solve the exact column boundary problem,
then aggregate with `f_t`. That route is authorized because radiation has no
shared nonlinear persistent store.

For E04, aggregate-first evaluation loses descendant-column routing and changes
the nonlinear `tanh` and capacity branches. Tile-first evaluation requires
inventing a distribution of the one persistent store. Replicating the store or
rejecting the admitted topology as a scientific domain restriction would also
be constitutive changes. None is authorized by the read-only contract or
digest-bound model definition.

The public transaction now fails closed before calculation unless every
stratum has exactly one full-cover tile. This containment prevents publication
of an invented candidate; it is not claimed as the final model domain.

## Other unresolved review work

The repeat Rust review also identified unresolved numerical diagnostic,
independent-owner reconstruction, atomic-commit API, arbitration-identity,
multirank liquid handoff, and line-count findings. These remain accepted
implementation work after the authority blocker is lifted; they are not the
reason for HOLD.

## First concrete lift action

Amend SC-VEGETATION-001 and the digest-bound model definition to select either
tile-resolved canopy-liquid state or an exact aggregation/distribution rule
covering store, interception, condensation, drainage, and descendant release.
Add independent heterogeneous-column oracle vectors. Then resume this same
package, remove the temporary fail-closed containment, complete the accepted
review findings, and run the deferred heavy and terminal gates.
