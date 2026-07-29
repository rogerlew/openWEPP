# Prospective Scientific Re-review — Agent B

Evidence class: `Static: final amended scaffold and source inspection; no
result-bearing execution`

Disposition: `PASS`

## Finding closure

| Finding | State | Evidence |
| --- | --- | --- |
| B-01 | `CORRECTED` | Runner projection, CAL-03 native source/consumer evidence, package-local direct-runtime phases, and downstream publication are separate claims. The synthetic producer uses `DirectDayFrame::run_r5c_decomposition_phase` and residue partition rather than calling the calculation kernel alone. |
| B-02 | `CORRECTED` | The complete decomposition recurrence, chronology, carry, context, and constants are frozen. All five residue-partition inputs are now explicitly frozen at zero and labeled `ASSUMED_FOR_EXECUTION`. |
| B-03 | `CORRECTED` | The five-pair analytic terminal ridge is distinct from daily recovery. Independent arithmetic inspection placed every pair within about `1.1e-15 kg m^-2` of the frozen target, inside the prospective `1e-12` tolerance. This is a design check, not CAL-05 result evidence. |
| B-04 | `CORRECTED` | The experiment is consistently a finite-horizon year-20 day-365 post-decay terminal-stock experiment. No equilibrium choice or claim remains in Included Scope. |
| B-05 | `CORRECTED` | The failure-and-boundary table freezes each mutation, value class, expected typed variant, and exact expected field; valid zero cases are distinguished from invalid cases. |
| B-06 | `CORRECTED` | Temperature is correctly identified as the limiting surface-decay modifier. Precipitation saturation is restricted to the separately reported standing-water factor. |
| B-07 | `CORRECTED` | The native-source lane retains all frozen CAL-04B accepted and later-stage identities without selecting a preferred member, keeps dry mass separate from Harvard carbon, and returns `NOT_ASSESSED` if complete member traces cannot be authenticated. |
| B-08 | `CORRECTED` | Harvard periods, units, material boundaries, and validity guards are retained. Needle and fine-woody stages now use the exact ADR-0042 tuple: `AUTHORITY_MISSING`, `NOT_CALIBRATION_READY`, and `NOT_ASSESSED`; the successor remains contract-first and cannot invent missing source physics. |

## Conclusion

All B-01 through B-08 prospective findings are corrected. The design now
supports result-bearing execution within its declared synthetic-readiness,
descriptive-observation, and data-limited claim boundaries. This pass does not
predict execution results or preassign terminal ADR-0042 statuses.

## Incident 001 amendment review

Evidence class: `Ran: independent read-only arithmetic inspection; no CAL-05
rerun`

Amendment disposition: `PASS`

Independent inspection of all 28 retained Harvard plot rows confirms:

- 24 rows exceed the original `1e-9 g C m^-2 yr^-1` serialized-mean closure
  tolerance;
- the maximum absolute residual is
  `1.6166666227945825e-8 g C m^-2 yr^-1` at plot `B1`;
- the amended `1e-7 g C m^-2 yr^-1` tolerance is exactly 100 times the
  original tolerance and approximately `6.185567` times the observed maximum;
  and
- every retained row is inside the amended tolerance.

The first attempt failed closed before writing
`harvard-source-stock-diagnostics.csv`; Incident 001 retains that failure and
does not classify it as a scientific result. The amendment changes only the
floating serialization guard for the already retained means. It does not
change a source value, unit, plot, material class, period, admissibility flag,
observation transformation, objective, model comparison, or interpretation.
The descriptive, noncontemporaneous, pooled-material scientific boundary is
unchanged. Reviewer B approves the amendment prospectively before rerun.
