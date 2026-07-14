# Dual-Review Finding Disposition

Evidence class: **Static + Ran**

Status: `VERIFIED; CLOSED`

All findings were accepted. No finding was rejected, deferred, or converted to
an unowned follow-up.

| Finding | Disposition | Implemented response |
| --- | --- | --- |
| `A-GATE-001` | accepted | Fresh closure snapshots production Rust plus all Rust/Cargo/gate measurement inputs, including `rust-toolchain.toml`, HEAD, and the complete Git index before, after, and after report generation. Any mismatch fails. Source and toolchain-selector mutation are directly tested. |
| `A-GATE-002` | accepted | Fresh closure is canonical-registry-only and the reviewed registry SHA-256 is pinned. Each entry now proves the adjudicated commit, source-at-commit hash, current source hash, evidence hashes, and exact binding/acceptance tokens. Substitute registries remain assessment-only. |
| `A-GATE-003` | accepted | Retained mode emits `ASSESSMENT-*` with `closure_eligible=false` and required repository provenance. Fresh mode enforces the current 17-crate production census. Generated filenames are cleared, every run emits a status/checksum envelope, and CI uploads logs/artifacts under `always()`. |
| `A-GATE-004` | accepted | Touched discovery includes `D`, records status, and emits both rename endpoints. Deletion and rename tests pass. |
| `B-01` | accepted | Same implementation as `A-GATE-002`; negative tests cover missing commit and changed/unrelated evidence. |
| `B-02` | accepted | Same implementation as retained-mode portion of `A-GATE-003`; LCOV, Cargo/Rust/compiler-helper versions, manifest, CRAP JSON, and retained provenance are machine-readable. |
| `B-03` | accepted | The workflow publishes `release_dir` before execution, tees the release log, records exit status, and uploads with `if: always()`. |
| `B-04` | accepted | Hosted CI installs pinned `cargo-nextest 0.9.138`; release evidence records `cargo nextest --version`. |
| `B-05` | accepted | Same implementation as `A-GATE-004`. |
| `B-06` | accepted | Parsing accumulates errors so the chosen output directory is known before failure. Known generated files are removed and an EXIT trap is installed before option-combination, prerequisite, or acquisition checks. The trap writes `run-status.json` and hashes every remaining artifact/log on pass or failure. Direct pre-acquisition and evaluation-failure stale-PASS tests pass. |
| `B-07` | accepted | Heavy evidence now cites the `laned_shadow_h2637` source header and attributes the instrumented failure to the documented threaded environment race; isolated full Nextest remains test authority. |
| `B-08` | accepted | Growth characterization names the exact annual and perennial tests and the package retrospective reflects completed implementation/measurement. |

## Residual Verification Findings

Reviewer A returned `PASS`. Reviewer B kept closure on `HOLD` after reproducing
two narrower residual gaps: B-06 cleanup occurred after semantic option checks,
and the measurement manifest omitted the consulted `rust-toolchain.toml`.
Both findings are accepted and implemented as described above. The toolchain
selector is now part of manifest schema v2, the driver also seals verbose Cargo
and rustc versions, and the focused suite passes 17/17.

The new fresh full-workspace census proved identical manifest-v2 snapshots at
`216/419` and closed at `2/2/0`. Reviewers A and B independently returned
`PASS` for both packages and lifted their initial HOLDs. Every finding is
closed; no finding is rejected, deferred, or left without an owner.
