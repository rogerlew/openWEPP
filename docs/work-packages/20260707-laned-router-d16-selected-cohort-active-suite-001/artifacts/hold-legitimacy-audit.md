# Hold Legitimacy Audit

Status: EXECUTED-HOLD-ACTIVE-RUN. Evidence mode: Static + Ran.

## Hold

The selected-cohort active plain-vs-hybrid suite cannot complete because
`mn_corn_h4` fails in active plain before publication:

```text
CLIHILL-E-011 runtime surface failure for r7c_direct_production_executor: HS-SIMPIPE-E-001 direct runtime day execution failed at lane 1 day 136: direct runtime kernel guard failed in laned_active_rev21_operands: lane 1 day 136 has LAI 0.01182723510043506 > 0 with missing/non-positive typed-management canhgt (rev-21 fail-closed)
```

## Why This Is Legitimate

- The selected member is source-authorized: `_map = disturbed`, class
  `agriculture crops`, selected `wepp_id = 4`, and native route coefficients
  are generated from WEPPpy Disturbed's explicit table.
- The runfile is valid after the package-local repair: `snow.txt` remains a
  copied sidecar and is not emitted as an invalid TOML string key.
- H2637 active plain and true active hybrid both run with separate output dirs;
  the failure is not a global runner, binary, or env setup failure.
- The failing guard is required by the Rev-21 operand path. It prevents routed
  friction from consuming vegetation drag when LAI is positive but typed
  canopy height is missing or non-positive.

## Static Source Evidence

The initial management projection publishes seed controls such as
`cancov_seed`, `bb_seed`, `bbb_seed`, `flivmx_seed`, and `hmax_seed`, but the
shown seed insertion block does not publish `canhgt_seed`:

- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/01_management.rs:539`

The active PMET authority path requires typed `canhgt` when PMET coefficients
are active, and the active day input subsequently requires LAI and canopy
height to be coherent:

- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:1252`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs:1264`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs:1126`

The current failure is therefore not safely closeable by inserting a fallback
height in the package-local `.man` file. That would create surrogate physics
and weaken the active Rev-21 fail-closed invariant.

## Outside This Package

Closing the blocker requires an authority-backed active row-crop canopy-height
runtime publication/source-lift:

1. Identify the baseline-authoritative source for annual-crop `canhgt` through
   plant growth and management projection.
2. Bind the symbol in the relevant science contract or runtime-surface
   contract.
3. Publish a positive typed `canhgt` on LAI-positive active crop days, or
   prove the guard should continue to fail for that case.
4. Add focused tests for row-crop active Lane D with LAI-positive days.
5. Rerun this selected cohort suite and then re-adjudicate D16/default
   promotion.

First follow-on package/action:

- `D16-ROWCROP-CANHGT-ACTIVE-RUNTIME-PUBLICATION-001`: source-authorize and
  implement active row-crop canopy-height runtime publication for Lane D Rev-21
  friction operands, then rerun the selected cohort.
