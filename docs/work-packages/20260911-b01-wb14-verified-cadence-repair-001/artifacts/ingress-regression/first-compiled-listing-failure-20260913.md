# First compiled-listing failure

Evidence class: Ran (listing build; no test selection or execution).

Commands from `/workdir/openwepp-experiments/b01-wb14-cadence/baseline-red`:

First invocation (format check, format fallback, then listing in one shell
command):

```sh
nix develop /workdir/openWEPP --command cargo fmt --manifest-path /workdir/openwepp-experiments/b01-wb14-cadence/baseline-red/Cargo.toml --check || nix develop /workdir/openWEPP --command cargo fmt --manifest-path /workdir/openwepp-experiments/b01-wb14-cadence/baseline-red/Cargo.toml; mkdir -p /tmp/openwepp-b01-wb14-cadence-targets/baseline-red; nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-cadence-targets/baseline-red RUST_MIN_STACK=67108864 cargo nextest list --manifest-path /workdir/openwepp-experiments/b01-wb14-cadence/baseline-red/Cargo.toml -p openwepp-hillslope-orchestrator --lib --locked --features persisted-restart-v1,restart-authority-evidence -E 'test(b01_wb14)'
```

The visible output showed compilation beginning. Its complete shell stdout/stderr
was not redirected and is UNOBSERVED. The first invocation's numeric exit is
UNOBSERVED from the tool wrapper.

Second invocation (the same listing command, before the source-stop direction):

```sh
nix develop /workdir/openWEPP --command env CARGO_TARGET_DIR=/tmp/openwepp-b01-wb14-cadence-targets/baseline-red RUST_MIN_STACK=67108864 cargo nextest list --manifest-path /workdir/openwepp-experiments/b01-wb14-cadence/baseline-red/Cargo.toml -p openwepp-hillslope-orchestrator --lib --locked --features persisted-restart-v1,restart-authority-evidence -E 'test(b01_wb14)'
```

The second invocation first reported `Blocking waiting for file lock on artifact
directory`, then emitted the retained compiler diagnostics and failed. Its numeric
exit is UNOBSERVED from the tool wrapper. No listing/test count was emitted.

First local error: E0432 at
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/surface_liquid_ingress_context_tests.rs:12`:
`super::DirectSurfaceLiquidOfeBinding` is not re-exported by
`direct_runtime::surface_liquid_ingress`; the compiler identifies the crate
re-export as the available path. The same cut reports unused `SoilLayerId`.

Also reported, outside the authorized write set, are E0004 non-exhaustive
`DirectTileGroundIngress` matches in the included integration authority sources:

* `tests/integration/land_surface_energy_real_hydrology_shadow_contract/raw_hash_tests.rs:220`
* `tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs:192`
* `tests/integration/land_surface_energy_real_hydrology_shadow_contract/precedence_tests.rs:710`

No correction was applied after this failed cut. Host `rustfmt` was unavailable;
the first invocation used `cargo fmt --check` and then Nix `cargo fmt` before the
listing command. The raw compiler diagnostic fingerprint is
`first-compiled-listing-rustc-diagnostics.jsonl`; the exact failed module,
fixtures and patch are under `first-compiled-listing-artifacts/`.
