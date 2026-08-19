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

## First frozen-review correction pass (2026-08-19)

The three reviews of `ae52487c099c5816a4bd2df2ea0d40af3a53d39e`
returned FAIL. Accepted findings were corrected before a new freeze:

- fresh interval-24 continuation now reconstructs the prepared day only from
  admitted GSI/forcing receipts and installs admitted staged GSI/cursor owners;
- complete live committed-owner canonical bytes, including configurations,
  GSI, and cursor, are checked unchanged on every poison;
- forcing admission joins run, destination, WB14, CO2, reference height, GSI,
  source climate, and receipt digests;
- real nonempty outgoing cross-midnight carry is custody-joined to the ending
  cursor and has a recomputed-digest parcel-omission poison;
- topology identity now binds ordered lanes, OFE/tile/WB14 destinations,
  LSE tiles, and soil-thermal layer maps;
- GSI history domains/equivalence, scientific surface lineage, BGC identity
  and nested digest, soil restart payload digest, and per-owner omissions are
  executable;
- the governing draft contract was reconciled to the user-required phase
  architecture: beginning GSI/cursor exist only in committed day-beginning;
- generated schema validation accepts all four vectors and rejects phase enum
  and interval-width substitutions.

Ran after correction: reference tests PASS 28/28, warnings-denied Clippy PASS,
direct authority PASS 7/7, forcing PASS 7/7, V10 PASS 3/3, AUTH11 PASS 3/3,
anti-evasion PASS, formatting PASS, and diff hygiene PASS. Exact-current
reviews remain undispatched until the replacement commit is frozen.

## Second frozen-review correction pass (2026-08-19)

The serialization/GSI-forcing review of
`e4e6d32cfb546c4db8a2fd18a2f980bf5a9e17d5` returned FAIL on three generated
artifact defects. The generator now constrains only the checkpoint phase's
`next_interval_index` to 1..47 while retaining the nested surface-continuation
0..47 domain, does not infer topology/configuration array cardinalities from
the four examples, explicitly retains 48 forcing intervals, and classifies
`DirectErosionDownstreamOperands.publication` as a persisted explicit DTO.
The ledger and schema were regenerated; the focused schema test now executes
interval-zero rejection and non-fixture lane/destination cardinality acceptance.

Ran after correction: reference tests PASS 28/28, warnings-denied Clippy PASS,
direct authority PASS 7/7, forcing PASS 7/7, V10 PASS 3/3, AUTH11 PASS 3/3,
anti-evasion PASS, and diff hygiene PASS. A new exact commit and all three
exact-current authority reviews are required before terminal verification.
