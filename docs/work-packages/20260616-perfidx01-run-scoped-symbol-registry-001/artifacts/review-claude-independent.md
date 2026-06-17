# PERFIDX01 — Independent Review (Claude Code)

Status: APPROVE Stage 1 — **with a design-blocking finding the completeness audit
surfaced that Stage 2 must resolve before any authority flip (likely an ADR-0022
refinement).**
Evidence mode: **Static** (code + diff) + **Ran** (invariant tests; inert-path check)

## Stage 1 is correct — approve

- **Invariants proven (my own runs):** `cargo test -p openwepp-kernel-contract
  symbol_registry` → 3/3 (sorted-id, fail-closed audit, equality export). Sorted-id
  is structurally guaranteed (`from_symbols` sorts+dedups, then id = index).
- **Bit-identity holds by construction.** The only runtime touch
  (`00_runner_intake_and_lane_setup.rs`) is `symbol_registry_audit::begin_if_requested(...)`,
  which returns `None` unless `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH` is set — the
  production path is unchanged. Consistent with the reported `ANCHOR_MISMATCHES=0` /
  `DETERMINISM_MISMATCHES=0`.
- **Completeness proven** (0 post-freeze unknowns across H2637 both variants + the
  1–5-OFE ladder) — even against an intentionally over-enumerated universe, which
  is the right stress posture. Fail-closed-on-unknown is implemented as a typed
  error. Storage authority not flipped. Gates green.

Stage 1 did its job: the registry + the load-bearing invariants exist and are
validated, with zero behavior change.

## Design-blocking finding (elevated from Codex's deferred "Residual Risk")

The completeness table shows the registry is **1,699,798 symbols for H2637** vs
**3,616 actually constructed** — ~470× over the working set, and ~280× the
**~4–6K** PERFARCH01/ADR-0022 assumed. The enumerator is combinatorial (climate
breakpoints × layers × layer-pairs × frost-fine × PL slots×crops×366 × per-OFE
families …). RSS nearly doubled (228→427 MB) just holding it.

**Why this is design-blocking, not just "measure memory in Stage 2":**

ADR-0022 specifies a **dense `Vec<Option<BoundaryValue>>` indexed by the global
`SymbolId`**, justified by "~6K symbols ⇒ clone = cheap memcpy." At 1.7M that
justification inverts:

- **Lookup speedup survives** — O(1) array index regardless of array size. ✓
- **Clone speedup — the *dominant* PERFHO01 cost — breaks.** A per-OFE surface
  densely indexed over a 1.7M-id space is a ~1.7M-slot (`~40 MB`) `Vec`, cloned
  ~14×/day/OFE. That memcpy is **larger and slower** than deep-copying the small
  (~hundreds-of-entries) BTreeMap it replaces. The prototype measured dense clone
  at **6K**, not 1.7M — so the 110× clone win does not transfer.

**Stage 2 must therefore decide the storage representation up front, not after an
authority flip.** Options to weigh (Stage-2 design / possible ADR-0022 amendment):
1. **Compact per-surface ids** — the cloned per-OFE state surface is small
   (~hundreds); store it densely over a *local* compact id space, not the global
   1.7M `SymbolId`. The global id stays the logical/sort key; the clone target is
   compact.
2. **Partition by clone-frequency** — keep the read-mostly forcing universe
   (climate `timem/intsty`, the bulk of the 1.7M) out of the per-phase-cloned
   surface entirely; only the per-OFE *state* gets indexed/cloned.
3. **Sparse** (`Vec<(SymbolId, value)>` sorted) for surfaces whose present-set is a
   tiny fraction of the universe.
4. **Tighten enumeration** to the *reachable* set rather than the full
   combinatorial bound (reduces the global universe).

Until this is settled, the central premise (clone = cheap) is unproven at real
scale. This does **not** invalidate the architecture — lookups + the registry are
sound — but it changes the dense-single-global-surface decision in ADR-0022.

## Recommendation

Approve and land PERFIDX01. **Before scaffolding PERFIDX02**, resolve the storage
representation (compact/partitioned/sparse) — I'd recommend a short ADR-0022
amendment capturing the 1.7M finding and the chosen representation, so Stage 2
implements against a clone model proven at H2637 scale, not the 6K prototype.
The no-independent-dual-review caveat Codex noted is addressed by this review.
