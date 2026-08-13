# E19 Potential/Final Numerical Ordering Hold Audit

Status: `HOLD / exact comparison authority missing`

Evidence mode: `Static + Ran`

## Blocked Boundary

`SC-VEGETATION-001` defines potential mineral-N demand from
`Coffer_pot`, final demand from the water-finalized `Coffer_final`, and the
typed protocol requires final demand not to exceed potential demand. The
contract and numerical companion provide no normalization or comparison rule
for a capped solve that is physically unchanged but differs from the potential
solve by binary64 iteration noise.

The first production composition of the sealed potential and capped carbon
operands, under bit-identical full water authorization, produced:

```text
Coffer_pot   = 0.004530518239130747 kg C m^-2 interval
Coffer_final = 0.004530518239130749 kg C m^-2 interval
Ndem_pot     = 0.00009755546838693540 kg N m^-2 interval
Ndem_final   = 0.00009755546838693542 kg N m^-2 interval
```

The first composition attempt reached the internal arbiter with one batch of
two NH4/NO3 requests before the typed boundary rejected final demand. Review
correctly rejected that sequencing. Remediation now precomputes and validates
all strata's final-versus-potential demand before the one global arbitration
call. The same case rejects as `final mineral-nitrogen demand exceeds potential
demand` with zero arbiter calls and zero requests published. No phase result,
finalized use, persistent state candidate, or public candidate is returned,
and the compared beginning vegetation bytes remain identical.

## Authority Conflict

The difference is numerical, but the permitted response is not inferable.
Available routes would change canonical behavior:

- applying a new E18/E19 comparison tolerance;
- clamping final demand or final carbon offer to the potential value;
- inflating potential demand with final demand;
- reusing potential gas/carbon results in a required from-beginning capped
  re-solve;
- accepting `F_N` against a request smaller than final demand.

The V6 portability amendment explicitly confines its tolerance to one rejected
`step_norm` evidence field and prohibits spreading it to accepted values,
residuals, conservation, authorization, or finalized-use bounds. Therefore the
implementation cannot reuse that tolerance or invent another.

## In-Scope Routes Attempted

The implementation added a pass-typed carbon aggregation core and separate
sealed potential/final accessors, corrected evergreen phenology validation so
no deciduous operands are synthesized, and composed an internal all-strata
nitrogen phase that preserves exact layer/species/owner identity and performs
one arbitration call only after every stratum passes preflight. The real fully
supplied fixture exposed the ordering failure above. The strict pre-arbitration
rejection and immutable vegetation-input behavior pass.

## Disposition And Lift Action

Only E19 request/finalization composition is held. V7 storage-transfer,
phenology, turnover, accepted water, and standalone typed nitrogen mechanics
remain valid. The public transaction remains fail-closed after the water
phase.

The first lift action is a contract-first decision that freezes one exact rule
for potential/final E18/E19 ordering under independently accepted nonlinear
solutions, with positive, cap-active, equality, just-inside, just-outside, and
poison vectors. Any tolerance must have explicit units, threshold, scope, and
non-propagation firewalls. A successor model identity is required if accepted
values or request construction change.
