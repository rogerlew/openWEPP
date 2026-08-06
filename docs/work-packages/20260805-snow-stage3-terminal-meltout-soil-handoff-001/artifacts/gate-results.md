# Gate Results

Status: Phase-1 authority `BLOCKED`; documentation gates PASS

Base commit: `2f423325`.

| Requirement | Evidence class | Result | Command/evidence |
| --- | --- | --- | --- |
| Applicable instruction chain | Ran | PASS | `tools/agents/find-agents --for ...` returned root, work-package, science-contract, crate, and test instructions for declared paths. |
| Current contract/source authority | Static | BLOCKED | `operand-lineage.md`, `pre-implementation-contract-gate.md`, and independent domain review. |
| HOLD legitimacy | Static | PASS | `hold-legitimacy-audit.md` identifies the missing-authority boundary and rejects five partial/proxy routes. |
| Initial worktree state | Ran | PASS | `git status --short` was empty at intake. |
| Pinned libsnobal identity | Ran | PASS | `git -C /home/workdir/pysnobal rev-parse HEAD` returned `bf8b41c71e3e54ae654ae04005ddf72566c47ee6`. |
| Pinned WEPP baseline availability and source inspection | Ran | PASS | The local checkout HEAD was `2f65506d239b449bbb73c6820ff9cb949fa55158`, so it was not treated as the baseline identity. `git cat-file -e dac3c950d8b16cc73774bf5ce2e7e11f80baac70^{commit}` passed, and commit-qualified `git show dac3c950...:src/<file>` reads passed for `tmpadj.for`, `frostn.for`, `grna.for`, and `watbal_hourly.for`. |
| Rust line-count governance | Ran | HOLD | `wc -l .../03_kernel_support_00_support_helpers.rs .../runoff_reconciliation.rs` returned `997` and `3177`; extraction was not reached because production edits were prohibited. |
| Package Markdown | Ran | PASS | `markdown-doc lint --path docs/work-packages/20260805-snow-stage3-terminal-meltout-soil-handoff-001 --format json`: `26` files, `0` errors, `0` warnings. |
| Package Markdown validation | Ran | PASS | `markdown-doc validate --path docs/work-packages/20260805-snow-stage3-terminal-meltout-soil-handoff-001 --format json`: `26` files, `0` errors. |
| Roadmap/catalog Markdown | Ran | PASS | Separate `markdown-doc lint` runs for `docs/ROADMAP.md`, `docs/work-packages/README.md`, and `docs/planning/snow-surface-energy-balance-roadmap.md`: each `0` errors, `0` warnings. |
| Diff hygiene | Ran | PASS | `git diff --check`. |
| American-English preview | Ran | PASS | `uk2us` preview over changed package prose produced no differences. Catalogs were not normalized because `uk2us` incorrectly rewrites the technical name `CoE`. |
| Dual terminal verification | Static + Ran | PASS | Both independent verifiers reconciled the authority boundary, source identities, review dispositions, narrowed diff, gate selection, and line counts with no remaining substantive findings. |
| Final write-set inventory | Ran | PASS | After disposition and prompt archival, `28` staged paths, with the byte-identical prompt represented as one rename, are all within the four documentation surfaces declared by `owned-file-manifest.md`; the archived prompt SHA-256 exactly matches the base active prompt. |
| Contract/Rust tests | Not run | NOT APPLICABLE to executed Phase-1 HOLD | No contract, Rust, test, manifest, fixture, selector, schema, or runtime source changed. The authority gate prohibited Phase 2 and production execution. |
| Heavy comparator/full workspace | Not run | NOT APPLICABLE | No executable increment exists; the package prompt requires the comparator runner only for selected heavy runs. |

The blocked authority rows prevent implementation completion and produce the
declared `executed HOLD`; they are not represented as passed or deferred.
