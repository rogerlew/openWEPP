# Default-Disabled Regression Gate

Static/Ran:

- PASS: `cargo test -p openwepp-runner r2a_ -- --nocapture`
  - Default fixture direct-runtime counters remained zero.
  - Explicit opt-in fixture recorded R5B direct-runtime counters and one
    compatibility-edge handoff.
- PASS: H2637 default-disabled release reps with direct-runtime/perf
  diagnostic env vars unset:
  - rep1 `643.38 s`, `229332 KB`
  - rep2 `640.54 s`, `229036 KB`
  - rep3 `644.59 s`, `229020 KB`
  - median `643.38 s`
  - threshold `<= 676.67 s`

Known warning retained: `MOFE01-MG-W-001` appeared once per H2637 rep and did
not change classification.

Verdict: PASS.
