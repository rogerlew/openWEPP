# ADR-0032: Watershed runtime entrypoint, job default, and canonical benchmark mode

**Status:** Accepted
**Date:** 2026-07-01 UTC
**Deciders:** Roger Lew, Codex
**Extends:** [ADR-0004](0004-subprocess-hillslope-orchestration.md),
[ADR-0006](0006-three-binaries-incl-replay.md),
[ADR-0007](0007-openwepp-runner-and-release-governance.md),
[ADR-0020](0020-totalwatsed3-dedicated-output-aggregation-cli.md)
**Ratification provenance:**
`docs/work-packages/20260701-wshedadr01-watershed-runtime-ratification-001/`

## Context

WSHEDPERF01 showed that the arboreal-dendrite routed watershed stage is already
small (`0.07-0.08 s`), while the current practical full openWEPP command chain
is dominated by serial hillslope subprocess execution (`avg 1:01.62`). The
watershed runtime architecture spec therefore sets process-level hillslope
fanout as the first performance lever, followed by typed pass inventory, typed
network-frame routing, publication cleanup, and deletion of the existing
symbol/writeback runtime.

The draft spec left three decisions open before implementation packages could
start cleanly:

- whether the full watershed supervisor should be a new binary or live under
  `openwepp-cli-watershed`;
- what `--jobs` does when omitted;
- which sidecar/input-discovery mode is canonical for future benchmark and
  ratification evidence.

## Decision

The public watershed runtime entrypoint remains **`openwepp-cli-watershed`**.
Do not introduce a separate `openwepp-cli-watershed-run` production binary for
the new supervisor. The binary owns both watershed execution modes:

- full watershed run mode: build a `WatershedRunPlan`, execute hillslope
  subprocess jobs, validate `PassInventory`, route, and publish outputs;
- explicit routed-stage reuse mode: route from an existing declared pass
  inventory for profiling, replay, diagnostics, and comparator work.

Full watershed run mode is the production destination. Routed-stage reuse is an
edge mode and must remain explicit; it is not a silent fallback and it does not
preserve the old shell-loop/shared-output runtime as a public production
executor.

The public `--jobs` default is **`1`**. Omitting `--jobs` is equivalent to
`--jobs 1`, the deterministic serial baseline. CPU scaling is opt-in by passing
an explicit positive integer `--jobs N`; `--jobs 0` and negative values are
invalid. openWEPP does not default to all logical CPUs or physical cores because
that would make unattended CLI/wepppy behavior host-dependent and could
over-subscribe shared systems. wepppy or an operator may choose and pass a
larger explicit value.

The canonical benchmark and ratification mode is
**`strict-committed-fixture`**:

- all adopted fixture inputs needed for the gate are committed to this
  repository, preferably under `tests/fixtures/watershed/`;
- legacy sidecar discovery is disabled;
- `/wc1`, scratch, or operator-local paths are not the sole evidence for the
  gate;
- the performance record labels timing scope, sidecar/input-discovery mode,
  job count, CPU inventory, and output-identity evidence.

`canonical-sidecar-discovery-off` remains a valid label for production-style
operator measurements that use canonical paths but are not based on a committed
fixture. `legacy-sidecar-discovery-on` remains valid for historical
characterization, comparator, and migration evidence. It is not canonical
benchmark evidence for W2/W3/W5 ratification.

## Consequences

Positive:

- The public watershed surface stays aligned with ADR-0006 and ADR-0020: channel
  routing remains in `openwepp-cli-watershed`, while totalwatsed3 remains a
  separate output-aggregation binary.
- The default run is deterministic and safe on shared hosts. Scaling is still
  available and intentionally visible through explicit `--jobs N`.
- Benchmark reports cannot compare WSHEDPERF01 discovery-on timing with future
  discovery-off committed-fixture timing without naming the difference.
- W2 can implement the serial supervisor without a binary-taxonomy debate, and
  W3 can add worker-pool parallelism behind the same public command.

Negative / costs:

- Users who want automatic CPU use must configure it outside openWEPP or pass
  `--jobs N`; openWEPP deliberately does not infer a host-wide default.
- The existing routed-stage CLI behavior must be renamed or mode-gated during
  migration so the default entrypoint can become a full watershed run.
- Early exploratory timing on `/wc1` remains useful but cannot close persistent
  benchmark or ratification gates until the fixture is committed.

This ADR does not decide latest-event `NoEvent` science-contract authority,
scratch-retention policy, pass freshness hash requirements, or the large
1,000+ hillslope fixture choice. Those remain follow-on architecture/spec
questions.
