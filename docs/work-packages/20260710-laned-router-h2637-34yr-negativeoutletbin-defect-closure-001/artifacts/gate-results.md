# Gate Results

Status: `EXECUTED-PASS`

Evidence mode: `Ran`

All commands ran from `/home/workdir/openWEPP` on 2026-07-11 unless an
artifact explicitly records its scratch working directory.

## Required Repository Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | final post-review pass, `9.88 s` |
| `cargo nextest run --workspace --profile full` | final post-review pass, run `e6e84783-62a8-4b91-9f5f-2a8b6a0cf222`; `1694` passed, `3` intentionally skipped, `593.690 s` |
| `cargo deny check` | pass: advisories, bans, licenses, and sources all `ok` |
| `git diff --check` | pass |

## Contract and Unit Governance

| Gate | Result |
| --- | --- |
| `.venv/bin/python tools/check_sc_binding_exposure.py .../SC-OFEROUTE-001.md` | `PASS-DEFERRED`: 10 binding-exposure rows and 9 preexisting science-review follow-on rows not yet consolidated |
| `bash tools/release/check_sc_unit_compliance.sh --path .../SC-OFEROUTE-001.md` | pass, no findings |
| `bash tools/release/check_unit_registry.sh` | pass: 21 registry tests plus workspace checks |
| `markdown-doc lint` on the final package directory | pass: 26 files scanned, no errors or warnings |
| `markdown-doc lint` on `SC-OFEROUTE-001.md`, the contract index, and work-package catalog | pass: no errors or warnings |
| `uk2us` spelling preview | package and touched contract surfaces need no further normalization; catalog-only suggestions were preexisting out-of-scope rows and were preserved |

The binding checker result is its defined successful deferred posture; rev 51
does not add a new unresolved binding row. The documentation results above are
the final post-review, post-verification, post-disposition sweep.

## Touched Invariants and Acceptance Cases

| Evidence | Result |
| --- | --- |
| original exact-dry pre-fix contract vector | expected fail with `NegativeOutletBin`, run `30a17d5a-de3e-41d0-9bab-513da8203b6a` |
| strengthened positive-outlet pre-fix vector | expected fail with `NegativeOutletBin` after temporarily removing only the rev-51 lower-bound line, run `22a7683c-1528-444b-9bb6-c7f630bc96f4` |
| final strengthened vector + recorder defense | `2/2` pass after restoring the exact production line, run `287ebe1a-0f18-4a1f-bdc2-86c352289576` |
| full orchestrator crate | `340/340` pass in `148.988 s` |
| Case-4 oracle ladder + 19-OFE conservation | `2/2` pass, run `95f6d51e-c068-4034-a648-94c4f978d3b8`, `151.001 s` |
| selected active `dx=5 m` cohort | `3/3` pass; see `suite_runner.md` |
| canonical H2637 34-year endpoint | both effective `wepp_ui` modes exit `0`; see `endpoint-validation.md` |
| protected daily/off outputs | all five outputs byte-identical; see `fidelity-and-byte-identity.md` |

## Source-Level Scope Checks

The production diff is confined to downstream predictor face construction in
`kinematic_wave.rs`; the defensive bin guard and its tolerance are unchanged.
No external-authority cohort fixture or required-case binding was edited, so
the external-authority anti-evasion guards are not triggered by this write
set. The corrected source file has `2570` lines; its warning-level disposition
is recorded in `line-count-governance.md`.
