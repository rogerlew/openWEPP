Ran 2026-09-25 under the detached writer source
`/home/roger/openwepp-experiments/cold-canopy-m1-20260920`.

`nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo fmt -p openwepp-land-surface-energy` exited 0.

`nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo check -p openwepp-land-surface-energy --tests --features m1-trust-region-physical-stage` exited 0. The checks compile the test-only recorder hooks; pre-existing test-only unused/dead-code warnings remain.

`nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-cold-canopy-m1-target CARGO_BUILD_JOBS=2 cargo check -p openwepp-land-surface-energy --tests` exited 0.

No result-bearing physical selector was run. The proposed selector remains the already-failing required ordinary positive:
`m1_coupled_tests::m1_trust_region_physical_adapter_ordinary_update_reassembles_before_root_materialization`.
