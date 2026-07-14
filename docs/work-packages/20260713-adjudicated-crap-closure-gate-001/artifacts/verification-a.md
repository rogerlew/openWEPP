# Independent Verification A: Adjudicated CRAP Closure Gate

Evidence class: **Ran + Static**

Final recommendation: **PASS**. Reviewer A's initial `HOLD` is lifted.

Reviewer A read the shared disposition and terminal post-review heavy evidence,
then verified the dispositions against the current source and generated
artifacts without reading Reviewer B's verification.

## Finding Verification

| Finding | Status | Verification |
| --- | --- | --- |
| `A-GATE-001` | **PASS / closed** | The driver creates before, after, and final manifests and compares them around measurement and report publication. Each manifest fingerprints HEAD, the full Git index, 216 production sources, and 418 measurement inputs. The three post-review manifests are byte-identical at SHA-256 `99930077a9965e42cb5791c52e909e17791fa400d869dd09f8c5b3c5ad80a367`. An independently generated current manifest has the same hash; all 634 listed source/input hashes were independently recomputed with zero mismatches. The source-mutation test passes. |
| `A-GATE-002` | **PASS / closed** | Fresh driver mode rejects any `--adjudications` override; the checker separately requires the canonical resolved path and reviewed registry hash. The canonical, archived, and checker-pinned hashes all equal `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`. For both entries, commit `fa50c0becf6ea63fd9697b4cfe9add66ae036207` resolves, source-at-commit and current source match the registered hash, and all adjudication/review evidence hashes and exact key/function/classification/hash/acceptance tokens validate. Negative substitute-registry, missing-commit, changed-evidence, wildcard, stale-source, and under-evidenced tests pass. |
| `A-GATE-003` | **PASS / closed** | Retained driver execution independently emitted `ASSESSMENT-PASS`, `acquisition_mode=retained`, and `closure_eligible=false` with a hashed repository provenance file. Fresh mode requires LCOV, pinned tool-version files, a current source manifest, and an exact 17-crate production census; independent Cargo metadata comparison matched the report's 17/17 sets. Generated canonical filenames are removed before acquisition, and the EXIT trap writes `run-status.json` plus hashes on failure or success. The stale-PASS/failure-envelope test passes. CI establishes `release_dir` before execution, records the pipeline exit, tees the log, and uploads with `if: always()`. Hosted CI installs Nextest `0.9.138`; the release runner records the actual version, and the current local version is `0.9.138`. |
| `A-GATE-004` | **PASS / closed** | Changed-path discovery uses name/status output with `ACMRD`, preserves deletion status, and emits both endpoints of renames/copies. The deletion and `R100-from`/`R100-to` focused test passes. The terminal report records the actual touched row as `M crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs`. |

## Terminal Fresh Evidence

The post-review hardened driver ran from an absent output directory using base
`3071849a0aec2abf8c17fe2405ce468f1533f631` and exited `0`. Independent checks
confirmed:

- `run-status.json`: fresh acquisition, exit `0`, result `PASS`;
- every entry in `sha256sums.txt`: `OK`;
- source manifests: `216` production sources, `418` measurement inputs, unique
  path counts equal declared counts, current HEAD/index and every file hash
  still match;
- measurement-input composition: `394` Rust paths and `24` Cargo/config/gate
  inputs, including `144` top-level `tests/*.rs` paths;
- production census: exact Cargo-metadata/report match at `17/17` crates;
- report: `status=PASS`, `debt_status=PASS`, `closure_eligible=true`, `8,330`
  production entries, `2` raw, `2` adjudicated, `0` actionable, and `0` invalid
  adjudications;
- touched inventory: one modified production source and zero touched or
  untouched actionable rows.

An independent checker invocation over the sealed LCOV/CRAP/manifests and the
current canonical registry again returned:

    adjudicated-crap: status=PASS raw=2 adjudicated=2 actionable=0 touched_files=1

## Independently Run Checks

| Command/check | Result |
| --- | --- |
| `.venv/bin/python -m unittest -v tests.python.test_adjudicated_crap_gate` | **PASS**, 15/15 in 4.630 seconds |
| Fresh checker reassessment of the sealed post-review artifacts | **PASS**, `2/2/0`, one touched file |
| Current source-manifest generation and comparison with before/after/final | **PASS**, same `216/418` manifest and SHA-256 |
| Independent recomputation of all manifest row hashes | **PASS**, zero mismatches |
| `sha256sum -c /tmp/openwepp-acrap-postreview-20260713/sha256sums.txt` | **PASS**, every sealed artifact |
| Independent Cargo metadata versus reported production crates | **PASS**, exact `17/17` set |
| Independent commit/source/evidence hash-and-token audit for both registry rows | **PASS** |
| Retained driver assessment with canonical campaign artifact | **PASS as assessment only**, `ASSESSMENT-PASS`, closure ineligible |
| Fresh driver invocation with a substitute registry path | **Rejected**, exit `2` before measurement |
| `.venv/bin/python -m py_compile` for checker and tests | **PASS** |
| `bash -n` for both release scripts | **PASS** |
| JSON, workflow YAML, and `git diff --check` | **PASS** |

The delegated terminal Rust lane remains applicable because the Rust source
identity is unchanged: format, workspace Clippy, full-profile Nextest
(`1,960/1,960` executed tests), and deny passed. The hardened gate rerun changed
only gate/governance surfaces and proved the same Rust source in its final
manifest.

## Residual Risks

- The measurement manifest intentionally fingerprints repository Rust,
  Cargo/config, and gate inputs. It does not claim to fingerprint every
  external fixture, environment variable, compiler installation, or dependency
  cache. Tool versions and ordinary test authority remain separately recorded.
- Instrumented `cargo llvm-cov --ignore-run-fail` observed the documented
  threaded `laned_shadow_h2637` environment race. This is explicitly attributed
  in heavy evidence; isolated full-profile Nextest passed on the same Rust
  source and remains ordinary-test authority.
- GitHub `always()` upload and Nextest installation were verified statically and
  by YAML parsing, not by launching a hosted Actions job in this review.

These are visible operational boundaries, not unresolved closure defects.

## Final Disposition

`A-GATE-001` through `A-GATE-004` are accepted, implemented, and independently
verified. There is no remaining Reviewer A blocker. The adjudicated CRAP gate
package is eligible for final `PASS` closure after Reviewer B's independent
verification and the parent package's final bookkeeping.

## Final Residual-Fix Verification (2026-07-14)

Evidence class: **Ran + Static**

Final residual-fix recommendation: **PASS**.

This verification was performed independently without reading Reviewer B's
verification. It addresses the two accepted post-disposition gaps against the
current implementation and the sealed fresh run at
`/tmp/openwepp-acrap-final-20260713`.

### Failure-Envelope Ordering And Stale-PASS Rejection

Static inspection confirms that every non-help invocation establishes the
output directory, generated-artifact cleanup list, and `EXIT` trap before
parse-error, semantic-mode, or prerequisite-failure exits. The successful
help-only path intentionally exits without creating an evidence envelope.

Three independent reproductions each began with a stale PASS report and stale
checksums, then exercised an early failure:

- an unknown argument parse failure;
- a fresh/retained semantic failure caused by supplying `--crap-json` without
  the required `--retained-provenance`;
- a missing alternate-registry prerequisite failure.

All three exited `2`, removed the stale PASS report, emitted a new
`run-status.json` with `result=FAIL` and `exit_code=2`, and produced a new
checksum inventory that passed `sha256sum -c`. The omitted-provenance stale-PASS
regression is therefore closed on the exact failure class identified during
post-disposition review.

### Final Sealed Fresh Evidence

Independent inspection and recomputation established:

- before, after, and final manifests are byte-identical, use
  `openwepp-production-source-manifest-v2`, contain `216` production sources
  and `419` measurement inputs, and have SHA-256
  `2b40242a65895c3e1dff365c87e8eca237570a188313fd7777a741c019096483`;
- an independently generated current manifest has the same hash and is
  byte-identical to the sealed final manifest;
- `rust-toolchain.toml` is present in the measurement-input set, and independent
  recomputation of all `216` source hashes and `419` input hashes found zero
  mismatches;
- sealed `cargo-version.txt` and `rustc-version.txt` exactly match the active
  `cargo --version --verbose` and `rustc --version --verbose` output, and both
  complete records are included in report acquisition provenance;
- every entry in the final `sha256sums.txt` passes verification;
- the report is fresh, closure-eligible, and `PASS`, with an exact `17/17`
  Cargo-metadata/report production-crate census, `2` raw rows, `2` adjudicated
  rows, `0` actionable rows, and `0` invalid adjudications;
- an independent checker reassessment returned
  `status=PASS raw=2 adjudicated=2 actionable=0 touched_files=1`.

The focused Python suite now passes `17/17`; Python compilation, shell syntax,
and `git diff --check` also pass. The release README and ADR-0021 accurately
describe the active Cargo/Rust compiler provenance, the manifest-v2 toolchain
selector coverage, and early-failure envelope behavior.

### Residual Disposition

Both accepted post-disposition gaps are implemented and independently verified.
No Reviewer A blocker remains. The adjudicated CRAP closure-gate package retains
its final **PASS** recommendation.
