# Gate Results

Evidence mode: `Ran + pending terminal`

Status: `focused gates pass; corrective terminal rerun pending`

Focused package tests, integration tests, gate-policy schema checks, formatting,
and selected-package Clippy pass. An earlier six-package nextest attempt exposed
the intentional 32-byte `DirectDayFrame` growth from two exact consumer
observations; the size guard was documented and updated, and its focused test
then passed. That interrupted run is diagnostic only and is not closure
evidence.

Generation 7 gate policy now binds every changed science path to exactly one of
the three amended contracts and to a package-level A1 gate. A fresh exact-diff
critical plan and passing receipt remain mandatory for closure.

The first node-executing terminal receipt at exact head `c626deb0` is retained
at `/tmp/canopy02-terminal-repaired-c626deb0/receipt.json` with receipt ID
`d607f2ef...`: 12 PASS, 2 FAIL, and 1 BLOCKED. All three canopy A1 gates passed
with exact inventories of 71 management/input/migration tests, 19 plant tests,
and 624 runtime/orchestrator tests. A0 failed because an input parser contract
had been added to the science-only registry; focused A0 passes after removing
that row. Full nextest's sole failure was a Unix socket exceeding `SUN_LEN`
under the long external artifact root, so the exact rerun will use a short
`/tmp` artifact root. Global CRAP was blocked only by that failed prerequisite.
