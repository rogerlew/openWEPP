# CAL-07C Prospective Review A

Evidence class: `Static`

Reviewer: `prospective-review-a`

Scope reviewed: `package.md` as amended before result execution,
`artifacts/pre-execution-source-inventory.md`, CAL-07 final disposition,
CAL-07B package/science summary, `SC-PLANT-001` OBL-PLANT-P-013 and GSI
VPD guards, ADR-0028, ADR-0042, and applicable agent/work-package/science
instructions.

## Review question

May CAL-07C proceed to result-bearing execution as a bounded, package-local
hourly-product VPD forcing reconstruction without production Rust or
canonical science-contract edits?

## Finding summary

| Severity | Finding | Disposition |
| --- | --- | --- |
| High | None. | No execution-blocking source-authority, operator, clipping, or production-scope defect found. |
| Medium | Authority-boundary wording still overstates CAL-07B unless read narrowly. | Accepted as a claim-calibration fix required before final disposition, not a blocker to result-bearing execution. |
| Low | Negative hourly paired-product rows must remain visually and tabularly prominent in every result claim. | Accepted as a validation/terminal-review watch item. |

## Accepted rationale

1. `SC-PLANT-001` requires GSI failures for negative or non-finite daily VPD
   and OBL-PLANT-P-013 derives production daily VPD from daily Tmax/Tmin and
   dew point with no clamp or bounded negative normalization. CAL-07C does not
   amend that obligation, loosen production guards, or mutate production
   forcing derivation.
2. CAL-07C instead proposes a package-local executor path that supplies the
   already typed daily `GsiDailyForcing::vapor_pressure_deficit_pa` operand.
   That is acceptable only as bounded research execution because the package
   excludes production readiness and declares any production replacement to
   require a separate contract-first package.
3. The amended hourly-negative boundary is legitimate for this package-local
   execution. The forcing operator admits the arithmetic daily mean of signed
   hourly paired-product VPD values. The 349 negative hourly components
   reported in `pre-execution-source-inventory.md` are not clipped, deleted,
   normalized, or hidden; they remain operands in the mean and become a claim
   ceiling. The admitted daily operand is the only value consumed by the GSI
   kernel, and the inventory reports all 1,666 admitted daily means as finite
   and nonnegative.
4. The package preserves source custody and inventory gates: fixed POWER URL,
   LST hourly keys, 39,984 `T2M` and 39,984 `T2MDEW` rows, 1,666 complete
   days, no daily VPD negatives, and daily Tmin/Tmax/mean-dewpoint
   compatibility against frozen CAL-07 daily operands within the stated
   serialized-resolution tolerance.
5. ADR-0042 supports claim separation: source/data limitations constrain
   empirical and transferability claims, but do not by themselves forbid a
   bounded execution when the daily typed operand remains in-domain and the
   limitations are disclosed. ADR-0028 does not authorize a production
   mechanism here because no contract-first production admission is attempted;
   CAL-07C correctly leaves that to a future package.

## Rejected rationale

- Rejected: `HOLD` solely because some hourly paired-product VPD components are
  negative. That would be required if the package admitted hourly values as
  valid subdaily atmospheric states or clipped them to create the daily value.
  The amended protocol does neither; it admits only the signed daily arithmetic
  mean and makes the negative components visible.
- Rejected: treating CAL-07C as a replacement or waiver of OBL-PLANT-P-013.
  The package explicitly excludes canonical contract, fixture, test, and
  production Rust edits, and it cannot claim production readiness from this
  evidence.
- Rejected: Order 7 advancement from the forcing fix alone. CAL-07 and CAL-07C
  both retain non-forcing evidence ceilings for provisional camera greenness,
  POWER grid representativeness, absolute canopy amplitude, evergreen-floor
  quantification, and downstream litter/decomposition consequences.

## Required claim-calibration follow-up

Before final disposition, revise or qualify the package statement that
“CAL-07B proved that ... paired hourly POWER products are positive.” CAL-07B
proved that for the three CAL-07 negative daily blocker dates. The full-period
CAL-07C inventory now reports 349 negative hourly paired-product rows, so any
unqualified full-period positivity claim would be false.

Terminal review should also verify that all result tables, figures, sidecars,
roadmap/catalog updates, and final verdicts state:

- negative hourly rows were retained signed and not clipped;
- the admitted Alerce operand is only the finite nonnegative daily mean;
- POWER grid forcing is not on-site meteorology;
- CAL-07C is not production-ready and does not amend OBL-PLANT-P-013; and
- Order 7 advances only if every non-forcing CAL-07/CAL-07C gate also passes
  with its evidence ceiling visible.

## Final disposition

`GO` for bounded CAL-07C result-bearing execution under the amended daily
operand boundary.

This GO does not authorize production Rust edits, canonical contract edits,
clipping/canonicalization, CAL-07 or CAL-07B artifact mutation, or production
readiness/Order 7 claims beyond the evidence actually regenerated in CAL-07C.
