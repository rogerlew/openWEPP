# Implementation and test evidence

Status: `IN PROGRESS / CHILD-1 HOLD RETAINED`.

No implementation or test closure is claimed by the scaffold. Append exact
commands, selected tests, result counts, and source-level consumer proof as
each phase lands.

`Static:` The exact-one snow custody and provider binding seams are now
implemented. Prepared-day fields and support identities are private, provider
binding returns an opaque validated capability, and destination coverage is
checked against every provider receipt. Terminal liquid uses a uniform
tile-ground depth basis and independently reconstructs the OFE-ground mass.
The parent support identity is now exactly 1,800 seconds
(`1_800_000_000_000` ns). Sequential provider validation joins the prepared
day to the committed beginning GSI state and cursor rather than requiring the
new receipt to equal the prior day receipt. Validated-day preflight also joins
each lane's complete destination set to its bound surface-liquid OFE; a lane
permutation fails before any Stage-3/V11 transition.
The actual snow-covered V11 lower-boundary consumer is not yet implemented;
the existing snow-free guard remains the fail-closed behavior.

`Ran:` `nix develop --command cargo check -p
openwepp-hillslope-orchestrator -p openwepp-vegetation
-p openwepp-land-surface-energy -p openwepp-biogeochemistry
-p openwepp-persisted-restart-v1 -p openwepp-runner` passed, with the known
11 dead-code warnings in the historical Stage-3 shadow path.

`Ran:` `nix develop --command cargo test -p
openwepp-hillslope-orchestrator --lib` passed: 742 passed, 0 failed, 1
ignored (743 total); the current suite includes the two new cadence/lane tests
and the two-day provider-bound capability test.

`Ran:` `nix develop --command cargo test -p openwepp-hillslope-orchestrator
--lib snow_stage3_v11_attachment::tests` passed: 2 passed, 0 failed. The
focused provider-bound regression passed: 1 passed, 0 failed, including
day-0-to-day-1 capability binding, replay, skipped-day, substituted-GSI-state,
and rewound-cursor poisons.

`Ran:` `nix develop --command cargo test -p
openwepp-climate-runtime-adapter` passed 8/8; `cargo test -p
openwepp-coupled-time` passed 13/13 across unit, authority, and frozen-oracle
targets; and `cargo test -p openwepp-vegetation --lib v11` passed 9/9.

`Ran:` `nix develop --command cargo test -p
openwepp-hillslope-orchestrator --lib --no-run` passed; `nix develop
--command cargo fmt --all -- --check` passed after formatting. `git diff
--check` passed.

`Ran:` the six-package warnings-denied Clippy command remains blocked with 27
findings: historical Stage-3 shadow dead-code/precision debt, pre-existing
attachment size/argument/precision debt, and existing scheduler/evaluator
debt. The one new needless-borrow finding from this increment was removed; no
broad lint allowance was added.
