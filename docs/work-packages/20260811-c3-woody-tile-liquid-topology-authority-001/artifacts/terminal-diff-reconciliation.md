# Terminal Diff Reconciliation

Status: `RECONCILED / dual terminal verification PASS`

Evidence mode: `Static + Ran`

Base HEAD: `02631ae92af6b073ed7957592fef4bad68dcf77f`

## Exact Status Roots and Ownership

The terminal `git status --short` contains these exact tracked files:

- `crates/openwepp-vegetation/src/transaction.rs` — explicitly authorized
  fail-closed containment: the heterogeneous-topology guard moved before every
  E04 evaluation;
- `docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md` —
  canonical V2 vegetation authority amendment;
- `docs/specifications/science-contracts/index.md` — approved/active registry;
- `docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/successor-release-decision.md`
  — V2 successor identity;
- `docs/work-packages/README.md` — package catalog lifecycle;
- `tests/integration/c3_vegetation_implementation_contract.rs` — contract-derived
  implementation guard and lint-safe decomposition;
- `tests/integration/land_surface_energy_balance_authority_contract.rs` —
  transaction-contract digest binding;
- `tests/integration/vegetation_boundary_authority_contract.rs` — V2 authority,
  fixture, digest, topology, and poison guards.

The terminal untracked roots are:

- `docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md`
  — new shared owner/transaction authority;
- this complete Stage-A package tree — authority, oracle, fixtures, reviews,
  gate receipts, concise comparator logs, verification, and prompt;
- `docs/work-packages/20260811-coupled-c3-forest-vegetation-model-stack-authority-001/artifacts/openwepp_c3_woody_v2_definition.json`
  — byte-identical canonical V2 definition copy;
- `docs/work-packages/20260811-coupled-c3-forest-vegetation-state-machine-implementation-001/artifacts/c3-hold-lift-guidance.md`
  — preserved operator-supplied lift directive.

Every path is inside the declared Stage-A write set or the specifically
authorized pre-authority containment change. `git diff --check` passes.

## Production and Protected-Boundary Proof

The sole production Rust delta is `15` additions and `6` deletions in
`transaction.rs`; it introduces no scientific equation and only executes the
existing fail-closed guard before E04. No runtime selector, feature default,
production consumer, deployment, publication, output, calibration, canopy
snow, soil transformation, or real-owner cutover path changed. V1 canonical
definition bytes remain identical to HEAD.

`transaction.rs` is `2,088` lines: a line-count `WARN` above `2,000`, but below
the `3,000` mandatory-refactor threshold. Stage A adds only the bounded early
containment helper/move. The existing implementation package owns later module
decomposition alongside the complete V2 transaction implementation.

## Generated-Scratch Reconciliation

Two Cargo target trees accidentally landed beneath evidence paths during
interrupted detached attempts. They were moved intact outside the repository:

- `/home/workdir/openwepp-task-trash/20260812T173742Z/target` (`996 MiB`);
- `/home/workdir/openwepp-task-trash/20260812T173831Z/target` (partial target).

An earlier repository-root scratch directory was likewise moved intact to
`/home/workdir/openwepp-task-trash/c3_v2_heavy_gates.jxh4GlllNC`. Concise logs
and all failed/interrupted attempt evidence remain in the package. A recursive
search finds no `target` directory in the Stage-A package, and the package is
approximately `1.5 MiB`.

Three raw command captures contained terminal-originated trailing whitespace
that made the staged diff fail hygiene. Their exact bytes were preserved under
`/home/workdir/openwepp-task-trash/c3_v2_raw_logs/`; the package retains the
corresponding structured command logs, summaries, and the complete additive
failure account in `comparator-gate-results.md` and `gate-results.md`.

## Terminal Evidence

Both final science rereviews pass; focused authority tests pass `14/14`;
admission passes with 45 contracts and receipt
`464b2675f17f75a6a9e92c6de0a70dae76ef03ca092c23f29d2ad965d62be628`;
workspace Clippy, doc tests, deny, format, and diff checks pass; and the exact
full-workspace run passes `2422/2422`. Both terminal verifiers pass with no
unresolved material finding. The kickoff prompt is archived byte-for-byte under
`prompts/archived/`.
