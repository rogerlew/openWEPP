# PL09 Pre-Execution Review (Claude Code)

Status: `complete`
Evidence mode: `Static`
Disposition: `BLOCK — three preconditions before queue execution`
Author: Claude Code (review lane per `CLAUDE.md`)
Date: 2026-05-22

Static:
- Reviewed PL09 `package.md` and all `artifacts/*.md`.
- Cross-read the totality inventory and parity gap register against live source
  in `crates/openwepp-hillslope-orchestrator/src/lib.rs` and
  `runtime_inputs.rs`, and against the legacy anchor
  `/workdir/wepp-forest_260430_baseline/src/`.
- Cross-checked PL09's queue against prior package outputs (ARCH14, CLIM01–04,
  CRF-001/002).

Ran:
- Nothing executable. No `cargo` gates were invoked. The grep evidence cited
  below was the only command surface used.

## Scope

This artifact reviews PL09's outputs **before** the proposed PL10–PL15 /
WB10–WB13 / INT10 hold-lift queue is executed. The objective is to surface
issues that, if not resolved first, would cause the queue to produce wasted or
structurally compromised work.

This review does **not** re-disposition PL08, does **not** disagree with PL09's
`RETAIN PL08 HOLD` verdict (which I concur with), and does **not** rewrite the
gap register. It identifies issues with the queue's *scope, ordering, and
preconditions*.

## Findings Register

| id | severity | class | statement |
|---|---|---|---|
| `CR-PL09-001` | `critical` | `block` | Typed kernel-surface remediation absent from queue; PL12/PL13/WB10–WB13 will recommit CRF-001 at scale |
| `CR-PL09-002` | `high` | `scope` | "Hold lift" frame understates that this queue is the engine's first production kernels |
| `CR-PL09-003` | `critical` | `block` | `H5.wat.dat` Tier-A `structure_diff` named but not diagnosed; diagnosis may descope half the queue |
| `CR-PL09-004` | `high` | `block` | Probable copy-paste at `lib.rs:33` (`PL_DECOMP_IMNGMT_SYMBOL` points at growth channel) |
| `CR-PL09-005` | `high` | `audit` | Placeholder constants (`slot_0001/crop_0001`) were accepted as "implemented" by prior dispositions; pattern audit warranted |
| `CR-PL09-006` | `high` | `scope` | Totality inventory omits previously-identified architecture defects (climate duplication, HBP-twice, 1500-not-enforced, etc.) |
| `CR-PL09-007` | `medium` | `governance` | `GAP-008` (non-cropland landuse) deferred via classification rather than ratified scope decision |
| `CR-PL09-008` | `medium` | `governance` | Evidence-mode `Static + Ran` mislabeled across PL09 artifacts; nothing executable ran |
| `CR-PL09-009` | `medium` | `ordering` | `PL13 → WB11` over-serializes hydrology lane on growth-kernel completion |
| `CR-PL09-010` | `low` | `ordering` | `PL13A → PL11` looks like dependency padding |
| `CR-PL09-011` | `medium` | `ordering` | Comparator emission/replay infrastructure built last (`PL14`); infra bugs surface at maximum cost |
| `CR-PL09-012` | `medium` | `governance` | Queue models no descope branch; every gap → a remediation |
| `CR-PL09-013` | `low` | `process` | Same-day governance density + dual-agent ratification pattern; previously flagged |

## Findings

### `CR-PL09-001` — Typed kernel-surface remediation absent from queue (`critical`)

PL12, PL13, WB10–WB13 are all kernel work. They will read and write the
existing string-keyed surface — `HillslopeWritebackSurface { state_surface:
BTreeMap<BoundarySymbol, BoundaryValue> }` where
[`BoundarySymbol(String)`](/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs#L32)
is a newtype around `String` and array values are flattened via
`format!("timem_{:04}", index+1)` style keys (verified in CLIM02-04 climate
seam port). ARCH14 ratified moving to typed state surfaces and unit-boundary
wiring. The PL09 queue does not include a kernel-surface package as a
predecessor to PL12/PL13/WB10. **Every kernel landed against the current
surface increases the eventual CRF-001 fix cost roughly linearly with the
number of read/write sites.**

Required: either insert a typed-kernel-surface package as predecessor to PL10
(preferred), or formally risk-accept reopening CRF-001 with documented kernel
count and migration cost.

Evidence:
- `/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs:32`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:599-606` (`format!("timem_{:04}", …)`)
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md` (ratified direction)

### `CR-PL09-002` — Queue scope misnamed (`high`)

PL09's gap register classifies `GAP-006` (no production growth/decomp/resup
kernel) as `critical/block` and WB10–WB12 propose first implementations of
ET, percolation, lateral transfer, drainage, runoff reconciliation, and
storage reconciliation kernels — i.e., the entire hydrology layer. Plus
the daily WB output surface and cross-lane coupling.

Calling this "lift PL08's hold" obscures that PL15's acceptance criteria
*are* the engine's first-kernel acceptance criteria. The release rule should
be framed accordingly. Recommendation: rename the closeout package's purpose
("first production kernel set + Tier-A comparator closure") so the bar set at
PL15 matches what is actually being decided.

### `CR-PL09-003` — `H5.wat.dat` `structure_diff` not diagnosed (`critical`)

`GAP-002` names the Tier-A symptom — `line_count_mismatch` and
`numeric_arity_mismatch` — and the queue's strategy is to implement the
entire engine and re-run the comparator at `PL14`. Line-count and arity
mismatches commonly resolve to header rows, schema-version drift, row
ordering, or fenceposting — none of which require new physics. A one-day
diagnostic against the current openWEPP comparator output could:

- confirm whether `H5.wat.dat` parity actually requires PL12/PL13/WB-stack
  completion, or
- isolate the diff to a small output-formatting fix descoping much of the
  queue.

Required: diagnose the structure_diff before scoping is committed to.

### `CR-PL09-004` — Probable bug at `lib.rs:33` (`high`)

```rust
const PL_DECOMP_IMNGMT_SYMBOL: &str = "pl_growth_slot_0001_crop_0001_imngmt";
```

The constant is named `PL_DECOMP_*` but resolves to the `pl_growth_*`
symbol channel. Either intentional symbol sharing (then needs an inline
comment recording the decision) or a copy-paste defect.

PL10's `active-slot-authority` package will generalize these constants
directly. If this is a bug, PL10 will faithfully propagate a wrong wiring
to all slots/crops. Resolve before PL10 starts.

Evidence:
- [crates/openwepp-hillslope-orchestrator/src/lib.rs:33](/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs#L33)
- [crates/openwepp-hillslope-orchestrator/src/lib.rs:42](/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs#L42)

### `CR-PL09-005` — Placeholder constants slipped past prior dispositions (`high`)

The hard-coded `slot_0001/crop_0001` dispatch (`PL_GROWTH_*_SYMBOL` /
`PL_DECOMP_*_SYMBOL` at
[`lib.rs:33-47`](/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs#L33-L47))
was accepted by earlier PL package gates as a *dispatch implementation*.
PL09 catches it as `GAP-003` and queues it for one remediation (PL10).

The finding worth recording is the *pattern*: a hard-coded constant
masquerading as dispatch authority was accepted as `implemented` by the
governance layer. Before kicking off PL10, audit every surface marked
`implemented` in PL09's [totality
inventory](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-totality-implementation-inventory.md)
for similar placeholder constants. Otherwise the queue could be working
around latent placeholders not yet surfaced.

### `CR-PL09-006` — Inventory's "totality" omits known architecture defects (`high`)

Not mentioned anywhere in PL09's totality inventory or queue:

- Climate runtime duplicated across hillslope and watershed orchestrators
  (CLIM03 finding, same `CLIGEN_V4_IP_CORRECTION_FACTOR` + parallel error
  enums + parallel `resolve_iclig`).
- `HBP` codec implemented twice
  (`crates/openwepp-input-contract/src/parsers/hbp.rs` and
  `crates/openwepp-legacy-bridge/src/hbp.rs`).
- `1500` breakpoint cap enforced only by parser; runtime seam
  `BreakpointCountOutOfRange` is a `u32::try_from` guard, not a policy
  guard.
- `CLIM-RUNTIME-E-010` (`PositiveBreakpointDrainWithNonPositiveDeltaTime`)
  appears unreachable via the live guard path.
- `0.70` `ip` correction has no provenance comment despite being
  citable to `stmget.for:182` (mn/dcf 3/95) and resolving `CLI-GAP-002`.

PL09 bounds itself to "PL08 hold relevance only," which is defensible —
but the document calls itself the "openWEPP totality implementation
inventory." Either narrow the title to match the scope, or expand the
scope to match the title. The hold-lift queue inherits these defects
unaddressed regardless.

### `CR-PL09-007` — `GAP-008` deferred via classification (`medium`)

`GAP-008` (runtime rejects `landuse != 1`) is classified `Tier-C
investigate`; the queue has no follow-on. If openWEPP intends to restrict
to cropland indefinitely, that is a *scope* decision deserving an ADR or
contract addendum. As written, it is silently parked under a label.

### `CR-PL09-008` — Evidence-mode `Static + Ran` mislabeled (`medium`)

PL09 disposition and most artifacts declare `Evidence mode: Static + Ran`.
The `Ran:` text describes reading artifacts and running a scoped doc-lint.
[gate-results.md](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/gate-results.md)
records `cargo fmt/clippy/test/deny = not run`.

Per `CLAUDE.md`'s truthfulness discipline, the verb should match the
evidence. PL09 is `Static` with a scoped docs-lint pass. The same loose
labeling appeared in ARCH14; this is the second instance of `Ran`
encompassing "I read documents." Recommend tightening across the
project: `Static (+ docs lint)` for discovery-only packages.

### `CR-PL09-009` — `PL13 → WB11` over-serialization (`medium`)

The queue serializes the entire hydrology lane on growth-kernel
completion. Hydrology kernels (ET, percolation, lateral transfer,
drainage) can be developed against stubbed growth state and tested
independently. The runtime ordering (`decomp → growth → watbal`) holds
within a day, but does not require WB11 to wait on PL13 at
development time.

The ordering rationale in
[pl08-hold-lift-work-package-queue.md](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md)
says only "water-balance kernels consume growth-updated state surfaces" —
a true runtime statement, not a development-time blocker. Recommend
decoupling WB10–WB12 from PL13 to parallelize ~4 packages.

### `CR-PL09-010` — `PL13A → PL11` looks like dependency padding (`low`)

`PL13A` (canonical alias continuity closure for `GAP-007`) is contract-text
work driven by reading the legacy Fortran and the parser. It does not
logically depend on PL11's event projection. Either state the dependency
reason or move PL13A in parallel to free the critical path.

### `CR-PL09-011` — Comparator infrastructure built last (`medium`)

`PL14` (Tier-A candidate emission + strict replay) depends on `INT10`,
which depends on the full PL13 + WB13 stack. Building the I/O and replay
framework *first* against stubbed kernels is a standard QA technique that
surfaces infra bugs when they are cheap. The current ordering puts every
comparator-framework risk at the end of an 11-package critical path, when
fixes are most expensive.

Note this is also the lowest-cost path to closing `GAP-001` independently
of `GAP-006`.

### `CR-PL09-012` — No descope branch in the queue (`medium`)

Every gap is mapped to a remediation. There is no modeled disposition of
the form "decline `GAP-X`, scope openWEPP narrower." That is a legitimate
option (especially for `GAP-007` and `GAP-008`) and excluding it inflates
the queue and locks in scope decisions implicitly.

### `CR-PL09-013` — Recurring patterns (`low`, prior-flagged)

For the record:

1. PL09 is dated `2026-05-22` along with PL05–PL08, CLIM01–04, ARCH11–14
   and several others. Volume on one calendar day raises mechanical
   concern about review depth; flagged previously in ARCH14 and CLIM
   review threads.
2. Four `review_agent_*.md` + `verification_agent_*.md` files of
   ~750–900 bytes each. If LLM-generated, that is one model agreeing
   with itself four times, not four independent checks.
3. The lettered `PL13A` name suggests mid-design queue insertion;
   minor smell.

## Severity Summary

| severity | count | ids |
|---|---|---|
| `critical` | 2 | `CR-PL09-001`, `CR-PL09-003` |
| `high` | 4 | `CR-PL09-002`, `CR-PL09-004`, `CR-PL09-005`, `CR-PL09-006` |
| `medium` | 5 | `CR-PL09-007`, `CR-PL09-008`, `CR-PL09-009`, `CR-PL09-011`, `CR-PL09-012` |
| `low` | 2 | `CR-PL09-010`, `CR-PL09-013` |

## Pre-Execution Preconditions

Three actions before PL10 starts. None require a new work package:

1. **Diagnose the `H5.wat.dat` `structure_diff`** against current openWEPP
   output (CR-PL09-003). Outcome determines whether half this queue is on
   the critical path.
2. **Resolve `lib.rs:33`** (CR-PL09-004): bug or intentional sharing?
   Audit the surrounding `PL_*_SYMBOL` constants and any analogous
   surfaces for the same shape (CR-PL09-005).
3. **Decide on the typed-surface question** (CR-PL09-001): either insert
   a typed-kernel-surface predecessor package, or document an explicit
   risk-acceptance of reopening CRF-001 at higher cost after PL10–WB13.

Recommended additional, lower-cost adjustments before execution:
- Decouple WB10–WB12 from PL13 (CR-PL09-009).
- Move comparator emission scaffolding (subset of PL14) earlier in the
  queue against stubbed kernel output (CR-PL09-011).
- Replace `Evidence mode: Static + Ran` with `Static (+ docs lint)` in
  PL09 disposition artifacts (CR-PL09-008).

## What This Review Does Not Address

- The Tier-A confidence-tier policy itself (ADR-0011 / comparator
  tier policy).
- PL08's HOLD verdict — concurred.
- The substance of growth/decomp/resup/WB kernel physics.
- Whether the legacy WEPP behavior captured in `tilage.for`, `grow.for`,
  `decomp.for`, `resup.for` is itself correct — only whether the queue
  faithfully ports it.

## Evidence Links

- [`/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/package.md`](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/package.md)
- [`/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-totality-implementation-inventory.md`](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-totality-implementation-inventory.md)
- [`/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-vs-baseline-pl-parity-gap-register.md`](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/openwepp-vs-baseline-pl-parity-gap-register.md)
- [`/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md`](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/pl08-hold-lift-work-package-queue.md)
- [`/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/gate-results.md`](/home/workdir/openWEPP/docs/work-packages/20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001/artifacts/gate-results.md)
- [`/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs`](/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs)
- [`/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`](/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs)
- [`/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs`](/home/workdir/openWEPP/crates/openwepp-kernel-contract/src/lib.rs)
- `/workdir/wepp-forest_260430_baseline/src/tilage.for`
- `/workdir/wepp-forest_260430_baseline/src/grow.for`
- `/workdir/wepp-forest_260430_baseline/src/decomp.for`
- `/workdir/wepp-forest_260430_baseline/src/resup.for`
- [`/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md`](/home/workdir/openWEPP/docs/work-packages/20260522-arch14-claude-architecture-review-disposition-001/artifacts/disposition-register.md)
