# Backlog — Irrigation activation (management-gated)

Status: concept (deferred future work; not scheduled)
Date: 2026-06-17
Owner: maintainers
Origin: surfaced during PERFIDX03 (indexed-surface authority flip) review — the
perf migration inadvertently wired irrigation on; extracted and deferred here.

## What

openWEPP will eventually support irrigation. The pipeline already **partly exists
but is dead**:

- parsers exist: `openwepp-input-contract` `irrigation_depletion`,
  `irrigation_fixeddate`;
- the hydrology kernel has full irrigation logic that **reads** irrigation symbols:
  `openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/irrigation.rs`
  (`resolve_fixeddate_irrigation_event`, `normalize_irrigation_event`,
  `irrigation.fixeddate.event_*` / `irrigation.depletion.period_*` symbols);
- orchestrator surface-builders exist: `build_hillslope_runtime_surface_from_irrigation_depletion`
  / `_fixeddate`.

The **missing link** is the hillslope runner wiring (parse the irrigation sidecars →
build the irrigation surface → seed it), so the symbols are never present and the
kernel's irrigation path never activates.

## The governing requirement (operator, 2026-06-17)

**Irrigation must run only when specified by the management.** Activation is gated
on the **management declaration**, *not* on the mere presence of irrigation sidecar
files. A run with irrigation sidecars present but no management-declared irrigation
must behave exactly as today (irrigation inert / bit-identical). PERFIDX03's
inadvertent wiring gated on sidecar presence, which is wrong and is why it was
reverted from the perf path.

## Promotion criteria (before a work package)

- Governing science-contract authority for irrigation application (the WEPP
  irrigation model: depletion-based + fixed-date scheduling) is identified/authored
  (`SC-*`), per ADR-0011 — this is process physics, not plumbing.
- Activation is **management-gated** (above), with a typed test proving a
  sidecars-present / management-silent run is bit-identical to today.
- Validation against an **irrigated fixture** (and the pinned legacy comparator as a
  flag per ADR-0017), with conservation closure on the irrigated water balance.
- Scoped as its own package — **not** bundled into a perf/refactor change.

## Not this

This is **not** part of the indexed-runtime-surface perf migration
(PERFARCH01/PERFIDX*). The migration treats irrigation symbols, if any, as an
ordinary registry family; it must not wire or activate irrigation.
