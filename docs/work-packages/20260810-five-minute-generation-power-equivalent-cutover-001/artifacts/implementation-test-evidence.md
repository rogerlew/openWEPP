# Implementation Test Evidence

Status: `PASS — focused reopened-package implementation gates`

Evidence mode: `Ran`

Ran from `/home/workdir/openWEPP` on the reopened implementation diff:

- Transaction, runner, and writer regressions cover existing-target sentinel
  preservation, concurrent-WAT5 refusal, day-2 source failure, forced WAT5
  close failure, forced publication and staged-unlink rollback, surfaced
  rollback-cleanup failure, missing-manifest rollback, strict bounded-memory
  chronology, final-name serialization, and successful output-set/manifest
  commit: 21/21, nextest `30732108-accb-414b-b324-7a60985b3a65`.
- Affected orchestrator/output/runner packages: 768/768, nextest
  `93c7e2bc-46f7-49e2-8d55-4c9a61983df7`.
- Named peak/WAT5 contract, property, storage-aware typed roundtrip, and
  HBP-routing exclusion targets: 14/14, nextest
  `e26fb292-0fc6-47bc-9a90-60ce41ee0fd7`.
- Advisory/direct-authority policy plus required-suite anti-evasion contracts:
  10/10, nextest `a9ac9af6-7843-4c11-9f26-00ff26562e7f`; shell anti-evasion
  guard PASS.
- Exact-worktree A0 admission: 43 contracts, 13 reopened science surfaces,
  authority fingerprint
  `84494b9e8a10a1cbada449106c9038732cdff9edbb4b76a3703d76a72f7948c7`.
- Package feasibility tooling: 5/5 in 0.73 seconds.
- Affected orchestrator/output/runner Clippy with all targets/features and
  `-D warnings`: PASS.

The package-plan commands named `power_equivalent_erosion_contract` and
`power_equivalent_real_consumer` are not test targets because the prospective
erosion branch ended at `NO_ADOPTION`; they are non-applicable, not omitted
passes.

An attempted dirty-tree A1 package run was deliberately interrupted after 180
passes when it entered the same long assurance inventory as the terminal full
campaign. It is non-admitted and will be superseded by the exact-clean full
workspace receipt.

Historical pre-reopen focused receipts retained below describe the original
implementation and are not substitutes for the new evidence:

- Orchestrator WAT5 behavior plus frame-layout guard: 18/18, nextest
  `c3cc26c2-a8df-431f-97e4-f09577bedf7c`.
- Output contract/path/atomicity suite: 23/23, nextest
  `760f1daa-50dc-4c30-9015-a87c19b67fc0`.
- Named WAT5 contract/property/typed roundtrip/HBP-routing exclusion/peak
  targets: 13/13, nextest `d7fa0d54-ee3e-4f7a-a6dc-d03bbf3b959c`.
- Named unit-boundary conversion vector: 1/1, nextest
  `d40f5549-38c1-423c-8668-0df9c970ec90`.
- Unit registry: 21/21 via `bash tools/release/check_unit_registry.sh`.
- Package feasibility tooling: 5/5.
- Affected-crate Clippy with all targets/features and `-D warnings`: PASS.
- `cargo fmt --all -- --check`, `cargo check --workspace`, and
  `git diff --check`: PASS at the focused gate.

Real CLI, independent Parquet reconstruction, source rejection, and protected
byte comparisons are recorded in the adjacent evidence artifacts.
