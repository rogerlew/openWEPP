# Contract-test implementation evidence

Status: complete

Evidence mode: Static + Ran

`tests/integration/snow_stage3_terminal_receiver_authority_contract.rs` binds
the five contract families, established table membership, prior-contradiction
absence, numeric terminal-liquid identity, half-open/midnight support, tagged/
zero/overlap/gap/replay poisons, total precedence, and restart transitions.
The existing terminal-numerics contract test now preserves event-only INV-034/
101 semantics at v13/v136.

Test-only functions in
`direct_runtime/surface_liquid_wb14.rs` invoke the actual shared
`compute_green_ampt_interval_infiltration` at 75/437/1125/1800 s against an
independent implicit ponded Green-Ampt bisection oracle. They prove the 437 s
transition is not a proportional full-bin proxy and exercise the existing WB14
wrapper with nonzero cumulative supply/infiltration and mass/excess closure.

Ran via `nix develop`:

- focused contract binaries: PASS, 12/12;
- shared variable-duration WB14 tests: PASS, 2/2;
- `cargo fmt --all`: PASS;
- `git diff --check`: PASS.
