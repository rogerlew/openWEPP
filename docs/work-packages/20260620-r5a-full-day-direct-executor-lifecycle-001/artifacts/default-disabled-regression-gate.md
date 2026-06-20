# Default-Disabled Regression Gate

Static plan:

- Runner default fixture must record zero direct-runtime counters.
- H2637 default-disabled median must remain `<= 676.67 s`.
- Direct-runtime and diagnostic environment variables must be unset for H2637
  default-disabled reps.

Ran:

- `cargo test -p openwepp-runner r2a_ -- --nocapture`
  - PASS: `r2a_default_fixture_run_constructs_no_direct_runtime_skeleton`
    observed zero direct-runtime audit counters, including day-frame commits.

- Release build:

  `release_build 57.93 1111320`

  `sha256sum`:

  - `target/release/openwepp-cli-hill`:
    `c1c180b2fb3049288d60b28fc6f0bee7fbf44f3da453ec97414dc1717d0705a0`
  - `target/release/openwepp-cli-hill.json`:
    `28cdca1f1e11dd9eb70546b48998c1d4e49ca38b484ac9e787530367aed4bf19`

- H2637 default-disabled reps with direct-runtime and diagnostic env vars
  unset:

  | Rep | Seconds | Max RSS KB |
  |---|---:|---:|
  | 1 | 643.98 | 228784 |
  | 2 | 647.95 | 228604 |
  | 3 | 643.45 | 229304 |

  Median: `643.98 s`.

Verdict: PASS, median `643.98 s <= 676.67 s`.

Known warnings: each rep emitted `MOFE01-MG-W-001`; warning text and
classification are unchanged and not treated as a package failure.
