# pre implementation contract gate

Status: satisfied for M-E0 contract/test scaffold; production implementation held

Evidence mode: Static

M-E0 gate:
- Reopened. M-E0 reread the package, staged plan, M-D architecture artifact,
  work-package governance, science-contract governance, crate/test governance,
  current `SC-RUNOFFPART-001`/`SC-WATBAL-001`/`SC-SYSTEM-001` MOFE authority,
  scheduler/writeback/publication seams, and the existing M-B contract smoke
  test before edits.
- Satisfied for contract/test scaffold. M-E0 amended the three contracts,
  updated the registry, registered `mofe01_per_ofe_state_contract`, and proved
  the positive authority test passes.
- Held before production implementation. The full M-E0 target now fails on the
  required red architecture gate because current production code lacks
  `PerOfeDailyWaterBalanceCollection` or equivalent typed OFE-keyed dynamic
  state.

M-A was characterization/scoping only and did not edit production kernel paths or science contracts.

Read before M-A execution:
- Root `AGENTS.md`.
- `docs/work-packages/AGENTS.md`.
- `docs/specifications/science-contracts/AGENTS.md`.
- `crates/AGENTS.md`.
- `tests/AGENTS.md`.
- Package `package.md`.
- `artifacts/mofe-staged-increment-plan.md`.

M-B gate:
- Satisfied. Before production edits, M-B reopened the science-contract gate and amended `SC-RUNOFFPART-001` and `SC-WATBAL-001` for the route-closure semantics implemented by the increment.
- `docs/specifications/science-contracts/AGENTS.md`, `crates/AGENTS.md`, and `tests/AGENTS.md` governed the contract/code/test edits.

M-C gate:
- Reopened. M-C read the package, staged plan, routing scope, work-package
  governance, science-contract governance, crate/test governance, current
  `SC-WATBAL-001` WB13/MOFE addenda, and the runner publication path before
  considering production edits.
- Held before implementation. Current contract/code still authorize and enforce
  single-row MOFE04 publication (`OFE=1`, `QOFE=Q`) while M-C requires real
  per-OFE semantics or a contracted equivalent. No production edit was made
  because the available runtime state is aggregate-only for WAT publication.

M-C2 gate:
- Reopened. M-C2 reread the package, staged plan, M-C evidence, work-package
  governance including the gate-evidence non-deferral rule, science-contract
  governance, crate/test governance, scheduler/writeback seams, MOFE hourly
  carry seams, WB13/WAT publication path, and existing M-B tests.
- Held before contract/code edits. The scoping read showed that current MOFE
  hourly carry arrays are hour-indexed transfer state, not per-OFE daily WB
  output state. Contracting per-OFE daily state before an implementable state
  surface exists would bless surrogate output synthesis, so M-C2 records a
  BLOCKED gate instead.

M-D gate:
- Reopened for design. M-D reread the package, staged plan, work-package
  governance, science-contract governance, crate/test governance, M-C2 scope
  evidence, scheduler/writeback seams, WB13/WAT publication seams, current
  contract addenda, and pinned legacy `irs`/`rochek`/WATBAL routing lineage.
- Satisfied for design-only scope. M-D produced
  `mofe-per-ofe-state-architecture.md` and made no production code, science
  contract, or test edits.
- At the M-D boundary, M-E0 was required to reopen this gate before
  implementation by amending `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
  `SC-SYSTEM-001` and installing contract-derived red tests for the M-D
  per-OFE state semantics. M-E0 satisfied that scaffold gate above.
