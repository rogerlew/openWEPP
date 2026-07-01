# 5. Concepts and glossary

The project vocabulary, in two parts: a translation table for developers
coming from legacy Fortran modeling code, then the openWEPP-specific concepts
in alphabetical order.

## 5.1 If you're coming from Fortran WEPP / libsnobal / SWAT+

| You know it as… | In openWEPP it is… |
|---|---|
| `COMMON` block / module-level shared arrays | Typed frame fields. Run-lifetime carry state lives on `DirectLaneFrame`; within-day working state on `DirectDayFrame`. Every quantity is a named field with a type; nothing is globally writable. |
| The daily `CALL` sequence in the main loop | The phase plan: an explicit, validated sequence of *phase spans* (chapter 3, §3.4). Order is data, not source layout; a missing producer is a runtime error naming what's missing. |
| A subroutine mutating shared state | A phase span: a method that computes from typed inputs, writes typed state fields, and publishes *downstream operands* for later phases. What it may touch is visible in its signature. |
| `REAL` variables, units by convention | Unit-carrying types (`openwepp-unit-boundary`) and a units registry. Field names carry units (`_m`, `_m_s`, `_kg_m3`, `_c`); mixing them is a compile error or a typed guard error. |
| Six-character WEPP variable names (`thetfc`, `frdp`, `salb`…) | Kept — deliberately. The canonical WEPP symbols remain the authoritative vocabulary in contracts, code, and outputs; internal aliases are recorded explicitly. Your knowledge of WEPP's naming transfers. |
| `GOTO`-era error handling / silent `NaN` propagation | Typed guard errors. `validate_finite`, range guards, and closure checks fail the run at the offending lane/day with a decomposed message. There is no "keep going with a default." |
| Writing `*.out` text files as you go | A streaming publication sink: typed rows → parquet row groups; HBP + loss built once at run end from a summary accumulator. |
| Recompile with `WRITE(*,*)` to debug | Env-gated JSONL trace hooks on hot phases, the run manifest's counters, and (planned) `openwepp-replay` over the HBP record. |
| "The old version is the reference" | The **science contract** is the reference; the legacy binary is a comparator *flag* (chapter 2, §2.1). |

Two habits to unlearn:

- **Don't reach for global state.** If a phase needs a value, it arrives as a
  typed input or a prior phase's downstream operand. If neither exists, that's
  a design conversation (and probably a contract amendment), not a shortcut.
- **Don't patch output to match legacy.** If openWEPP and legacy disagree, the
  divergence is routed through the comparator process — contract decides.

## 5.2 Glossary

**ADR** — Architecture Decision Record (`docs/decisions/`). Numbered,
immutable once ratified, superseded rather than edited.

**Closure** — a conservation identity holding within tolerance (water balance,
mass balance). "Closure check" = the runtime assertion of it; failing closure
is a hard error carrying the operand decomposition. The roadmap orders work by
closure, not magnitude (chapter 2, §2.3).

**Comparator / comparator tier** — the practice of diffing openWEPP against
the pinned legacy baseline as an *investigation signal*. Confidence tiers
route the signal: single-OFE daily water balance is high confidence; hourly or
watershed-level deltas are investigation triggers only. Never an acceptance
gate ([ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)).

**Day frame (`DirectDayFrame`)** — the typed working state of one OFE-day;
seeded from lane carry state, mutated by the phase sequence, committed back.
See chapter 3, §3.3.

**Direct runtime** — the array-native, fully-typed hillslope execution path
(`direct_runtime/` in the hillslope orchestrator). "Direct" is a historical
contrast with the deleted symbol-map ("compatibility") runtime — since
ADR-0031 it is simply *the* runtime. Chapter 6 tells that story.

**Downstream operands** — the typed outputs a phase publishes for later phases
to consume. The runtime-enforced version of "this routine's results feed that
one."

**Guard / guard error** — a typed validation at a phase or kernel boundary
(finite, nonnegative, in-range). Guards fail with the symbol name, value, and
location; they are the reason bad values cannot travel.

**HBP (hillslope binary pass) shard** — the per-hillslope binary record that
crosses the hillslope→watershed process boundary (§4.2).

**Identity gate / protected outputs** — the regression discipline for
behavior-preserving changes: named output artifacts (H2637 HBP/WAT/PASS/loss/
manifest) must remain byte- or value-identical across the change. This is
*internal regression protection*, distinct from legacy comparison (which is
semantic and tolerance-based, [ADR-0003](../decisions/0003-parity-semantic-not-bit.md)).

**Kernel** — a pure function over typed state implementing contract physics
(e.g. the WB11 hydrology kernel, the winter frost partition). Kernels own
physics; orchestrators own time-stepping and topology.

**Lane** — one OFE (overland flow element) in runtime vocabulary.
`DirectLaneFrame` is the per-OFE carry state; "lane index" is the OFE index.

**OFE** — overland flow element: a homogeneous soil/management/slope segment
of a hillslope. A hillslope is an ordered chain of OFEs; H2637 has 19.

**OFE-day** — one OFE simulated for one day; the unit of hot-path work and of
performance accounting (H2637 = 235,961 of them).

**Phase span** — one entry in the per-day execution sequence: compute →
mutate → publish operands → record shadow projection (§3.4).

**Protected outputs** — see *identity gate*.

**Science contract (`SC-<DOMAIN>-<NNN>`)** — the normative physics document
for a process: governing equations, invariants (`INV-<DOMAIN>-<NNN>`),
tolerances, units, provenance. The correctness authority
(`docs/specifications/science-contracts/`).

**Seed authority** — the typed computation of day-zero state
(`DirectProductionSeedAuthority`): initial layer stores, controls, ET demand.
"Authority" because exactly one code path is allowed to produce it.

**Shadow projection** — a typed snapshot a phase records of its own results,
used by audit, diagnostics, and cross-phase reads. Historical name: during
the rewrite these "shadowed" the legacy runtime for comparison; the audit
role remains.

**Sidecar** — a legacy auxiliary input file (`snow.txt`, `pmetpara.txt`, …)
discovered next to the WEPP inputs; handled by `openwepp-legacy-bridge` with
explicit warnings for anything unknown.

**Symbol / `BoundarySymbol`** — the canonical WEPP name of a quantity
crossing a boundary. Once the key type of an entire (deleted) runtime
representation; today used at I/O edges, guard errors, and diagnostics.

**Winter column** — the stateful snow + frost sub-solver whose layered state
persists across days (§3.5, [ADR-0026](../decisions/0026-stateful-winter-column-sub-solver.md)).

**Work package (WP)** — a dated, self-contained initiative under
`docs/work-packages/<yyyymmdd-name-nnn>/`: scope, prompts, artifacts,
reviews, closure. The project's execution log and evidence store (chapter 7).

**wepppy / wepppyo3 / wepp-palimpsest** — the consumer orchestrator, its
legacy interchange layer, and the legacy WEPP source surface, respectively
(§1.5).
