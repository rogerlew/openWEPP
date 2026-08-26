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

## Terminal exact-clean workspace qualification

Candidate SHA: `58ea61a2c303756f5f01c2f81f2516534750377c`, equal to
`origin/main` and clean before and after execution.

Nextest run ID: `1e58916c-6350-421e-8100-301bc6ccef56`.

Result: 3,376 run; 3,365 passed; exactly 11 failed; 6 configured skips. A
mechanical comparison found an empty diff between both the final failure-name
set and normalized-signature set and the retained historical-eleven census.
No Assurance, retained-guard, historical-candidate, or protected V9 test
failed. The one-file V9 overlay retained exact libcrypto SHA-256
`0cd331307536a397ab9c83c6dbeeb3474d0a5114f397ce03d1762adb96d3c781`.

Disposition: `PASS / QUALIFIED BASELINE`. The command returns nonzero only
because the exact historical eleven remain intentionally visible; no waiver or
expected-failure annotation was added. The repository remained clean.
