# Disposition

Status: `closure candidate — terminal verification pending`

Defects `PEAK-HOURLY-001`, `PEAK-RETURN-002`, and `PEAK-UNITS-003` are closed.
The production path consumes the closing 24-bin post-partition hourly runoff
ledger, preserves modeled-hour surface return, stores the maximum hourly mean
as `m/s`, and applies hillslope area exactly once when publishing `m3/s`.
Positive runoff without hourly custody fails closed.

Closure binds implementation/contract/test commit
`33831787b7029b28b0716c8458f08a11899db446`, release binary SHA-256
`ac8790faf32a5b98993427b636084c04ba468955458c4fc18f3874cea709c4c3`,
and frozen plan SHA-256
`32e6f5e99a77747fcdd93388302f2a5ffb496a87b764ac4505e09691955db756`.
The complete Topanga cohort ran 280 baselines and all 1,088 mutations with no
unexplained volume-stable peak discontinuity. The exact-head Critical workspace
gate passed 2,346/2,346 tests, and all four implementation reviewers returned
PASS with no open findings. Dual terminal verification remains before final
closure.

The admitted claim is maximum hourly mean hillslope runoff flow. This package
does not claim instantaneous/subhourly peak flow, legacy numerical parity,
calibration, empirical accuracy, or routed watershed/channel flow.
