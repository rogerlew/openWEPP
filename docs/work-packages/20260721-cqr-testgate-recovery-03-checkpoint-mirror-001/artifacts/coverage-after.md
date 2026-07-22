# Coverage After

Ran: the authoritative corrected clean HEAD `d5af6207` used one scoped library
traversal: 120 discovered, 119 passed, zero failed, and the isolated child
harness was intentionally ignored except when invoked by the characterization
driver. Command wall was 509.21 seconds.

Production lines are 1-174. Coverage is 127/131 lines (96.95%) and 204/229
regions (89.08%), passing the ADR-0021 glue-tier 85% thresholds. All 15
production functions map, none is below 75% region coverage, none is zero, and
the minimum is `prepare_mirror_destination` at 13/16 (81.25%).

Source SHA-256:
`ad566245ef97bee37cd941a1278f58b4b0d9dfc351d51e42513b547cfff6a152`.
Evidence root: `/tmp/cqr-checkpoint-mirror-corrected-Q1hmoD`.
