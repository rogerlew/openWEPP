# Gate Results

Status: `PASS / terminal direct gates complete`

Evidence mode: `Ran`

| Gate | Result |
|---|---|
| Release build | `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`: PASS; binary SHA `4e0ebd96...da47` |
| Real CLI cohort | four exact-current runs, 61,364 v4 rows, 154 primary windows: PASS |
| Compatibility | every pre-v4 top-level/hourly operand plus WAT/HBP exact at all sites: PASS |
| Independent reconstruction | ten annual fields within `2.2204e-15 m`; storage/handoff/Stage-3 maxima `9.9973e-13`, `1.3878e-17`, `2.5045e-17 m`: PASS |
| Primitive/anti-alias tests | package Python `6/6`; hourly snowfall and routed-liquid mismatches explicitly tested: PASS |
| Focused snow contracts | six existing binaries, `34/34`: PASS |
| Figure custody | six PNG/Markdown/source/generator hash chains: PASS |
| Syntax/format | Python compile, `cargo fmt --all -- --check`, `git diff --check`: PASS |
| JSON | every tracked package JSON parses under `json.tool`: PASS |
| Markdown | every package Markdown plus three catalogs: zero errors/warnings |
| Spelling preview | reviewed; only three false positives that would corrupt the `CoE` acronym or file names |
| Dual review | Review A PASS; Review B PASS; no finding remains |
| Dual terminal verification | Verification A PASS after one low lifecycle-status fix; Verification B PASS; no finding remains |

Exact logs and SHA-256 inventory are retained under
`target/snow_prepeak_mass_transition_physics_adjudication_v2/gates/`.

Quick, frost, and Critical full workspace profiles were not rerun. The package
has no production, contract, test, fixture, selector, or public-output diff;
its frozen exit criteria explicitly make full-workspace correctness
`NOT_APPLICABLE`. The focused existing snow suite and exact real CLI surface
are the conservative direct gates for this documentation/evidence-only change.
