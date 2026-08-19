# Gate results

Status: `intake only / no restart validation yet`

Ran at the original draft checkpoint: the existing
`direct_hydrology_restart_authority_contract` reported 3/3 PASS. These are
draft authority documentation/schema-shape checks only. They do not execute
canonical checkpoint serialization, restoration, continuation, rollback, real
vectors, or the typed poison matrix and therefore are not restart validation.

Ran at superseding intake: branch, clean-tree, exact `HEAD`, and `origin/main`
checks PASS at `1cac432a4a5d2a0de87122bd68b69ab83cffe21a`.

All implementation, focused, authority-release, and terminal gates remain
`NOT RUN` for this resumed execution until their corresponding changes exist.

## HOLD-remediation intake and primitive reference increment

Ran at starting commit `bb3cc3a0ed...`:

- branch/ancestry/origin inventory: PASS;
- clean worktree and `git diff --check`: PASS;
- instruction discovery over every declared path: PASS;
- package-local reference crate unit tests: PASS, 6/6;
- package-local reference crate all-target Clippy with warnings denied: PASS;
- strict canonical parser tests cover reordered bytes, whitespace, duplicate
  members, and unknown members;
- primitive tests cover signed zero, exact lowercase widths, u32 overflow,
  negative day indices, and interval 48 rejection;
- the first actual runtime mapping (`DirectWaterState`) exhaustively
  destructures all six fields and round-trips their bits exactly.

Disposition remains `HOLD / REMEDIATION IN PROGRESS`. The remaining runtime
owners, artifact regeneration, complete poison matrix, and reviews are not yet
passed; production restart implementation remains forbidden.

## Sustained authority completion pass (2026-08-18)

Ran:

- package-local reference tests: PASS, 25/25;
- package-local all-target warnings-denied Clippy: PASS;
- artifact generator: PASS; four typed vectors, schema, manifest, mapping
  metadata, ledger, and poison inventory regenerated;
- focused `direct_hydrology_restart_authority_contract`: PASS, 5/5;
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS;
- `auth11_required_suite_obligation_guards_contract`: PASS, 3/3;
- `git diff --check`: PASS before independent review dispatch.

Static and executable reference evidence covers exhaustive frame
destructuring, projection/restoration equivalence, optional canonical winter
carries, interval-24 fresh-object continuation, exact abort-to-day-beginning,
nested and outer digest rejection, cursor/topology/configuration/owner joins,
and unchanged live bytes on poison. Three authority reviews are in progress;
terminal verification remains intentionally undispatched.

Independent serialization/GSI-forcing review: FAIL. The reviewer found that
the generic owner envelope and one-byte payload fixtures are not actual typed
GSI/forcing/V10/LSE/soil/BGC authority; the InProgressDay DTO omits accepted
GSI, staged GSI, beginning/ending cursor, and full validated forcing receipts;
continuation/abort evidence is synthetic; the schema is descriptor-only; and
several named poison categories are inventory strings rather than typed
admission executions. These are closure blockers and supersede the preliminary
PASS labels above. Terminal verification remains blocked pending correction
and exact-current re-review.

## Exact `89410d7d` continuation intake

Ran: required HEAD, branch/origin synchronization, clean-tree, diff-hygiene,
and complete instruction-discovery checks PASS. See
`exact-89410d7d-intake.md`.

Static: prior failed reviews are accepted as remediation requirements. Owner
lineages are separated in `owner-lineage-domains.md`. No prior finding is
marked corrected by this prose reconciliation; all remain OPEN until their
named executable evidence passes. Production restart remains forbidden.

## Exact-current typed-owner remediation (2026-08-18)

Ran on the uncommitted remediation diff based on `89410d7d`:

- package-local reference tests: PASS, 27/27;
- package-local all-target warnings-denied Clippy: PASS;
- deterministic typed artifact generator: PASS;
- focused direct-hydrology restart authority: PASS, 5/5;
- focused snow-free half-hour forcing adapter: PASS, 7/7;
- focused V10 nighttime authority: PASS, 3/3;
- authority-suite anti-evasion guard: PASS;
- AUTH11 required-suite obligation guards: PASS, 3/3;
- workspace formatting and diff hygiene: PASS.

The regenerated authority contains typed GSI, forcing, V10, LSE-V2,
direct-hydrology, soil-thermal, and BGC owners; a true phase-union schema; four
repository-backed vectors; generated field metadata/ledger; real interval-24
fresh-object continuation; exact day-beginning abort; and typed poison
admission with actual live bytes unchanged. Independent exact-commit reviews
remain intentionally undispatched until the remediation commit is frozen.
