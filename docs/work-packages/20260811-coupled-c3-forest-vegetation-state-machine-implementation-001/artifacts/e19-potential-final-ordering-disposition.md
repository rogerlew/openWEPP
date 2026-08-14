# E19 Potential/Final Ordering Disposition

Status: `accepted correction / implementation overconstraint`

Evidence mode: `Static + Ran`

## Preserved observation

The historical audit remains unchanged in
`e19-potential-final-numerical-hold-legitimacy-audit.md`. Its full-water
fixture observed these exact binary64 values:

```text
Coffer_pot   = 0.004530518239130747 kg C m^-2 interval
Coffer_final = 0.004530518239130749 kg C m^-2 interval
Ndem_pot     = 0.00009755546838693540 kg N m^-2 interval
Ndem_final   = 0.00009755546838693542 kg N m^-2 interval
```

The carbon and nitrogen pairs differ by two ULPs. The values and the original
reasoning are retained as historical evidence; they are not normalized.

## Canonical disposition

`SC-VEGETATION-001@11` defines potential requests from `Ndem_pot`, followed by
finalization from the independently reconstructed `Ndem_final`:

```text
Fext = min(Dext_final, Asum)
F_N,l,q = Fext * A_N,l,q / Asum   when Asum > 0
Nused = Nint_use + Fext
eta = 1 for zero demand, otherwise min(1, Nused / Ndem_final)
NSC_C1 = (1 - eta) * Coffer_final
```

The contract contains no invariant requiring `Ndem_final <= Ndem_pot`.
`SC-BIOGEOCHEM-001` independently requires
`0 <= finalized use <= authorization <= request`; it does not order the two
physiological demand calculations.

Therefore `NitrogenProtocolError::FinalDemandExceedsPotential`,
`VegetationError::NitrogenDemandOrdering`, and the pre-arbitration monotonicity
guard were noncanonical implementation overconstraints. They are removed.
Potential requests remain immutable, arbitration occurs exactly once, and the
canonical finalization caps external use at the authorization sum. Any unmet
final physiological demand lowers `eta`, with unsupported carbon retained in
`NSC_C`.

Fresh correctness review found and rejected one remaining aggregate form of
the same overconstraint: binary64 `internal_use + external_use` can round one
ULP above `final_total_demand` even though both source operands were produced
by the canonical bounded equations. The extra aggregate rejection was removed.
An exact adjacent-bit regression now proves that canonical
`eta=min(1,Nused/Ndem_final)` accepts the receipt, yields `eta=1`, and retains
zero unsupported NSC without altering either source use operand.

No comparison tolerance, clamp, request inflation, second request,
reauthorization, or model-version change is used. The V6 rejected-diagnostic
`step_norm` portability tolerance remains isolated from E19 accepted values.

## Bounded claim

The corrected phase composes V7 phenology/turnover and E19 mineral-N
request/authorization/final-use as an uncommitted candidate. It does not claim
a BGC owner debit, energy-owner completion, all-owner atomic commit, runtime
activation, calibration, or public E01--E22 completion.
