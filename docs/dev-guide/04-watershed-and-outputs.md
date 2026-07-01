# 4. Watershed runtime and outputs

How hillslope runs compose into a watershed run, how state crosses the
process boundary, and who owns the output surface.

> **Status note (2026-07-01).** The watershed tier is mid-rewrite. The
> architecture below is the ratified target
> ([watershed runtime spec](../architecture/watershed-runtime-architecture-specification.md),
> [ADR-0032](../decisions/0032-watershed-runtime-ratification.md)); the
> serial supervisor (WSHED-W2) is executed and the bounded worker pool
> (WSHED-W3) is in flight. Parts of the *current* watershed orchestrator
> still use the symbol-keyed writeback surfaces that the hillslope tier
> already deleted — they are scheduled for retirement by this rewrite, not a
> pattern to extend.

## 4.1 The process model: subprocess-per-hillslope

A watershed run is a fan-out of independent hillslope simulations followed by
channel routing over their results
([ADR-0004](../decisions/0004-subprocess-hillslope-orchestration.md)):

```text
openwepp-cli-watershed
  ├─ typed watershed run plan          (which hillslopes, which channels)
  ├─ bounded worker pool               (N × openwepp-cli-hill subprocesses,
  │                                     deterministic job order, --jobs cap)
  ├─ pass inventory + freshness gate   (validate every H*.hbp exists, parses,
  │                                     and matches the plan)
  ├─ watershed network frame           (typed channel/impoundment state)
  ├─ deterministic dispatch            (topologically-ordered routing,
  │                                     openwepp-watershed-orchestrator)
  └─ watershed publication             (parquet + HBP-sidecar outputs)
```

Three properties are deliberate:

- **Isolation.** Hillslopes only communicate with the watershed tier through
  **HBP pass shards on the filesystem** — the same contract wepppy drives.
  A hillslope crash cannot corrupt sibling state; a hillslope binary upgrade
  cannot change routing except through the versioned shard format.
- **No shell.** Subprocess argument vectors are constructed explicitly
  (`std::process::Command`, no shell interpolation).
- **Determinism across parallelism.** `--jobs` bounds concurrency, but job
  *ordering* and routing dispatch are deterministic, so a `--jobs 1` and a
  `--jobs 16` run produce identical outputs.

The measured motivation for the rewrite (WSHEDPERF01, 2026-07-01): on the
36-hillslope fixture the routed channel stage costs ~0.08 s while the
hillslope fan-out costs ~60 s — watershed performance *is* hillslope fan-out
throughput, which is why the worker pool, not routing, leads the program.

## 4.2 HBP pass shards

The HBP ("hillslope binary pass") shard is the inter-binary contract: the
per-day event/water/sediment record a hillslope run leaves behind for the
watershed tier to route. The format follows the legacy wepp-palimpsest
contract (`docs/contracts/`), which is what lets openWEPP hillslopes slot
under existing consumers. Shards are structured input from a nominally
trusted producer but are still bounds-checked on read; malformed shards are
rejected, not repaired.

## 4.3 Fixtures

The watershed spec defines a fixture ladder; any fixture used as an
acceptance gate must be committed to this repository:

| Fixture | Size | Role |
|---|---|---|
| arboreal-dendrite | 36 hillslopes | smoke / baseline timing (WSHEDPERF01 measurements; not yet committed) |
| `carnivorous-adobo` | 32 hillslopes + routed stage | preferred development fixture, committed at `tests/fixtures/watershed/carnivorous-adobo/` |
| (1,000+ hillslope fixture) | — | required before performance ratification |

The single-hillslope performance anchor is **H2637** (19 OFEs, 34 years,
235,961 OFE-days) — the endpoint every hillslope performance number in the
work-package log refers to.

## 4.4 The output surface: openWEPP-native, closure-semantic

openWEPP owns its output schemas
([ADR-0019](../decisions/0019-openwepp-owns-its-output-surface-wepppyo3-legacy-only.md)):
the legacy `wepppyo3` interchange schemas are frozen as wepp-legacy-only, and
openWEPP-native outputs match consumers by **closure semantics** — the water
balance identities a consumer computes must hold — rather than by inheriting
the legacy column layout.

Per hillslope run: WAT parquet (daily water balance), PASS parquet, plot
parquet, HBP shard, loss report (JSON), and a run manifest carrying runtime
counters and provenance (source commit, binary checksum). Per watershed run:
watershed parquet plus HBP-sidecar outputs (`chan_out` et al.).

Aggregations get their own binary rather than a post-processing script:
`openwepp-cli-totalwatsed3`
([ADR-0020](../decisions/0020-totalwatsed3-dedicated-output-aggregation-cli.md))
reads completed per-hillslope parquet read-only and emits the totalwatsed3
water-balance aggregation. The rule it embodies: **simulation binaries
simulate; aggregation binaries aggregate; neither reaches into the other's
state.**

## 4.5 Release boundary

Released engines are named `openwepp_YYMMDD*` and ship with JSON sidecars for
each role; `open_wepp_runner` is the launcher that resolves a release
invocation to the right binary
([runner contract](../contracts/openwepp-runner-contract.md),
[release contract](../contracts/openwepp-binary-release-contract.md)).
Release candidates pass the gate scripts under `tools/release/`, including
anti-evasion checks that fail the release if a required validation was
weakened or skipped.
