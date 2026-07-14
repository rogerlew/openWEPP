# Independent Post-Disposition Verification B

Evidence class: **Static + Ran**

Date: `2026-07-14`

Reviewer B performed this verification independently and did not consult
Reviewer A's verification artifact or messages.

## Final Verdict

| Package | Verdict | Basis |
| --- | --- | --- |
| `20260713-adjudicated-crap-closure-gate-001` | **PASS** | Findings `B-01` through `B-08` are closed. The final fresh gate is closure-eligible and reports `2/2/0` with a complete, stable evidence seal. |
| `20260713-cqr-growth-state-crap-regression-001` | **PASS** | The unchanged growth source remains below the binding threshold, its exact characterization tests pass, and the final fresh workspace census has no actionable row. |

The initial `HOLD` is lifted for both packages.

## Finding Disposition Verification

| Finding | Status | Independent verification |
| --- | --- | --- |
| `B-01` | **CLOSED** | The canonical registry SHA-256 is pinned in the checker and matches the archived registry at `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`. Both entries resolve commit `fa50c0becf6ea63fd9697b4cfe9add66ae036207`; their current and historical source hashes match; and all six adjudication/review evidence hashes match. Negative focused tests reject missing commits, changed or unrelated evidence, source drift, wildcard evidence, and substitute fresh registries. |
| `B-02` | **CLOSED** | An independent retained run produced `ASSESSMENT-PASS`, `debt_status=PASS`, `closure_eligible=false`, no base/head or touched-scope claim, and a hashed repository provenance artifact. The fresh report records LCOV, CRAP JSON, source-manifest, registry, Cargo, rustc, cargo-llvm-cov, and cargo-crap provenance. Retained evidence therefore cannot masquerade as current closure evidence. |
| `B-03` | **CLOSED** | Static workflow verification confirms that `release_dir` is published before execution, combined output is teed to `release-gates.log`, `PIPESTATUS[0]` is preserved, the exit status is recorded, and artifact upload uses `if: always()`. The workflow YAML parses successfully. |
| `B-04` | **CLOSED** | Hosted CI installs pinned `cargo-nextest 0.9.138`, `cargo-llvm-cov 0.8.7`, and `cargo-crap 0.2.2`. The release driver records `cargo nextest --version`; the fresh gate also seals verbose Cargo/rustc and measurement-helper versions. |
| `B-05` | **CLOSED** | Touched discovery uses `--name-status --find-renames --diff-filter=ACMRD`, retains status, and records both rename endpoints. Direct deletion/rename coverage passes in the 17-test focused suite. |
| `B-06` | **CLOSED** | Cleanup and the `EXIT` failure-envelope trap are installed before option-combination, prerequisite, or acquisition checks. A direct stale-PASS reproduction with omitted retained provenance exited `2`, removed the old report, emitted a fresh `FAIL` `run-status.json`, and replaced the old checksum file with a hash of the new envelope. The final passing run hashes all 16 generated lineage artifacts; every checksum verifies. |
| `B-07` | **CLOSED** | The final instrumented log's only failed target is `laned_shadow_h2637`. Its source header requires Nextest process isolation because its process-global environment mutations race under threaded `cargo test`; the observed failures match that mode. The binding terminal full-Nextest result remains `1,960/1,960` on growth source SHA-256 `1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`. |
| `B-08` | **CLOSED** | Characterization now names the exact annual and perennial consumer tests, and the retrospective records completed implementation and measurement. Reviewer B reran the zero-cap, annual, and perennial exact test IDs; each passed `1/1`. |

## Final Fresh Evidence Seal

The terminal command was:

```text
bash tools/release/run_adjudicated_crap_gate.sh --base-ref 3071849a0aec2abf8c17fe2405ce468f1533f631 --output-dir /tmp/openwepp-acrap-final-20260713
```

`run-status.json` records fresh acquisition, exit `0`, and `PASS` from
`2026-07-14T07:51:01Z` through `2026-07-14T08:26:23Z`.

### Identity And Census

- `source-manifest-before.json`, `source-manifest-after.json`, and
  `source-manifest-final.json` are byte-identical at SHA-256
  `2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483`.
- Reviewer B independently regenerated the manifest and obtained the same
  bytes and hash.
- Each schema-v2 manifest contains `216` production sources and `419`
  measurement inputs, including `rust-toolchain.toml` at SHA-256
  `3e18e70208ee460635e239a91c142cf67371feafb718b05617ff06f388bf96df`.
- The earlier requested `418`-input identity is intentionally superseded:
  `418` omitted the consulted toolchain selector and cannot be closure
  evidence for the corrected implementation.
- An independent `cargo metadata` census of manifests under `crates/` matches
  both report lists exactly at `17/17` production crates.

### CRAP Closure

- Report status: `PASS`; debt status: `PASS`; closure eligible: `true`.
- Production entries: `8,330`.
- Raw/adjudicated/actionable: `2/2/0`.
- Invalid or stale adjudications: `0`.
- Touched record: `M
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`.
- Touched and untouched actionable rows: `0/0`.
- Caller metric: CC `27`, coverage `97.22222222222221%`, CRAP `27.015625`.
- Extracted helper metric: CC `5`, coverage `100%`, CRAP `5`.
- Growth source: `1,668` lines, SHA-256
  `1ce345e533159d7317f8c7d1a5f41b292a27896aa53d8e10d693d6366a6eb041`.

### Principal Artifact Hashes

| Artifact | SHA-256 |
| --- | --- |
| `adjudicated-crap-report.json` | `3ad2a65a0c8526ab2155bad26d08f915e8b257f5b70b62f3006dd78381fe098d` |
| `workspace-crap.json` | `5fe3d67263508a9c2a7fbfb473ab40b00380d38392a5a033a33e4658a9452c40` |
| `workspace.lcov` | `bf6c20a4dab61145011051e982aff4d749190036979e7298cbeacecf2a9c9256` |
| all three source manifests | `2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483` |
| `adjudication-registry.json` | `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f` |
| `sha256sums.txt` | `9838e7d5533d60ce6048d414aefb0a245247f5c7169d8440fc8d7f0873191a1f` |

The recorded toolchain is Cargo `1.92.0`, rustc `1.92.0`, cargo-llvm-cov
`0.8.7`, and cargo-crap `0.2.2`.

## Reviewer-Executed Checks

| Check | Result |
| --- | --- |
| `.venv/bin/python -m unittest -v tests.python.test_adjudicated_crap_gate` | **PASS**, 17/17 |
| Python compile check and shell syntax check | **PASS** |
| Canonical registry pin, current/historical source hashes, and six evidence hashes | **PASS** |
| Independent retained assessment | **PASS**, assessment-only `2/2/0` |
| Direct pre-acquisition stale-PASS reproduction | **PASS**, fail-closed envelope replaced stale evidence |
| Independent schema-v2 manifest generation and four-way comparison | **PASS**, byte-identical `216/419` |
| `sha256sum -c` over the final seal | **PASS**, 16/16 |
| Independent Cargo-metadata/report crate comparison | **PASS**, `17/17` |
| Exact zero-cap, annual, and perennial Nextest filters | **PASS**, each `1/1` |
| Workflow YAML parse and CI integration inspection | **PASS** |
| `git diff --check` | **PASS** |

## Residual Risks

- The coverage subprocess uses threaded libtest and therefore retains the
  documented `laned_shadow_h2637` environment race. It must not replace the
  separate full-Nextest authority.
- `sha256sums.txt` records absolute generation paths. Its hashes are complete,
  but direct `sha256sum -c` use after relocating an uploaded artifact requires
  path adjustment. This is an operational usability risk, not a content-binding
  or closure defect.
- CRAP closure verifies maintainability debt and its evidence lineage. It does
  not itself validate scientific models or physical correctness.

No source changes were made during Reviewer B verification.
