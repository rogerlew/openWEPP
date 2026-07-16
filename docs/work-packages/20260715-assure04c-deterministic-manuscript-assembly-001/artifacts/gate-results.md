# ASSURE-04C Gate Results

Status: TERMINAL PASS

Evidence class: Ran where individual rows are populated

| Gate | State | Evidence |
| --- | --- | --- |
| Contract-derived assembly tests | PASS | Preimplementation compile failed on 19 absent assembly APIs as intended; twice-remediated assembly suite passes 9/9 inside the 31/31 assurance aggregate |
| Focused assurance tests | PASS | Source, planner, and assembly suites pass 31/31 in terminal verifier B's independent renewal; strict workspace/all-target Clippy with warnings denied passes. |
| Quick workspace | PASS | Terminal verifier B's independent renewal passes 1,926/1,926 selected tests; run `d490f68f-2eff-4d33-a25d-bc5aef8d99e5`. |
| Documentation | PASS | Final closeout `markdown-doc lint` passes 30 files with zero errors/warnings; `markdown-doc validate` passes the same 30 files with zero errors; `git diff --check` passes. |
| Full workspace closure | PASS | Independent fresh restart: formatting and strict workspace/all-target Clippy pass; full Nextest run `2344d4b1-ec78-40e0-8d5c-5474cdb438ee` passes 2,011/2,011 with zero failures, 3 skipped, 5 slow, and 186 binaries; JUnit SHA-256 `5b1b417542ebe3363e512626b5a139d7ca9815789a12e0016338ad4c1369768f`; `cargo deny check` passes all categories. See `heavy-gate-runner.md`. |
| Fresh adjudicated CRAP | PASS | Fresh run against frozen base `e704f0202278ebb86c6a8c667caf73d599be04ab`: raw 2, adjudicated 2, actionable 0, touched production files 7. Touched-file maxima: `cli.rs` 19, `error.rs` 20, `lib.rs` no measurable row, `v2.rs` 30, `assembly.rs` 25, `confined.rs` 15.3586, and `planner.rs` 15.2767. The 226-source manifest `ed4213f8be4d1921740658865f4f3ec12cc1804b4c8d7e64ff16d9d7ae9c5d5e` is byte-identical before, after, and at final check. |
| Dual review and disposition | PASS | All findings were accepted and remediated; after two technical renewals and a governance-only confirmation, both independent reviewers returned PASS with no remaining blocker |
| Dual terminal verification | PASS | Two independent read-only verifiers audited every acceptance row and primary evidence. Verifier A returned PASS with no blocker. Verifier B returned PASS and independently renewed 31/31 focused tests, quick profile 1,926/1,926 run `d490f68f-2eff-4d33-a25d-bc5aef8d99e5`, retained staging check, actual renderer reproduction, protected hashes, and documentation validation. See `terminal-verification-a.md` and `terminal-verification-b.md`. |

## Rendered Consumer Evidence

The retained staging tree is
`artifacts/retained-staging/usersum/assurance/reports/linear-groundwater-reservoir-recurrence/0.1.0/`.
Real CLI `build --all --staging-root ...` and read-only `check --all
--staging-root ...` both pass with one report and the same ten output
identities. The report SHA-256 is
`52083380371cd6b32b10af8453f610601d7f0e3874ff297c735982e6b0629975`;
the supplement is
`5f2f014ac783ad2dc123a1af3baf41c6908ea9137465b28224e5d08b7b5c51bf`.
Both SVGs contain `role="img"`, title, description, monochrome patterns,
persistent text labels, and visible Markdown data alternatives. Every
generated local link resolves inside the retained consumer. The actual
WEPPcloud usersum `cmarkgfm` renderer passes the manuscript, supplement, and
escaped-metadata probe; see `usersum-renderer-proof.md`.

## Protected And Write Boundaries

The four protected public-transition files retain their intake SHA-256 values.
The aggregate sorted `usersum/**` file-hash stream remains
`deb9f2c646aa5eb4ad9e427f8a7cec6ad51e3f9f6b47ffe29d14b4aed4bdcb7a`.
The tracked public builder/checker still report zero reports when
`--staging-root` is absent. Current changed and untracked paths are confined to
the declared write set.
