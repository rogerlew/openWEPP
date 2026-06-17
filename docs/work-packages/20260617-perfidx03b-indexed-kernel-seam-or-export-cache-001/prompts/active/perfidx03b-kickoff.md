# PERFIDX03B Kickoff - Indexed Kernel Seam / Export Cache

Scope: local repository performance/blocker-closure task; flat-file reads/edits
only; no external connectivity required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through disposition.

Required reading:

- Core:
  - `AGENTS.md`
  - `docs/work-packages/AGENTS.md`
  - `crates/AGENTS.md`
  - `docs/work-packages/20260617-perfidx03b-indexed-kernel-seam-or-export-cache-001/package.md`
  - `docs/decisions/0022-indexed-runtime-surface-representation.md`
  - `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03_disposition.md`
  - `docs/work-packages/20260616-perfidx03-indexed-surface-authority-001/artifacts/perfidx03-worker-handoff.md`
- Conditional:
  - `docs/specifications/science-contracts/AGENTS.md` only if implementation
    starts changing process behavior or canonical contract authority.
- On demand:
  - PERFIDX02 clone/shadow evidence and touched Rust modules listed in
    `package.md`.

Required-reading budget: local scoped package/readme files are under threshold;
no heavy science contract pre-read required because this is a storage/seam
performance package with no process-physics change.

Files: use the write set declared in `package.md`.

Task: close the PERFIDX03 export-cost blocker and re-enable indexed authority
only if the hot path avoids per-lane/day full `BTreeMap` export and proves OFE5
no-regression plus output identity.

Constraints:

- Preserve `BoundarySymbol` compatibility at external/logical seams.
- Preserve kernel writeback payload shape.
- Preserve deterministic sorted-symbol effects.
- No `SC-*` changes.
- No irrigation activation or sidecar wiring changes.
- No silent defaults or fallback wrappers for missing required state.

Subagent requirement: none. The available subagent tool requires explicit user
authorization for delegation, and this operator prompt did not request subagents.
Run closure gates locally and record command evidence.

Autonomy: execute package phases end-to-end and update required artifacts without
requesting additional user direction unless hard-blocked.
