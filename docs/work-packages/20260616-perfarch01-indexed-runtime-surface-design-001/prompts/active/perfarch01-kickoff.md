# PERFARCH01 Kickoff — Indexed Runtime-Surface (Design + Feasibility)

Execution mode: package-end-to-end (architecture design + feasibility).

Autonomy: execute the design + feasibility end-to-end (audit → registry/id design
→ indexed-store prototype + measurement → hazard validation → feasibility verdict
→ staged plan → draft ADR) without asking for direction on intermediate steps.

## This is design + feasibility — not implementation

Produce the design, a quantified feasibility verdict, a staged bit-identity-gated
implementation plan, and a **proposed** ADR. **Land no production or contract
change.** Throwaway prototypes for measurement are fine (do not commit prototype
code into production paths; record the measurements).

## The decision already made

Operator target: take the single-hillslope wall-clock from ~85× vs legacy toward
**≤10× (≤5× if feasible)**. Incremental PERFOPT passes are Amdahl-capped (PERFOPT01
= 1.15×); the chosen path is the **architectural** change — replace the string-keyed
`BTreeMap<BoundarySymbol, BoundaryValue>` runtime surface with an indexed/array-backed
store, preserving the `BoundarySymbol` API + bit-identity + determinism. See
`package.md` for the proposed design (symbol registry → sorted-order `SymbolId` →
`Vec<Option<BoundaryValue>>` backing → resolve-once hot paths) and the enumerated
bit-identity hazards.

## Steps

1. **Complete the audit + read ARCH16** (`20260522-arch16-scheduler-hot-path-surface-optimization-001/`)
   — does it already cover/abandon surface indexing? Confirm the symbol-universe
   cardinality and the per-OFE-day clone/lookup/`format!`-construction **cost
   share** (so the projected speedup is grounded, not asserted).
2. **Design the registry + id assignment** — sorted-symbol-order `SymbolId`s (so
   id-order ≡ sorted-string-order, the trick that preserves sorted-iteration);
   deterministic **pre-registration** of dynamic indexed symbols from known counts
   (`nsl`, climate points, PL slots); `(root, index)`→id resolution tables. Resolve
   the open question: lazy-intern vs pre-register (lazy breaks sorted-id order).
3. **Prototype + measure** a dense `Vec<Option<BoundaryValue>>` backing on one hot
   path (per-OFE-day clone + `runtime_surface_symbol_value` + frost/WB19 layer-state
   lookups). Measure the clone/lookup savings vs the BTreeMap baseline (`Ran:`).
4. **Validate the bit-identity hazards** (package.md table): prove
   `apply_kernel_writeback` ordering, the decomposition prefix-scan guard, and HBP
   key order are preserved by the design.
5. **Feasibility verdict** — a quantified projected speedup; can we credibly reach
   ≤10×? ≤5×? If not, the honest floor + reason.
6. **Staged plan + proposed ADR** — incremental stages, each a behavior-preserving
   optimization package gated on `anchor_mismatches = 0`; draft
   `docs/decisions/00NN-indexed-runtime-surface-representation.md` for ratification
   before Stage-1.

## Hard constraints

- No production / contract change in this package (design + feasibility only).
- The eventual implementation must preserve **bit-identical** outputs
  (`anchor_mismatches = 0`, the MOFE01/WSHED01/FARPOINT01/PERFOPT01 anchor pattern)
  and determinism (`docs/numerics/`: no FP-reduction reorder, no per-OFE sequencing
  change, pinned-seed reproducibility). Design within those.
- Truthfulness: projected speedup must be prototype-**measured** (`Ran:`), not
  asserted; label `Static:` vs `Ran:`. An honest "≤10× not reachable, floor is X×"
  is an acceptable verdict.

## Required reading

- `docs/work-packages/20260616-perfarch01-indexed-runtime-surface-design-001/package.md`
- `AGENTS.md`, `docs/codex_exec_plans.md`, `docs/numerics/README.md`,
  `docs/decisions/0003-parity-semantic-not-bit.md`,
  `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- PERFHO02 `artifacts/perfho02-profiler-evidence.md` (the perf attribution);
  PERFOPT01 disposition (the bit-identity anchor method).
- ARCH16 package + the runtime-surface code in `package.md` Dependencies.
