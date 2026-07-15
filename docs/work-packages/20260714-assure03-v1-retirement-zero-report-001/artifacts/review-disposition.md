# ASSURE-03 Review Finding Disposition

Status: PASS — all review and verification findings accepted, remediated, and
independently verified

Evidence class: Static + Ran

Disposition owner: parent execution agent

## Review A

| Finding | Disposition | Remediation and evidence |
| --- | --- | --- |
| `A-001` — eight removal actions misstated | Accepted; remediated | The eight deleted compiler rows now say `remove`. The recovery test enforces both directions: every `remove` path is absent and every `preserve-or-revise` path is a current file, in addition to recovering and hashing all 51 frozen blobs. |
| `A-002` — stale gate/living records | Accepted; remediated in stages and verified | Package progress and evidence statuses distinguish the historical attempts, renewed r4 terminal run, terminal verification, and final closure. `gate-results.md` dispositions every package gate and separates skip-stability transition evidence from release qualification. |

## Review B

| Finding | Disposition | Remediation and evidence |
| --- | --- | --- |
| `B-001` — candidate-named failure upload | Accepted; remediated | Workflow preflight precedes candidate-directory creation. Candidate upload requires `success()`; failure evidence uses `openwepp-release-failure-evidence-*`. The workflow contract test enforces ordering, names, and conditions. |
| `B-002` — preflight symlink evasions | Accepted; remediated | Marker admission rejects `-e` or `-L`; retired directories reject any nested file, symlink, or special entry. Real copied aggregate-script tests prove dangling marker and nested retired symlinks fail before release-directory creation. |
| `B-003` — snapshot-root symlink escape | Accepted; remediated | Snapshot admission uses `symlink_metadata`, rejects an ID symlink/non-directory, canonicalizes and confines a target, scans descendants before reading, and tests both a complete external target and a descendant-file symlink. |
| `B-004` — skip-stability evidence overclaim | Accepted; remediated by control and claim correction | Candidate workflow publication now requires successful separately bound stability as well as validation, preflight, and assembly. `--skip-stability` evidence is explicitly transition-route verification, not a conformant candidate, release qualification, or production release. The complete-candidate evidence bundle remains a future release execution obligation, not an ASSURE-03 closure claim. |
| `B-005` — recovery actions | Accepted; remediated with `A-001` | Same manifest and bidirectional test correction. |
| `B-006` — stale gate metadata | Accepted; remediated with `A-002` | Same living-record and per-gate correction. |

## Focused Remediation Evidence

- `cargo nextest run --profile quick --test assurance_dossier_build_contract`:
  terminal focused remediation run `76c752cb-aff2-4135-86e1-fe3d439aba05`,
  13/13 passed after decomposing the symlink-preflight test below the enforced
  100-line function limit and rejecting Unix special entries.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `bash -n` on all three affected assurance/release scripts: PASS.
- `git diff --check`: PASS.

The renewed r4 terminal aggregates passed after `VB-001` remediation. Each ran
all 1,974 tests, and each fresh adjudicated CRAP acquisition was closure
eligible at threshold 30 with `raw/adjudicated/actionable = 2/2/0` across 13
touched production files. Exact executed identities and the source-freeze proof
are in `heavy-gate-runner.md`. The independently checked r3 post-heavy delta
record remains historical evidence and is superseded as the terminal identity
by r4.

The parent audit also found that the shared exact-file scanner ignored Unix
special entries. This adjacent fail-closed defect was corrected before the
terminal rerun: public-output and snapshot scans now reject unsupported entry
types, with a Unix-socket regression test. It does not expand the package's
publication or scientific scope.

## Terminal Verification Findings

| Finding | Disposition | Remediation and evidence |
| --- | --- | --- |
| `VA-001` — post-heavy package/prompt metadata delta | Accepted; closed by bounded audit | `post-heavy-bounded-delta-audit.md` exactly reconstructs the executed heavy manifest from two strict in-memory reversals, proves 38 unchanged rows and two governance-only changes, and establishes the then-current manifest. Verification A independently reproduced it. |
| `VA-002` — heavy artifact says “current/final tree” | Accepted; superseded and closed by r4 | The wording correction closed the historical r3 ambiguity. The renewed r4 heavy record now binds the current terminal source identity directly; the bounded r3 audit remains chronology only. |
| `VA-003` — two stale artifact descriptions | Accepted; remediated | The migration inventory now records 13 integration tests, and the `VA-002` row above describes the renewed r4 identity rather than the superseded r3 wording state. Artifact-only correction; no source or gate rerun required. |
| `VB-001` — duplicate-key catalog and retired-root special entry evade preflight | Accepted; remediated | Release preflight now requires the exact typed zero-report catalog SHA-256 and treats any retired root as invalid unless it is a real, non-symlink, completely empty directory. Copied-aggregate tests reject a duplicate-key catalog and a retired-root Unix socket before release-directory creation. Focused run `3143e492-8993-4a68-a8da-119765236e6f` passed 13/13. |
| `VB-002` — terminal identity and stale “11-test” wording | Accepted; remediated | Gate metadata says 13 tests. The renewed r4 freeze and both aggregates now bind the post-`VB-001` source; the earlier bounded delta is retained only as chronology. |

## Terminal Verification

Verification A independently reproduced the r4 source identity, all 51 frozen
blobs/actions, nine retained science-evidence hashes, quantitative snow/SNOTEL
counts and residual, one-file public surface, zero export, active-v1 negative
searches, 112 local links, and focused preservation tests. Its one artifact-
reconciliation finding, `VA-003`, was accepted and passed a narrow recheck.

Verification B independently reproduced the r4 source identity, ran all 13
focused contract tests, exercised the exact catalog and preflight evasion
matrix, reconstructed the zero-report snapshot, checked workflow consumers,
verified all 16 retained CRAP checksums, and found no new issue. Final verdicts:
Verification A **PASS**; Verification B **PASS**. Neither verifier edited the
repository.

After closure-only package, roadmap, catalog, and prompt-archival edits, the
bounded delta audit found exactly ten changed manifest rows and no protected
implementation/public change. Both verifiers independently reproduced current
manifest `1178d3b69e83a4e612bedb94f038dce0dd7d18074c251bcb27775d870d407bd7`,
the ten-row classification, prompt inverse, protected hashes, retained CRAP
identity, and claim boundaries. Final closure-tree verdicts remain **PASS**.
