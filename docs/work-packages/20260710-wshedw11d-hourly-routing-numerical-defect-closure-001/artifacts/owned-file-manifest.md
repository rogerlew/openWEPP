# Owned File Manifest

Status: `EXECUTED-COMPLETE`

Evidence mode: `Static + Ran`

The worktree was clean at package intake. Every W11D-owned modified file belongs
to the declared package write set or its required governance closeout. A
concurrent change to `docs/dev-guide/06-history-and-performance.md` appeared
during execution; it is outside this package, was left untouched, and is not
claimed as W11D work.

## Canonical contracts

| File | Action |
|---|---|
| `SC-ROUTE-001.md` | complete recurrence, branch-specific storage/daily-volume, zero-peak and MC admissibility invariants, guards, tolerances, vectors |
| `SC-SYSTEM-001.md` | terminal event publication through impoundment dependencies, bounded channel ancestry, wave balance semantics |
| `SC-INFILE-CHANINP-001.md` | cardinality-conditional record 4, zero-count closure, consistent applicability metadata |

## Rust production and tests

| File | Action |
|---|---|
| `crates/openwepp-input-contract/src/parsers/chaninp.rs` | parse zero-count three-record canonical payload |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly.rs` | all-interval recurrence, KW/MC hydraulic storage, daily volume, dry carry, MC typed stability guards |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/network_frame.rs` | typed storage fields, channel/impoundment terminal selection, sediment mass integration, unit vectors |
| `crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/hourly_tests.rs` | red/green recurrence/storage, independent Manning, admitted/rejected MC, cross-day and conservation vectors |
| `crates/openwepp-runner/tests/mt3_hbp_hourly_consumer_contract.rs` | real CLI zero/KW/CREAMS/MC release matrix and terminal anti-alias tests |
| `tests/integration/infile_chaninp_parser_contract.rs` | strict/compat zero-count and malformed closure tests |
| `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` | protected MT3 vectors use admissible KW lane; terminal channel/impoundment behavior retained |

## Protected fixture correction

| File | Action |
|---|---|
| `tests/fixtures/watershed/p102-sediment-active/runs/pw0.chn` | wrapper-only `ipeak=4 -> 3`; removes the incidental inadmissible MC grid while preserving the active hourly KW sediment lane |
| `tests/fixtures/watershed/p102-sediment-active/README.md` | records the exact W11D selector change, rationale, scope, and unchanged p102 HBP substrate |
| `tests/fixtures/watershed/p102-sediment-active/input-manifest.sha256` | refreshes the `pw0.chn` checksum; `sha256sum -c` passes all 18 entries |

## Package/governance

This package's `package.md`, `artifacts/*.md`, package catalog, roadmap, and
W11C handoff are the only closeout documents. Reviewers/verifiers were
restricted to their named artifacts; the heavy runner is restricted to
`gate-results.md`. No branch, dependency, secret, generated binary, fixture
blob, or unrelated source file is added. The exact fixture scope above was
added after the first full-profile run exposed the newly invalid incidental MC
selector; the production guard was not weakened and the actual documented
fixture, not a test-only staged copy, now passes.
