# CLAUDE.md — `docs/audits/`

> Operating guide for authoring audits in this directory. Loaded into Claude Code's context when working under `docs/audits/`.

## Authorship

Claude Code owns audit authoring. Codex does not write audits. An audit is a Claude artifact: a deliberate, evidence-labeled, static or executional read of the codebase or of a specific surface, with explicit caveats. It is not a work package, not an ADR, and not a contract.

## What an audit is

An audit answers a *current-state* question: "is X present", "does Y match Z", "are there gaps between A and B". It is grounded in what is in the repo at the time the audit is written. It is **not** a plan, decision, or proposal — those belong to `docs/work-packages/`, `docs/decisions/`, or work-package prompts.

An audit can be:
- **Coverage** — does the implementation cover the contract surface? (worked example: [20260525_water_erosion_kernel_audit.md](20260525_water_erosion_kernel_audit.md))
- **Cross-reference** — does openWEPP correspond to wepp-forest baseline routines? (same example, Section 10)
- **Stub scan** — are there `todo!`/`unimplemented!`/placeholder bodies in code that claims to be physics?
- **Contract→code congruence** — does each `INV-*-NNN` invariant correspond to a code-level guard or test?
- **Boundary** — does a parser-without-consumer or consumer-without-parser exist?
- **Security** — see the security-review checklist in repo-root `CLAUDE.md`.
- **Provenance** — for each `REF-*-LEGACY-*` anchor, is the cited line range still valid at the recorded hash?

If the question is "what should we do about X?" — that's a work package or a decision, not an audit.

## File naming

`YYYYMMDD_<short_topic>_audit.md` — date is the audit-authoring date in `Asia/[user-tz]` or repo-default; ISO-ordered so the directory sorts chronologically. Hyphens or underscores in the topic; keep it under ~60 chars total.

Examples:
- `20260525_water_erosion_kernel_audit.md`
- `20260603_provenance_anchor_revalidation_audit.md`
- `20260710_parser_without_consumer_audit.md`

## Required document structure

Every audit must lead with this header:

```markdown
# <Title> — YYYY-MM-DD

Status: Draft | Final
Last updated: YYYY-MM-DD
Evidence mode: Static | Executional | Mixed
Scope: <single sentence specifying what is and is not in scope>
```

Then sections in this order:

1. **Purpose** — one paragraph stating the question the audit answers.
2. **Method** — concrete steps: what was grepped, what was read, what was executed, what was *not* executed. Quote commands if helpful.
3. **Inventory / findings** — the body. Tables are preferred for cross-reference work; line-linked file references (`[name](path#Lnn)`) are required for any claim about code.
4. **Caveats** — what the audit does **not** prove. Sampling limits, lack of execution, recency windows.
5. **Recommended follow-ups (not performed in this audit)** — optional. If present, label clearly as not-done so a reader cannot confuse it with the audit's findings.

## Evidence-mode discipline (non-negotiable)

The repo-root [CLAUDE.md](../../CLAUDE.md) §"Truthfulness About Work Performed" applies here with extra force:

- **Static** — read source, contracts, config; reasoned. No `cargo test`, no kernel invocation, no command execution beyond `grep`/`find`/`ls`/`cat`/file reads.
- **Executional** — commands were actually invoked in the audit session. Name them: `cargo check`, `cargo test -p <crate>`, the specific CLI fixture run.
- **Mixed** — both modes present; label each section.

Violations of evidence labeling that have actually happened and must be avoided:
- Writing "I verified that X" when only `grep` ran. Use "grep returns zero hits for X" instead.
- Writing "the kernel produces Y" when the kernel was never run. Use "the kernel emits a `WritebackField::bounded(Y, …)` at line N" — that is the actual evidence.
- Writing "X matches wepp-forest" when only the algorithmic *shape* was compared. Use "X has the same control-flow shape as `<file>.for:<L>-<L>`; numerical-parity diff not performed."

The verb must match the evidence. If the audit reader is surprised to learn what was not actually checked, the audit failed.

## Re-running the kernel-physics + cross-reference audit

This is the worked-example runbook for [20260525_water_erosion_kernel_audit.md](20260525_water_erosion_kernel_audit.md). Use the same skeleton for any future "are kernels real / what's missing vs baseline" audit.

### Phase 1 — inventory production kernels

```bash
grep -rn "impl HillslopeKernel\|impl WatershedKernel" crates --include="*.rs" | grep -v target | grep -v worktree
```

For each `impl`, find the dispatch entry function (`run_hillslope_phase` / `run_watershed_node`) and enumerate the match arms. Each match arm is one production kernel surface.

### Phase 2 — stub scan

```bash
grep -rn "todo!\|unimplemented!\|FIXME\|placeholder" crates --include="*.rs" | grep -v target | grep -v worktree | grep -v test
```

Distinguish:
- Real stubs: empty function bodies, `todo!()`, `unimplemented!()`.
- **Anti-stubs**: guards that *reject* placeholder output (e.g. `OWSOUT-E-004`). These are *not* stubs.
- **NOP-by-class**: a `_ =>` arm that returns a "kernel does not own this phase class" status. Confirm by checking the phase class is dispatched to a *different* kernel by the scheduler. Not a stub.

### Phase 3 — per-kernel physics summary

For each production kernel arm, read the entry function body. Look for:
- Substantive numerical computation (not just guard chains).
- Inline citations of baseline routines (e.g. `// Bottom-up routing mirrors legacy WEPP percolation ordering in PURK.`).
- Continuity / closure checks against an observed-side balance (a strong signal the physics is real).
- Symbol writebacks via `WritebackField::bounded(...)` — verify the symbol matches what the SC contract specifies.

Cite each finding with a line-linked file reference. Never describe physics without a line link.

### Phase 4 — contract anchoring

For each kernel arm, locate the SC contract under [docs/specifications/science-contracts/contracts/](../specifications/science-contracts/contracts/). The contract's `## Authority Anchors` table cites the `REF-*-LEGACY-*` rows that point at wepp-forest source files and line ranges under hash `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

The audit confirms *contract-to-baseline anchoring exists*. It does **not** confirm numerical parity against the cited lines — that's a per-WP exercise.

### Phase 5 — cross-reference against wepp-forest

Source location: [/workdir/wepp-forest_260430_baseline/src/](../../../wepp-forest_260430_baseline/src/) — 233 `.for` files at hash `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

```bash
ls /workdir/wepp-forest_260430_baseline/src/*.for | wc -l
```

For each baseline routine in scope:

1. Read the leading comment block to identify purpose:
   ```bash
   awk '/^[Cc][^A-Za-z0-9]/ {print; n++; if (n>20) exit}' /workdir/wepp-forest_260430_baseline/src/<file>.for | head -22
   ```
   The `+ + + PURPOSE + + +` block is the canonical statement.

2. Grep openWEPP for the concept (not the Fortran symbol — the *physics concept*):
   ```bash
   grep -rEni "<concept>" crates --include="*.rs" | grep -v target | grep -v worktree
   ```

3. Categorize each baseline routine:
   - **Confirmed implemented** — Rust analog exists with same algorithmic shape.
   - **Missing** — no Rust analog, grep negative, no SC contract claiming coverage.
   - **Reduction / placeholder** — Rust has *something* with the right name, but the formula does not match wepp-forest physics. **This is the most dangerous category** because guards pass and the surface looks real. Flag it explicitly; do not call it "implemented."
   - **Algorithmic-shape divergence** — Rust uses a deliberately different formula (e.g. Foster-Meyer power-law vs Yalin). Confirm against the SC contract; if contract-sanctioned, note it but do not call it a gap.
   - **Out of scope** — plant, decomposition, I/O, climate adapter unless the audit explicitly covers them.

4. Watch for **parser-without-consumer** surfaces. If a parser is invoked then assigned to `_`:
   ```bash
   grep -rn "let _<surface> = parse_<surface>" crates --include="*.rs"
   ```
   The parsed input is silently inert. This is a hidden-contract risk.

### Phase 6 — phase-class taxonomy check

```bash
grep -A 20 "pub enum HillslopeKernelPhaseClass" crates/openwepp-kernel-contract/src/lib.rs
```

The variants of `HillslopeKernelPhaseClass` are the canonical list of first-class kernels. If a wepp-forest routine class (snow, frost, channel-erosion, canopy, infiltration) has no matching phase-class variant, the corresponding physics is either:
- folded into a sibling phase class as a helper (snow/frost are folded into runoff reconciliation),
- handled outside the kernel (climate adapter), or
- not implemented.

The phase-class enum is the structural signal of what is and is not a first-class kernel.

## Common pitfalls

- **Mistaking guard density for physics.** A function with 200 lines of `require_state_scalar` and `require_state_range` is *not* automatically a real kernel. Find the actual computation.
- **Mistaking algorithmic citation for implementation.** A comment `// Mirrors PURK ordering` is evidence the author *intended* parity, not evidence parity is achieved. Read the math.
- **Mistaking a reduction for a stub.** A function that returns *something* numerical but uses the wrong formula will pass `grep` for stubs but fail any parity check. The snow/frost couplings in [hydrology.rs:2330](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2330) and [hydrology.rs:2496](../../crates/openwepp-hillslope-orchestrator/src/hydrology.rs#L2496) are this category.
- **Calling a missing surface "stubbed."** If there is no Rust file, no module, no phase-class variant, and no parser, it is *missing*, not stubbed. Stubs are present-but-hollow; missing is absent.
- **Inflating a static read into an executional claim.** Never write "the kernel emits X under condition Y" without running the kernel. Write "the writeback at line N is `WritebackField::bounded(X, …)`; the branch that emits it is gated by condition-shape Y."
- **Conflating SC contract claims with code claims.** The SC contract may say "the kernel must do X." The code may or may not do X. Cite the contract for the *requirement*, cite the code for the *implementation status*, separately.

## Skeleton for a new audit

Copy this for new audits in this directory.

```markdown
# <Concise Title> — YYYY-MM-DD

Status: Draft
Last updated: YYYY-MM-DD
Evidence mode: Static
Scope: <one sentence>

## 1. Purpose

<one paragraph: the question being answered>

## 2. Method

- Enumerated <X> via `grep …`.
- Read <Y>.
- Did **not** execute <Z>.
- Did **not** diff against <W>.

## 3. <Inventory / Findings / Coverage table>

<tables with line-linked references>

## 4. Findings

1. <terse declarative finding>
2. …

## 5. Caveats

- <sampling limit>
- <execution gap>
- <recency window>

## 6. Recommended follow-ups (not performed in this audit)

- <optional>
```

## Lifecycle

Audits are **point-in-time snapshots.** Do not edit a past audit to reflect new state — write a new dated audit. The old audit's value is its frozen view of what the repo looked like on its date.

Exception: typo corrections, dead-link repair, and adding a clarifying note dated `YYYY-MM-DD` are fine. Do not silently rewrite findings.

When adding a note that corrects a specific row of a table, do **not** insert the note between rows — that splits the table and breaks the renderer. Place notes immediately after the complete table under a "**Notes on N.N:**" bullet list, identifying which row each note targets.

Audits do not get "closed" or "completed" — they are written once and read forever. If their findings drive work, the work belongs to a work package; the audit stays as the evidence record.
