# Current continuation — executed HOLD
Status authority: final-disposition.md. Adoption is not complete.
Authorization: prompts/active/execute.md; local commits only, no push.
Both independent terminal verifiers completed and accept truthful HOLD delivery.

## Canonical state
Original LSE version 31 is restored at its retained canonical path, 264863 bytes:
`7a002bcac2ad640716b52f4bd59326f241d5fd3084d26e56f6a703eb496ed61d`.
Legacy whole-contract reading remains required. All seven original Rust consumers
are restored, with no production or contract-science changes. No candidate chapter
directory remains at the canonical path. A minimal format-status correction records
implemented structural support and pilot HOLD; it changes no grammar or adoption gate.
Unrelated `$pkg/` and `tmp/` remain untouched.

## Checker and retained candidate
Canonical tools/sc_contract_directory.py is shared by the binding, unit and A0
consumers. It validates actual definitions, 32 coverage keys, declared membership,
confined paths, anchors, provenance, aliases and deferred status. It is not a semantic
engine. Candidate v32, its 14 chapters, seven reconciled Rust consumers, test bridge
and checker snapshot remain under artifacts/candidate-tree as immutable evidence.
Baseline: b932db101cce07d0860b45b5ecaa8ddb7f455b58. Candidate commit: 5f76475dd.
Restored substantive HOLD cut: 3ec624453. Reviewed candidate set hash:
`99ca67fb97351a7eaf495e89fc54720c314795acfbe7bcdc9c3ccab47a8596a8`.
source-manifest.json, candidate-manifest.json and candidate-recovery.json bind exact
inputs. Compare them with actual state before reuse; mismatch requires reconciliation.

## Exact invocations
From /workdir/openWEPP:
```sh
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
```
This checks restored LEGACY authority: PASS with 16 BEI rows is not directory adoption.
To reconstruct the candidate in a new external location:
```sh
.venv/bin/python docs/work-packages/20260907-directory-contract-checker-lse-adoption-001/artifacts/restore_candidate.py /tmp/NEW-UNUSED-DIRECTORY
```
Use the repository .venv Python on that tree's tools/check_sc_binding_exposure.py
with --strict and the reconstructed absolute entry path; run its unit checker with
--path pointing to the same entry. Recovery verifies 40 files before creating a new
external fixture and rejects existing/repository outputs. This is not a full Cargo
workspace or executable reconstruction. Full Rust reproduction needs the complete
named baseline plus the archived overlay, local .venv and supported Nix environment.
Do not run the one-time build_candidate.py in the canonical checkout to reactivate it.

## Evidence and blockers
Pytest: 83 PASS. Candidate strict: 17 BEI/100 definitions, PASS; unit/A0 PASS.
Restored legacy strict/unit/A0 PASS. Selected Rust: 129 run, 111 PASS, 18 FAIL, exit100;
baseline/corrected candidate/restored final have identical names and panic reasons.
No-new-failure evidence is not a passing gate. Formatting PASS; Clippy -D warnings
FAIL with 1136 errors in unchanged production dependencies. Actual heavy-runner quota
unavailability is recorded; supported parent Nix execution was used openly.
No physics campaign was selected merely for Markdown relocation.

The expected-red v31 production seam and frozen external runtime identity cannot be
resolved by permitted loader/reference changes. This request authorizes neither
production implementation nor weakened predicates. Required gates remain unmet.
Both reviewers accept scientific preservation and corrected bounded reading answers;
initial failures remain recorded. Identity alone reduces selected LSE bytes by 62.25%;
other tasks grow. Context reports measure source inventories/ranges, not runtime
context, tokens or quota. They bind archived candidate bytes from before restoration.

## Remaining action and reopening boundary
All safe work for this delivery is finished; the required adoption gates remain unmet.
Do not push. Adoption requires appropriately authorized blocker resolution and all
frozen gates passing; owner-paused EXP-R remains paused. Unrelated
authorized architecture is not blocked by this pilot. Retain the earlier verified
custody bundle /tmp/openwepp-lse-candidate-before-baseline-20260907 until explicit
owner/audit release; its manifest is separately identified in gate-results.md.
