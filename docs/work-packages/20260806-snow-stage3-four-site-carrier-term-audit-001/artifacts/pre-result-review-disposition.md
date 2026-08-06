# Pre-Result Review Disposition

Status: `CLOSED / all findings dispositioned / v2 PASS-PASS admitted`

Evidence mode: `Static` plus package-local unit tests. No model result has run.

| Finding | Disposition | Amendment |
| --- | --- | --- |
| Complete latent was substituted for surface latent | Accepted | Surface latent is a separate daily operand; the frozen disabled selector requires exact zero. The paired delta includes complete latent minus surface latent, and the asymmetric test fixture proves the distinction. |
| Self-referential execution SHA and stale binary | Accepted after second review | The caller supplies the full independently admitted SHA out of band. Execution requires exact clean HEAD before build, before analysis, and before acceptance; it builds and retains that SHA's release CLI and rechecks its binary hash after all runs. |
| Same-state output called a seasonal trajectory | Accepted | Claims and output names now describe water-year-stratified independent condition samples. Seasonal integration, chronology, melt, and persistence claims are prohibited. |
| Coverage could pass on one hour | Accepted | Screen eligibility requires 30 evaluated days, 0.25 calendar coverage, and 10 eligible samples per site. Lower-support samples remain descriptive and explicitly excluded. |
| Roth values and ratio semantics were wrong | Accepted | Final-paper values are 93%, 92%, and 47%. The audit declares the literature partition ratio `NOT_COMPARABLE` and computes no clipped ratio. |
| Five-term subset was called complete energy | Accepted | The quantity is the implemented external subset. Marks' total is `NOT_COMPARABLE` because snow-ground heat is absent; per-term context remains descriptive. |
| Development-only Snowbird could decide the verdict | Accepted | Only Mica Creek, Niwot, and Paradise decide the prospective screen. Snowbird remains a required fourth replay and non-decisive diagnostic. |
| Trace identity/support/applicability checks were incomplete | Accepted | The parser checks day/lane sequence, exact tags, 16-hex nonzero fingerprints, full-day-or-zero support, strict booleans, inactive zeros, applicability truth table, support reconciliation, and v4 inactive-snow reasons. Negative tests cover the critical failure modes. |
| Observation years could disappear | Accepted | Chronology, water-year labels, earliest peak ties, missing/zero years, out-of-trace years, censoring, analysis, and screen exclusion receive explicit census dispositions. |
| Runfile consumer and publication custody were incomplete | Accepted | Every generated TOML runfile is parsed to prove the exact staged climate path/hash and exact `pass/loss/wat` publication keys. HBP is asserted as `outputs.pass`; expected output cardinality and control/paired WAT/HBP bytes are checked. |
| Retained verification trusted producer receipts | Accepted | A complete retained-artifact manifest is hashed. Verification reparses runfiles and traces, rechecks sources/observations, reconstructs all sample/site results, and directly rechecks WAT/HBP byte identity. |
| Derived output still used `complete_*` for the no-ground subset | Accepted after second review | Every derived JSON/CSV/site/screen key uses `implemented_external_subset`; only canonical producer schema field names retain `complete_arm`. |
| Mandatory v5 auxiliary operands were unread | Accepted after second review | Hourly/daily vapor, complete energy, cold-content, cold-required/change, excess, available ice, sublimation, melt, terminal energy, internal exchange, closure residual, and maximum residual fields are finite, reconciled, or exact zero/N/A as required. Negative tests reject contradictory complete energy and nonzero N/A residuals. |
| Runfile PASS custody was recorded, not asserted | Accepted after second review | Parsed `outputs.pass/loss/wat` paths must exactly equal the retained HBP/loss/WAT paths; a mismatched path fails before execution. |
| Marks collapsed ranges were not reproducible/comparable | Accepted after second review | Collapsed ranges and all numerical Marks classifications were removed. Marks remains qualitative context with an explicit different-site/period/estimand/boundary `NOT_COMPARABLE` result. |
| V1 execution validator treated the applicable component residual as exact-zero N/A | Accepted after rejected v1 attempt | The v1 namespace is retained and rejected before metrics. V2 applies the already-frozen `1e-6 J m^-2` daily tolerance only to this applicable producer residual; all true same-state N/A fields remain exact zero. No science rule or threshold changed. |

Fresh result-blind v2 reviews passed on exact clean commit
`3ee1bac3ee849fbe00b517d1d227140f87fedc2a` before the v2 namespace was
created.
