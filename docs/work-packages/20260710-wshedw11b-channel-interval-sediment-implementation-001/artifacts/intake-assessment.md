# Intake Assessment

Status: `EXECUTED-PASS`

Evidence mode: `Static` plus `Ran` repository/source discovery commands.

## Workspace and instruction intake

Ran from `/home/workdir/openWEPP` on 2026-07-10:

- `git status --short` showed the W11B scaffold as untracked and the already
  scoped roadmap/catalog/W11 handoff linkage as modified. Numerous unrelated
  untracked `artifacts/` paths also exist and are preserved untouched.
- `tools/agents/find-agents --for ...` over every declared write path resolved
  root `AGENTS.md`, `docs/work-packages/AGENTS.md` for package/catalog files,
  `crates/AGENTS.md` for Rust and runner tests, and
  `docs/specifications/science-contracts/AGENTS.md` for canonical authority.
- The package explicitly authorizes the required source-review, review,
  verification, comparator, and heavy-gate subagents.

## Authority confirmation

Static: `SC-ROUTE-001` v53, `INV-ROUTE-015..020`, the W11A handoff, both
Codex amendment dispositions, and the pinned baseline SHA
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70` leave no executor science choice.
No canonical contract amendment is required before implementation.

The held W11 support matrix's `ipeak > 5` fail-closed row is stale. Canonical
`INV-ROUTE-006` and the WS11 addendum authorize `ipeak >= 4` as
Muskingum-Cunge; pinned `wshchr.for:473-572` agrees, while only `ipeak = 5`
selects dynamic coefficient refresh. Canonical v53 governs W11B.

## Defect reproduction and mechanisms

Static:

- `GAP-ROUTE-014-A` is present in
  `routing/01_ws22_ws23_ws26_detachment.rs`: the widening terminal caps `dct`
  but returns the uncapped width and old depth instead of reconstructing
  capped erosion and deriving geometry. The current characterization test locks
  that divergence.
- `GAP-ROUTE-014-B` is present in the same file: the low boundary-shear
  terminal computes class detachment but returns unchanged `depmid`; pinned
  `dcap.for` re-enters incision with `timsh = timpot` and decrements depth.
- `WSHED-W11B-DIRECT-001` is present in production: routed channel wave and
  sediment states are scalar, hourly contributors plus dependency nodes are
  rejected, event routing uses peak fractions, and downstream channels cannot
  consume same-grid per-class egress.

## Seven-gate and envelope assessment

All seven correction gates pass statically: the failures are reproducible,
their mechanisms and owners are named, v53 plus pinned source supplies proximate
authority, typed failures remain protected, each correction has an anti-alias
vector, and the package names focused plus real-CLI acceptance surfaces. The
declared write set contains the actual frame, kernel, routing, runner-test, and
fixture owners; no wrapper-only route is needed or authorized.

Disposition: proceed to contract-derived tests and the pre-implementation gate.
