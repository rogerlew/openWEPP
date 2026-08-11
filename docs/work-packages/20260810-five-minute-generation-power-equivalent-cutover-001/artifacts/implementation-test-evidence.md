# Implementation Test Evidence

Status: `PASS — focused reopened-package implementation gates`

Evidence mode: `Ran`

Ran from `/home/workdir/openWEPP` on the reopened implementation diff:

- Transaction and runner failure regressions cover existing-target sentinel
  preservation, day-2 source failure, forced WAT5 close failure, forced
  mid-publication rollback, missing-manifest rollback, and successful
  output-set/manifest commit. The forced-close runner receipt is nextest
  `a224551a-a63c-48df-8e83-85fd23afd236`.
- Affected orchestrator/output/runner packages: 762/762, nextest
  `c038158b-206b-447c-a90f-ac6fd20a9022`.
- Named peak/WAT5 contract, property, storage-aware typed roundtrip, and
  HBP-routing exclusion targets: 14/14, nextest
  `beb9ab04-854d-413e-8b1f-ed6ac0d0a544`.
- Advisory/direct-authority policy contract: 7/7, nextest
  `29536d97-dca5-441b-a5c4-04326ee4a8ed`.
- Required-suite anti-evasion contract: 3/3, nextest
  `e4407957-0489-48d3-9b70-8999cd06de87`; shell anti-evasion guard PASS.
- Exact-worktree A0 admission: 43 contracts, 13 reopened science surfaces,
  fingerprint `134c65ccfe96425cbbfbc822cf6c493a2993e952167fd2f85c24b24ff996c7a4`.
- Package feasibility tooling: 5/5 in 0.73 seconds.
- Affected-crate Clippy with all targets/features and `-D warnings`: PASS.

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
