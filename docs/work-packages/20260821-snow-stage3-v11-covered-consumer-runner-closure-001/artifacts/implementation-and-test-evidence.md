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
The actual snow-covered V11 lower-boundary consumer is not yet implemented;
the existing snow-free guard remains the fail-closed behavior.

`Ran:` `nix develop --command cargo check -p
openwepp-hillslope-orchestrator -p openwepp-vegetation
-p openwepp-land-surface-energy -p openwepp-biogeochemistry
-p openwepp-persisted-restart-v1 -p openwepp-runner` passed, with the known
11 dead-code warnings in the historical Stage-3 shadow path.

`Ran:` `nix develop --command cargo test -p
openwepp-hillslope-orchestrator --lib` passed: 739 passed, 0 failed, 1
ignored.

`Ran:` `nix develop --command cargo test -p
openwepp-hillslope-orchestrator --lib --no-run` passed; `nix develop
--command cargo fmt --all -- --check` passed after formatting. `git diff
--check` passed.

`Ran:` warnings-denied Clippy remains blocked by the pre-existing historical
shadow dead-code warnings and existing attachment lint debt; no broad lint
allowance was added.
