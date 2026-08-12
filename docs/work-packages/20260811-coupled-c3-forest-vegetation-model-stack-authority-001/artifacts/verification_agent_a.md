# Independent Terminal Verification A

Status: `PASS / no unresolved material findings`

Evidence mode: `Static + Ran`

Verification instant: `2026-08-11`, after the parent declared the package
science, source-identity, review, validation, and terminal lifecycle surfaces
frozen.

## Exact Verified Snapshot

- Base/HEAD: `669aafb60df3ac4eeed2661cc4db4ad33f3f2265` on `main`.
- `git status --porcelain=v1` SHA-256:
  `496a451812655169952e94f80615ab9a3b40bc2fd2f2b9798b6a17d2b66a88d9`.
- Tracked binary-diff SHA-256:
  `c7dc734569a78d06795ec38034e10bcb8d73b4846ca844a204ea135ec4385d05`.
- Sorted 132-file SHA-256 manifest over the owned package/core/lifecycle
  surface, excluding both verifier-owned outputs and ignored `__pycache__`:
  `6acca709617ae1c5a681f0762861c053fb12523b9fe78952590f6af676299a59`.

| Load-bearing surface | SHA-256 |
|---|---|
| `SC-VEGETATION-001.md` | `2c8cc8322ce8c4404e212f3a12f7f2aea7547ec8f23ab3eafc5e53f7672127e7` |
| `SC-BIOGEOCHEM-001.md` | `6cfd2143f9941613e6f6324d2790f88773c9b9eafa1ab8cad72e5a95df6794b4` |
| model-definition JSON | `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157` |
| independent calculator | `ac8caf95e2b8bccadc528e168d0e466504bca88c15e86b7bfba89438f4ec13e8` |
| focused Rust contract test | `7aeb2201bec9fdf078b114ceb569ed6fad7b3d8f9d03c76c1fd21647dcb658b3` |
| gate results | `de0403a10f74fac562725ae11e3cd9aa70dc85a5905b19e374a30cb7ca6ca9ae` |
| final Review A | `eb7a930198a49f8fd545b313d208f8da772e39c50fc1426f3afabbc93d3b66f9` |
| final Review B | `7fa6940f8108d9c7d779b27c96ca753fc0a0c3403bdd217460d804af789a90f6` |
| terminal Verification B closure recheck | `751f6cc70cceedbc8575c1921707c50e5d498270d9889e08756d20131d8f5895` |
| archived kickoff prompt | `7546dc99ddbbca94497b9b772f01ec94c392b36fe93156bf204a09e63e3cc3ee` |

## Independently Ran

- `.venv/bin/python .../artifacts/reference_calculator.py`: exit `0`,
  `"all_pass": true`; output SHA-256
  `bd180d63e8d4e3ccae78fbeec308ddd27024db9c466e1d8eef47656d6df0f368`.
- `cargo nextest run --test vegetation_boundary_authority_contract --profile
  quick`: 12 passed, 0 skipped; run ID
  `ba741efe-813b-4d86-a474-bd5844d51f1f`.
- `check_science_contract_admission.sh --base-ref 669aafb... --worktree`:
  `A0_ADMITTED`, authority SHA-256
  `4a21ecc5fc1c26f8b4aed159d48f2274c4eaf9469468360761f4cd466cd37d46`.
- Both `check_sc_unit_compliance.sh` invocations: PASS.
- `markdown-doc lint` on the complete science-contract tree: 62 files, zero
  errors/warnings.
- `markdown-doc lint` on this package before this verifier write: 39 files,
  zero errors/warnings.
- `git diff --check`: PASS.
- `cargo nextest list` inventory comparison: quick `2349`, full `2398`,
  quick-only `0`, full-only `49`; the accepted full run covers every quick
  test plus 49 additional tests.

## Gate-Legitimacy Determination

PASS. I inspected the retained command receipts rather than relying on the
summary prose. Comparator command 38 used external scratch
`/home/workdir/openwepp-full-terminal.sFYG0b`, exited `0`, and reached natural
completion: `2398/2398` tests passed, 33 excluded by the declared full profile.
The scratch directory was removed. Command 37 independently passed the former
ENOSPC-affected CQR test using a second external scratch directory, also
removed.

Historical command 24 used an invalid in-checkout `TMPDIR`. It is now clearly
marked non-compliant and is not acceptance evidence. The fresh clean command
38 removes any dependency on retry adjudication. The workspace all-targets
Clippy attempt exposed one base-identical lint in an unchanged land-surface
test; terminal exact-diff reconciliation selects warnings-denied Clippy on the
only changed Rust target, which passed in commands 32 and 47. Full-workspace
correctness is independently supplied by command 38. Commands 39--50 reran the
post-correction A0, unit, documentation, formatting, focused Clippy/test,
calculator, and diff checks; all passed.

## Hard-Release Audit

| Criterion | Determination |
|---|---|
| one coherent stack | PASS: `OPENWEPP_C3_WOODY_V1`; alternatives are explicitly rejected, not runtime switches |
| canonical authority | PASS: `SC-VEGETATION-001@5` and `SC-BIOGEOCHEM-001@1` are consistently `approved/active` in frontmatter, body, and index |
| digest binding | PASS: the contract binds the current model-definition digest; the focused test recomputes all selected contract-section and whole-BGC hashes |
| source custody | PASS: binding citations have local exact bytes, checksums, locators, rights, and bibliography mappings |
| corrected de Pury identity | PASS: contract and bibliography use DOI `10.1111/j.1365-3040.1997.00094.x`; the stale DOI is absent |
| caller schema/state | PASS: fields are required and typed, the complete initial tissue C/N state is enumerated, and hidden defaults/consumed aliases fail |
| coupled causality | PASS: photosynthesis, Medlyn stomata, energy, transpiration, hydraulics, C gain, allocation, future LAI/root state, turnover, and litter/CWD share one admitted state machine |
| mixed strata | PASS: evergreen/deciduous strata retain identity, topology, optics, state, roots, parameters, and transactions; no averaged mixed row exists |
| ownership/atomicity | PASS: vegetation does not mutate soil water; water and N use request/authorization/finalized-use/atomic-commit boundaries with exact receiver custody |
| conservation evidence | PASS: independent calculator and Rust assertions cover water, energy, C, N, dry material, area/rate aliases, double counts, and byte-identical rollback |
| native forest ET | PASS: canopy, wet surface, and forest floor retain independent operands; PMET donation is prohibited and poisoned |
| exclusions | PASS: C4, crops, nonvascular strata, canopy snow, recruitment/succession, fire/catastrophic disturbance, and unsupported aerodynamic branches fail typed |
| AUTH/GAP closure | PASS: all 16 `AUTH-RHEC-001..016` rows are dispositioned; `012` is the required snow deferral and `013` preserves provenance; all 23 canonical `GAP-VEGETATION-*` rows are reconciled |
| empirical claims | PASS: intent is a calibration-readiness assessment only; `NOT_CALIBRATION_READY` blocks empirical claims without blocking separately scoped implementation authority |
| successor | PASS: the released successor targets the complete coupled state machine; the soil-transformation dependency fails/zeros explicitly and authorizes no temporary nutrient source |
| dual science review | PASS: both final reviewers rechecked the corrected DOI and current contract hashes and report no material finding |

## Rights, Provenance, And Write-Set Audit

The pinned read-only checkouts remain at RHESSysEastCoast
`375c75b1cd2202217651dff43aa113d80b9c1118` and GIS2RHESSys
`6b20883dea7c9fd92f71ec69eaca015ebf6dfe18`; both license files hash to
`4fd4ecf2fd01cf53c99754bcac5a6dbee255a0be0539dd84ffe12e06808374be`.
The five newly binding restricted PDFs recompute to the recorded hashes and
are ignored by `references/copyrighted/**`; none is tracked.

`git diff --name-only` plus non-ignored untracked inspection contains only
package/reference metadata, canonical contracts/index, the one contract test,
lifecycle files, and the two prospective successor packages. No `crates/**`,
production source, runtime selector, activation, deployment, or publication
path is changed. The changed Rust test is 721 lines and the calculator 754,
matching line-count governance and remaining below the warning threshold.

## Finding Disposition

| Finding | Disposition | Final evidence |
|---|---|---|
| `TVA-001` invalid in-checkout ENOSPC retry scratch | `accepted / closed` | command 24 demoted; external commands 37 and clean 38 pass |
| `TVA-002` calibration-readiness intent conflicted with blocked rows | `accepted / closed` | intent now says assessment only, not a closure objective or claim |
| `TVA-003` canonical de Pury DOI disagreed with bibliography | `accepted / closed` | DOI corrected; A0/focused gates and both exact-byte science reviews refreshed |
| `TVA-004` changed-test line count recorded as 722 instead of 721 | `accepted / closed` | current `wc -l` and governance artifact both report 721 |

No finding is rejected, deferred, follow-up, or unresolved.

## Closure-Delta Recheck

PASS. After both verifier artifacts existed, the parent changed only five
lifecycle prose/status surfaces. I inspected the exact final bytes:

| Closure surface | SHA-256 | Determination |
|---|---|---|
| `package.md` | `f2827078477d084c64fc7c0102cfff81390ee17c60bd371423036b22872e3b4c` | status, progress, and outcome truthfully record complete authority release and no runtime change |
| `artifacts/final-disposition.md` | `82fbc405d1fa8e33fcfa46b49a1b2619143f1c525dbd44865e04b29abf4aebfd` | accurately names both verifier PASS results and the clean 2,398/2,398 full run |
| `artifacts/pre-implementation-intent.md` | `f6655bc77a93fffa75fd613dc765fd002078fe739b71613da03012291447c725` | `fulfilled` is limited to implementation authority and retains all no-runtime/no-empirical-claim boundaries |
| `artifacts/worker-handoff.md` | `950b8d3ebb100a4428912aab3f51fa7fe7a5e9fbbf7c2bbd4e7383d1757726af` | releases only the whole-state successor and preserves `NOT_CALIBRATION_READY` / `NOT_VALIDATED` |
| work-package catalog | `29dc5149a5501ed2694edd9477474372d602e062409ea0bafc81a0db2ec1eb01` | changes this package from `ACTIVE` to `COMPLETE` without altering successor scope |

These edits change no canonical contract, model-definition digest, source
identity, calculator, test, review, gate receipt, successor package, or
restricted-reference byte. Package Markdown lint, catalog Markdown lint, and
`git diff --check` pass on the closure delta. The archived kickoff prompt still
hashes to `7546dc99ddbbca94497b9b772f01ec94c392b36fe93156bf204a09e63e3cc3ee`.
No heavy rerun is required for prose-only lifecycle closure.

The package-level README now consistently reports `complete /
implementation-authority released` (SHA-256
`c3d7ac96df361a018c7c9b8d51a530db7745a6e0a04c82e1d09af20b5962bfa4`).
The mutable active kickoff copy was removed after both original terminal
verifications; `prompts/active/README.md` now states that no active prompt
remains (SHA-256
`bbb7055f49b585f58c0b0456de3a0575d933ffc9ea96ab11d7030b3710194b0c`).
The original byte-preserved archive remains present at its unchanged digest.
This prompt-lifecycle correction is truthful and reduces the owned manifest by
one file without changing authority content.

## Verdict

`PASS`. The exact verified science/core and terminal lifecycle bytes meet the
kickoff hard release criteria for `complete / implementation-authority
released`. This verdict
authorizes no production implementation, runtime activation, calibration,
validation, transferability, deployment, publication, push, or external
message. The final `COMPLETE` markers are consistent with both independent
verifier PASS artifacts and do not alter the verified science, source,
contract, test, successor, or gate bytes above.
