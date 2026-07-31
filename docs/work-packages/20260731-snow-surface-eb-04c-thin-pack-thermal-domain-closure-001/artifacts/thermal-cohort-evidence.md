# Thermal Cohort Evidence

Evidence class: `Ran`

The package-local replay executed the exact 22 thermal targets frozen by
EB-04B with the replay-captured corrected debug runner binary. Later validation
rebuilt `target/debug/openwepp-cli-hill`; the replay report binds the executed
binary's unchanged pre/post-run hash rather than claiming today's target binary
retains that hash.

EB-04A's `failure_day_index` is the one-based last completed trace day; the
typed rejection occurs while processing the following day. Replay therefore
names both explicitly and requires the authoritative suspension/collapse branch
within that two-day source boundary window. This admits an earlier correcting
branch that prevents the rejected state from forming without calling it the
rejection day.

| Check | Result |
| --- | --- |
| Exact targets attempted | 22/22 |
| Passed formerly rejected processing day | 22/22 |
| Authoritative suspension/collapse branch observed in source boundary window | 22/22 — 6 suspension, 16 lower collapse |
| Forbidden absolute-zero, vapor-underflow, closure, or missing-layer error | 0/22 |
| Complete trajectories | 20/22 |
| Suspended duration on source branch day | 32,160 to 86,400 s |
| Minimum unresolved total mass | 0.0015508698 to 0.9998420563 kg m^-2 |
| Lower-collapse duration on source branch day | 1,800 to 86,400 s |
| Minimum collapsed lower mass | 0.0010844922 to 0.9997003261 kg m^-2 |

Two nonzero terminal results remain after passing their former thermal
rejections: `harvard_open/LS` continues to day 3,715 and `marcell_open/S` to day
14,678 before reaching new corrected-trajectory occurrences of the
`prior_layers.thickness_m` geometry mechanism already admitted to EB-04D.
Those later states are not claimed to be the same captured EB-04B geometry
targets and do not weaken the 22/22 thermal closure.

Machine-readable case evidence, trace hashes, input-manifest hash, binary hash,
and terminal classifications are in `thermal-cohort-replay.json`.

The replay tool takes a kernel-managed exclusive lock on
`target/snow_surface_eb04c_replay.lock`,
publishes the report atomically, and verifies that its binary, crates/tests
diff, and own source identity are unchanged from launch through completion. If
the owning process exits, the kernel releases the lock automatically. Preserve
interrupted output separately before retrying; interrupted attempts are not
evidence.
