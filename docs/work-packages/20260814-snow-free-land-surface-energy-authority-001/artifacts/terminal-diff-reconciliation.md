# Terminal Diff Reconciliation

Evidence class: `Static` and `Ran` as labeled.

## Frozen identity

- Campaign base and current parent commit before this child is committed:
  `0db1960129ad4f8fc4e292b20574dfe7229d5fe1`.
- Branch: local `main`, one commit ahead of the older `origin/main` at intake.
- The worktree intentionally contains the campaign coordinator, all four child
  scaffolds, and the complete Child-1 authority release candidate.
- No reset, pull, rebase, branch creation, push, deployment, activation, or
  production cutover occurred.

## Exact diff scope

Static: the tracked diff contains nine files:

- `docs/ROADMAP.md`;
- the LSE, vegetation, and vegetation-transaction science contracts;
- the science-contract index;
- the work-package catalog;
- the annotated bibliography;
- the LSE and vegetation authority-contract test targets.

Static: 111 intended untracked files are grouped as follows:

| Surface | Files | Purpose |
| --- | ---: | --- |
| Child-1 LSE authority package | 73 | Authority, fixtures, schemas, evidence, retained reviews and verifications, archived prompt, and gate logs |
| Campaign coordinator | 16 | Intake, architecture freeze, dependency ordering, and campaign lifecycle |
| Child-2 scaffold | 6 | Real-hydrology arbitration package intake only |
| Child-3 scaffold | 6 | LSE runtime package intake only |
| Child-4 scaffold | 6 | Real-consumer package intake only |
| Vendored references | 4 | Rights-reviewed load-bearing scientific sources |

No file below `crates/`, no Cargo manifest or lockfile, no runner selector, no
production runtime dispatch, and no production output publication path is in
the Child-1 diff. Child 1 therefore releases implementation authority only.

## Frozen authority bytes

| Surface | SHA-256 |
| --- | --- |
| LSE V1 definition | `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f` |
| C3 woody V8 definition | `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b` |
| Independent calculator | `1156fa88a6d7e4dd98f6dd70fe5b891f69e0b6825694179ac4d687a38907c859` |
| Joint canopy-ground core | `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5` |
| Independent vectors | `7b6a303ae434ca6ad59c7082ebf486300214427d6abe20c36bfaa9b8cbdab91c` |
| Configuration schema | `6499b98cc1e25f1379bc0ad6052a7536e20c4bfbb9335f9ba5c8de191ae2f009` |
| Coupled transaction schema | `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f` |
| Diagnostics schema | `41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c` |
| Forcing schema | `f1fb785e9e582ae9e20eac4b5f44fa2b5f0651f8535d0972520dbfff3d926b55` |
| State schema | `91243e4087fa2c4775cb3629fe14c64379def4977d3c54a72348ac56d5fa4ee8` |
| Water protocol schema | `2e5ade752deb0751bb31222da5d8fe3f6a1e5fbee407e20780fa26242a7afd07` |

The immutable definitions bind canonical-section digests rather than whole-file
contract hashes. V1--V7 historical model-definition bytes remain protected by
the vegetation authority suite; V8 is a successor identity and does not mutate
V7.

## Review and finding reconciliation

Static: all failed `A`, `A2`--`A6`, `OWN`, and `OWN2`--`OWN6` reviews remain in
the package as immutable historical evidence. The terminal science/numerics and
hydrology/ownership reviews both report `PASS / GO` against vectors
`7b6a303a...`. `review-finding-disposition.md` identifies the exact correction
and confirmation evidence for every accepted finding; no material finding is
unresolved or silently deferred.

## Executed validation

Ran:

- LSE authority: 7/7 passed;
- vegetation authority plus implementation and AUTH11 focused population:
  46/46 passed in the final campaign validation set;
- all 11 typed diagnostics and 76 executable poisons passed;
- science admission, authority anti-evasion, and all three affected
  science-contract unit gates passed;
- workspace strict Clippy passed after retaining and correcting one test-only
  readability/line-count failure;
- full workspace nextest passed 2,674/2,674 with 33 skipped and 34 slow tests
  under `TMPDIR=/tmp/ow-lse-full`;
- workspace doctest invocation and dependency policy passed;
- package Markdown passed for 44/44 files;
- the full documentation scan covered 20,152 files and reproduced exactly the
  frozen 15 unrelated broken links with zero warnings, no new broken link, and
  no changed pre-existing target;
- formatting and diff hygiene passed.

The requested comparator runner was delegated twice. Both attempts ended in
service-capacity infrastructure failure after retaining partial logs. The
parent then executed only the unfinished heavy commands. These attempts remain
recorded in `gate-results.md`; they are not represented as comparator PASS.

## Terminal boundary

This exact diff may close only as:

`COMPLETE / snow-free land-surface-energy implementation authority released`

It authorizes no runtime implementation by itself, selector change, production
cutover, output publication, calibration, or empirical-validation claim. Both
independent terminal verifiers returned `PASS / GO` after the complete 57/57
finding inventory was reconciled. The kickoff prompt was then archived
byte-for-byte at SHA-256 `46d80862...`.
