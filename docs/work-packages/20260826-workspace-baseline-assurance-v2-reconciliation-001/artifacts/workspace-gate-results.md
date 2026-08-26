# Workspace gate results

Evidence class: `Ran`.

## Exact-clean run setup

Candidate SHA: `edf5f77edf9e159b095996ec8d14c774ba49615f`, equal to
`origin/main` and clean before execution. The protected V9 descriptor library
was bound read-only at its exact absolute path with bubblewrap. The repository,
canonical `/workdir/.cache/openwepp` cache, test scratch, and home state were
the only writable binds.

Two setup attempts were nonqualifying and stopped:

1. A read-only repository bind prevented AUTH06 from creating its test-owned
   temporary tamper fixture.
2. A writable repository bind without explicit `OPENWEPP_CACHE_ROOT` selected
   an empty home cache, losing `cargo llvm-cov` and offline nested-Cargo crates.

Both were environment-configuration failures. The repository remained clean;
no result from either attempt is used as qualification evidence. Before the
canonical attempt, the overlay independently passed exact libcrypto hash,
`cargo llvm-cov --version`, and offline `cargo metadata` prerequisites.

## First canonical-cache full workspace attempt

Command class: `tools/dev/heavy cargo nextest run --workspace --no-fail-fast`
inside the exact V9 overlay with
`OPENWEPP_CACHE_ROOT=/workdir/.cache/openwepp`.

Nextest run ID: `79c0a74a-4129-45ae-9000-6ded71adb19a`.

Result: 3,376 run; 3,359 passed; 17 failed; 6 configured skips. The protected
V9 oracle passed. Eleven failures matched the historical name/signature set.
The only six additional failures were all assertions in three historical
candidate binaries that still read active canonical contracts:

- two `snow_stage3_terminal_chronology_v19_contract` tests;
- two `snow_stage3_terminal_batch_temporal_v20_contract` tests; and
- two `snow_stage3_terminal_batch_temporal_v21_contract` tests.

Disposition: active defect, corrected by binding those historical guards to
their exact preserved Git checkpoints while retaining every assertion. Focused
rerun: 7/7 passed, nextest run ID
`a0b3029a-2546-4359-9804-cfeb7a7602bc`.

An exact-clean full-workspace rerun from the landed correction remains required
for terminal qualification.
