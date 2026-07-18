# Code-Quality Refactor Work-Package Authoring Guide

- **Status:** Active
- **Last updated:** 2026-07-11
- **Applies to:** work packages whose goal is **metric-driven, behavior-preserving**
  cleanup of a Rust module — complexity decomposition, dead-code removal,
  duplication consolidation, magic-number symbolization, and lint-debt burndown.

A code-quality refactor is a **specialization of a mechanical refactor**: the
change is behavior-preserving, but it is driven by a quality metric with a
numeric before/after target rather than by a structural seam. This guide adds
only what the mechanical guide lacks; it **delegates** everything shared.

## Relation to other guides (delegation — do not restate)

| Concern | Authority |
|---|---|
| Refactor mechanics: gate ladder, artifact set, anti-drift, low-cost-model playbook, file/module split patterns | [mechanical-refactor-authoring-guide.md](mechanical-refactor-authoring-guide.md) |
| Test-first precondition / coverage closure of the target module | [module-test-enhancement-authoring-guide.md](module-test-enhancement-authoring-guide.md) |
| CRAP ≤ 30 bound, cover-then-decompose sequencing, baseline ratchet | [ADR-0021](../decisions/0021-module-coverage-closure-thresholds.md) |
| Gate selection, lifecycle, escalation, and evidence reuse | [testing-and-gate-strategy.md](testing-and-gate-strategy.md) |
| Float-comparison rule, symbol glossary, determinism | [rust-scientific-coding-standard.md](rust-scientific-coding-standard.md) §7.7 |

This guide adds: the **quality-metric catalog** (§3), the **numeric-equivalence
guard** (§4), the **intra-function decomposition pattern** (§5), and the
**lint ratchet** (§8).

## 1) When to use this shape

Use a code-quality refactor package when a module is flagged by a quality
metric and the fix is behavior-preserving:

- CRAP > 30 (`cargo-crap`) — cyclomatic-complexity decomposition;
- high `cognitive_complexity` / `too_many_lines`;
- dead code (coverage cold spots that are not guard arms);
- duplication; magic numbers; accumulated `#[allow(clippy::…)]` debt.

It is **not** a mechanical refactor (those are seam/structure-driven, no metric
target) and **not** a contract/kernel change (those alter semantics). One
package targets **one module and one quality dimension** (§2.2).

## 2) Hard preconditions (Normative)

1. **Test-first.** The target module is at coverage closure
   (module-test-enhancement §2), **or** the package lands characterization
   coverage *before* any decomposition. You may not refactor under-tested code —
   the coverage is the safety net (ADR-0021 cover-then-decompose).
2. **One dimension per package.** No mixing decomposition + dead-code + naming in
   one diff (mechanical guide anti-drift, §3.5/§8). Each dimension is separately
   measurable and reviewable.
3. **Behavior-preserving = numerically identical (§4).** If a change can alter
   computed results, it is not a code-quality refactor — route it to a
   contract/kernel package with contract-first governance.
4. **Classify before ranking or suppressing.** Apply the symbol-level taxonomy
   in module-test-enhancement §3 to every row above the metric threshold. Raw
   tool output is discovery evidence; `E-*` and CRAP-above-30
   `R-INFRASTRUCTURE` rows contribute to actionable CQR ranking.
   `R-OBSERVABILITY`/`R-IRREDUCIBLE-CRAP` rows require the recorded independent
   disposition, and `X-*` rows require exact denominator evidence. A module
   name or broad glob never grants an exception.

## 3) In-scope dimensions and their closure metric

| Dimension | Tool / metric | Closure target | Notes |
|---|---|---|---|
| Cyclomatic complexity | `cargo-crap` | CRAP ≤ 30 per eligible fn | decompose (§5), test-first; ADR-0021 |
| Cognitive complexity | `clippy::cognitive_complexity` | 0 violations in the module at threshold | opt-in per module (§8) |
| Function length | `clippy::too_many_lines` | remove the module's `#[allow]`s as fns shrink | already on via `pedantic`; ~100 allows are the backlog |
| Dead code | coverage cold spots + `clippy::dead_code` | delete genuinely-unreachable code | **delete, don't `COVERAGE-EXCLUDE`**; distinguish from not-yet-wired |
| Duplication | review / extraction | consolidate, behavior-preserving | one source of truth |
| Magic numbers | named consts w/ units + provenance | value-identical constants | precedent EROD20; feeds the SC symbol glossary |
| Lint debt | `#[allow(clippy::…)]` census | remove allows the refactor obviates | 202 allows in tree; burn down, do not add |

**Out of scope — route elsewhere, never bundle into a code-quality package:**
error-handling / fail-closed behavior changes; **anything that reorders float
operations or changes numeric output**; public-API changes; threshold/tolerance
changes; `unsafe` changes (security-reviewed separately).

## 4) The numeric-equivalence guard (Normative)

This is the guard the mechanical track never needed: moving a whole function
preserves its float operations exactly, but **decomposing one does not by
default**.

- A behavior-preserving refactor of scientific code must preserve numeric output:
  **bit-identical**, or within the contract's named tolerance **only** if a
  reassociation is unavoidable **and documented and contract-authorized** (at
  which point the package is no longer pure code-quality and escalates).
- Rust/LLVM do not reassociate floating-point without fast-math, so an extraction
  is bit-identical **iff it preserves expression order and grouping**. The
  decomposition must **not**: regroup a float expression, hoist or eliminate an
  intermediate that changes rounding, change accumulation order in a loop, or
  alter short-circuit evaluation of side-effecting / `Result`-returning calls.
- **Evidence:** the precondition's characterization tests are the equivalence
  oracle — they must pass unchanged across the refactor. The disposition states
  how numeric identity was preserved.

## 5) Intra-function decomposition pattern (the CRAP operation)

The mechanical guide's split patterns move whole functions between files and
**leave cyclomatic complexity unchanged**. CRAP reduction requires extracting
*within* a function:

1. **Cover first** (§2.1).
2. Identify a cohesive sub-block — a `match` arm, a guard cluster, a loop body —
   and extract it to a private helper with explicit typed params/returns,
   **preserving exact statement and expression order** (§4).
3. Prefer extracting **whole branches** (e.g. the multi-OFE case 1–4 arms) over
   splitting a fused expression.
4. Move **one extraction at a time**; run focused tests + `cargo-crap` after each.
   The parent's CRAP drops as complexity migrates to helpers; each helper must
   also land ≤ 30.
5. Pass borrows; introduce no clone/allocation that changes a numeric path.
6. Re-measure: parent and all new helpers ≤ 30.

## 6) Procedure

Delegates the gate ladder and artifact mechanics to the mechanical guide; adds
the metric loop.

1. **Authorize and scope.** One module, one dimension; tier and symbol-level
   eligibility per ADR-0021; name
   `YYYYMMDD-cqr<NN>-<module-slug>-001`; register in `docs/work-packages/README.md`.
2. **Precondition check.** Confirm coverage closure is present or co-delivered (§2.1).
3. **Baseline metric.** Run the dimension's tool; record `*_before` evidence
   (`crap_before.md`, a clippy count, an allow census, …). Preserve the raw
   rows, then publish the classification ledger and actionable rows separately.
4. **Refactor.** Apply §5 (decomposition) or the relevant mechanical pattern, one
   edit at a time, numeric equivalence preserved (§4).
5. **Re-measure.** Record `*_after`; the dimension target (§3) is met.
6. **Gate loop.** Mechanical guide §6's exact terminal plan, including the
   module's affected coverage/CRAP and no-regression gate. Critical changes,
   campaign closure, and release use full workspace/global evidence. Before
   planner/executor cutover, use the mechanical guide's conservative full
   fallback.
7. **Evidence and disposition.** Before/after metric, the numeric-equivalence
   statement, any exclusions.

## 7) Required evidence artifacts

- The mechanical guide §9 artifact set (modularization/parity, line-count
  governance, dual review/verification, disposition, handoff).
- `*_before` / `*_after` for the dimension metric (e.g. `crap_before.md` /
  `crap_after.md`, or the clippy/allow census).
- Raw-to-actionable eligibility ledger: exact row, classification, source hash,
  gate treatment, evidence, and dual-review disposition.
- **Numeric-equivalence statement** — how output identity was preserved and which
  characterization tests evidence it.

## 8) Lint ratchet (`[workspace.lints.clippy]` + `clippy.toml`)

The closure gate runs `clippy -- -D warnings`, which promotes every `warn` lint
to `deny`. A new lint with existing violations therefore **cannot** be enabled
workspace-wide without bricking the gate. Adoption is a ratchet:

- **`too_many_lines`** is already on (via `pedantic`). Its baseline is the
  existing `#[allow(clippy::too_many_lines)]` attributes (~100 in tree). A
  code-quality package **removes** the allow as it shrinks a function — never adds
  one to dodge work.
- **`cognitive_complexity`** (nursery, off by default) is declared in
  `[workspace.lints.clippy]` at **`allow`** with a comment marking it a ratchet
  target, and its threshold set in `clippy.toml`. A package opts a module in with
  a module-level `#![warn(clippy::cognitive_complexity)]` (→ deny under the gate)
  as that module reaches closure, then clears it. The lint is promoted to a
  workspace `warn` only once the backlog is clear.
- **The adjudicated `cargo-crap` gate** is the active repo-wide complexity
  ratchet. The completed CQR campaign left no actionable production row above
  30. The gate retains raw rows for visibility, accepts only exact current
  adjudications from `tools/release/adjudicated_crap_exceptions.json`, and fails
  on any new actionable row. Clippy's complexity lints remain local signal
  until promoted.

The same pattern applies to any other lint promotion (e.g. burning down the 202
`#[allow(clippy::…)]`): enable per-module, clear, then promote workspace-wide.

## 9) Anti-patterns

- Decomposing under-tested code (no safety net) — cover first.
- **Float regrouping disguised as extraction** — silent numeric drift; §4.
- Mixing quality dimensions in one package.
- Deleting "dead" code that is actually **not-yet-wired** (e.g. a gated wave-2
  lane) — verify before deletion.
- Adding a `#[allow(clippy::…)]` to pass the gate instead of fixing the cause.
- Treating an entire binary, parser, error module, adapter, or formatter host as
  ineligible because of its filename or dominant role.
- Adding an inline CRAP allow, wildcard, or package-local exception instead of
  using the independently reviewed adjudication registry.
- Promoting a clippy lint to `warn`/`deny` workspace-wide with outstanding
  violations — bricks the gate; use the per-module ratchet.

## 10) Acceptance criteria

A code-quality refactor package is complete only when:

1. The dimension's metric target (§3) is met on the module.
2. Numeric equivalence is preserved and evidenced (§4).
3. The module's coverage gate is not regressed.
4. Exactly one quality dimension was touched.
5. Mechanical guide §10 acceptance (seam complete, gates recorded, line-count
   dispositioned, reviews dispositioned) is satisfied.
