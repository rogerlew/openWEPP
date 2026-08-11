# Rust QA Review

Status: `PASS — reopened Rust implementation QA at exact clean SHA`

Evidence mode: `Static + Ran`

## Reopened final review — 2026-08-11

Exact clean SHA `e7851f1a626c1e6e77d64c2618f26bb772787b36` passed with no
High or Medium QA blocker. The review verified explicit manifest-construction
rollback composition with retained backup paths; split typed domain/closure
errors; aggregate-hour no-target failure; positive-year, advancing-calendar,
nonfinite, and overflow guards; correct hardlink/unlink state; real WB14
positive-storage Parquet closure; exact performance identity; and no observed
transaction leftovers. Low debt remains limited to post-commit backup-cleanup
reporting, standalone-writer temporary-unlink semantics, and numeric-column
test lookup.

The historical review below is retained as superseded context.

Review identity: `HEAD c9f28a7dbe7adf69d8e6d54ebd8da57568af5552`
plus the current dirty WAT5 implementation and terminal A0 policy correction
inspected and tested on 2026-08-10.

## Findings

None.

## Terminal A0 Correction Review

The final delta does not alter runtime or output implementation. Its Rust
changes are policy/contract assertions in
`tests/integration/advisory_linter_authority_contract.rs` and an authority
retarget in `tests/integration/subhourly_generation_contract.rs`.

The assertions are meaningful and fail closed:

- the direct impact-map test requires exactly 17 atomic
  `SC-OUTPUT-WAT5-001` bindings;
- each of the four shared runtime paths must retain exactly the independent
  `SC-PLANT-001` and `SC-OUTPUT-WAT5-001` bindings;
- the compact, schema-validated definition set must include executable A1
  definition `hard-invariant-wat5-runtime-v1`;
- the worktree authority fingerprint must include every parser input,
  including the external-authority input-surface registry;
- the WAT5 contract test preserves unchanged peak invariants
  `INV-WATBAL-102..104` while binding diagnostic behavior and
  `TOL-WAT5-001` to approved, active `SC-OUTPUT-WAT5-001`.

No scoped test, authority mapping, or package evidence retains a stale
`WATBAL-105` expectation. Exact worktree admission independently returned:

```text
A0_ADMITTED contracts=43 science_surfaces=17 base=c9f28a7d... head=WORKTREE authority_sha256=6f95845b5065e9134cded858e69ed359b2e42bd32318f800f87801d4088d1298
```

The retained final focused receipts are 12/12 at nextest `c71d7b95...` and
independent 12/12 at nextest
`3a63cdad-31e5-41b9-8cf4-623204765075`.

## Prior Finding Disposition

### `WAT5-RQA-HIGH-003` — `CLOSED`

The prior finding required exact-source real-consumer, source-rejection, and
noninterference evidence after the material WAT5 corrections. The refreshed
release evidence satisfies that requirement:

- The current release target and both p61 manifests identify binary SHA-256
  `f264661135cde810ff4914df80f5aba1e176349af89537794f18187e49bbc85a`;
  the enabled and disabled runs used that same binary.
- Independent hash checks confirmed byte identity for HBP
  (`fd01aead...`), PASS (`e5b8d5ac...`), WAT (`707bdc15...`), and loss JSON
  (`92c88db6...`) between `/home/workdir/openwepp-wat5-terminal/on` and
  `/home/workdir/openwepp-wat5-terminal/off`.
- Independent Parquet inspection confirmed 26 columns, 24 rows, unique global
  bins `0..23`, hours `0..1`, exact 300-second support, correct hour/bin keys,
  and 24/24 null values in each of the three erosion power-equivalent fields.
  The two hourly reconstructed residuals were within floating-point tolerance
  (`-1.78e-15 mm` and `0 mm`); the maximum producer-recorded residual was
  `3.47e-15 mm`.
- The p102 terminal stderr records lane 1/day 1 failure with
  `WAT5-E-001 positive additional supply lacks 300-second timing`; inspection
  found no WAT5 target or WAT5 temporary file.

The earlier implementation findings remain closed: five-minute boundary
replay and continuous Green-Ampt state, global day-bin indexing, exact-positive
and WAT5-E004 guards, typed multi-hour Parquet round trip, recursive
HBP/routing exclusion, and atomic no-replace publication all have focused
regression coverage.

## Exact-current QA Runs

- `cargo fmt --all -- --check`: PASS.
- Orchestrator WAT5 behavior plus frame-layout guard: PASS, 18/18, nextest
  `1aa30dca-55c5-412c-bd89-d7ea66b68fb5`.
- Output contract/path/atomicity package: PASS, 23/23, nextest
  `ee3818a4-acc8-4b4c-b9ec-90504ce84788`.
- Five named peak/WAT5 contract, property, typed round-trip, and HBP/routing
  exclusion binaries: PASS, 13/13, nextest
  `a20cb849-bd7c-44fc-9663-a053bb7337d9`.
- Final advisory-authority, WAT5 contract, HBP exclusion, and authority-input
  fingerprint assertions: PASS, 12/12, nextest `c71d7b95...`; independent
  PASS, 12/12, nextest `3a63cdad-31e5-41b9-8cf4-623204765075`.
- Exact worktree science-contract admission: PASS, 43 contracts and 17 science
  surfaces, authority SHA-256 `6f95845b5065e9134cded858e69ed359b2e42bd32318f800f87801d4088d1298`.
- Authority-suite anti-evasion shell guard: PASS.
- Required-suite obligation guards: PASS, 3/3, nextest
  `579bd206-e0fb-4beb-a991-ae5b2b7f7c4f`.
- Affected packages, all targets/features, Clippy with `-D warnings`: PASS.
- Final changed Rust contract targets, Clippy with all features and
  `-D warnings`: PASS.
- `cargo deny check`: PASS for advisories, bans, licenses, and sources; it
  emitted only the existing unmatched `MIT-0` allowance warning.

## Non-blocking Debt / Follow-ups

- The exact line-count artifact records three preexisting multi-authority
  WARN files (`03_tests.rs` at 2,905 lines, `runoff.rs` at 2,869, and
  `00_core_frames.rs` at 2,713) with concrete split directions. None reaches
  the 3,000-line blocker; avoid further cross-authority growth.
- `HillslopeWatSubhourlyRow` and its writer are public construction APIs while
  domain validation currently occurs in the production generator. If external
  callers are introduced, make invalid-row construction impossible or add
  writer-boundary validation for finite/nonnegative values and key/time
  relationships.

## Package Gate State

The final post-A0 exact terminal workspace run passes 2,380/2,380 with 33
ordinary skips, nextest `b920db77-070f-4686-a7bf-2e2727094374`; post-A0
workspace doctests pass with zero failures. The superseded pre-repair
2,379-test receipt is not used for closure. The current parser-registry A0
delta is covered by the exact admission, focused contract, anti-evasion,
formatting, and Clippy checks above. Remaining lifecycle receipt/disposition
updates are parent-owned package administration, not an implementation QA
finding.

## QA Pass Statement

The current WAT5 Rust implementation is readable and cohesive at its new
module boundaries, uses typed fail-closed errors, has robust focused behavioral
and publication coverage, and passes formatting, warnings-denied lint,
dependency-policy, real-consumer, source-rejection, closure, and
noninterference checks. The terminal A0 correction adds meaningful,
warnings-clean fail-closed authority assertions without changing runtime
behavior. Rust QA passes with no blocking findings.
