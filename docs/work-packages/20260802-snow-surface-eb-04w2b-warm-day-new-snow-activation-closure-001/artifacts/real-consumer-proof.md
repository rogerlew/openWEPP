# Real-Consumer Proof

Status: passed

Evidence mode: **Static + Ran**

The production direct-runtime path now resolves SIMIMPL28 typed hourly rows for
any material precipitation before the snow decision. Direct publication's
pre-gate depends only on material precipitation or prior SWE; it no longer
uses daily mean temperature. Its focused real-path test loads the production
hillslope fixture, supplies a warm-mean, zero-pack day whose unchanged
Harder-Pomeroy provider produces hourly snow, constructs the real production
run frame and day input, and proves the published snow coupling carries a
storage gain plus positive after-day SWE into the hydrology projection. A
separate SIMIMPL28 test proves warm precipitation resolves rows while warm
dry/no-pack forcing remains suppressed.

The registered EB-04W integration target independently passes warm new-snow,
mixed rain/snow, all-rain, and exact/just-over activation-threshold vectors at
the shared public boundary.

The `openwepp-runner` `snowbench_coe_melt` executable also calls the same public
partition and independently validates its consumer-visible before/input/
output/after operands. A focused consumer test proves a material disagreement
is rejected. The first frozen W2A runner executed the release snowbench binary
for all eight cells with zero process failures and maximum reconstructed mass
residual `2.220e-15 m`, but review adjudicated that run
prerequisite-ineligible; it carries no terminal closure or scientific-result
authority.

These are real downstream consumers; no wrapper, skeleton, shadow counter,
daily-mean pre-gate, or producer-only assertion carries the closure claim.
