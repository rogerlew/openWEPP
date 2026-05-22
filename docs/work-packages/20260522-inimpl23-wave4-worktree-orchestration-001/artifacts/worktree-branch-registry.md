# INIMPL23 Worktree Branch Registry

Evidence mode: `Ran` + `Static`

`Ran` commands:
- `git worktree list --porcelain`
- `git branch --list 'inimpl*'`
- `ls -d .worktrees/*`
- `git rev-parse HEAD`

## 1. Baseline

- Baseline reference branch: `main`
- Observed baseline `HEAD` at authoring time:
  `e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`
- Interpretation: [INFERENCE] Use a single Wave 4 scaffold baseline commit
  derived from current `main` before provisioning `INIMPL24..29` worktrees.
- Post-provision note (2026-05-22): [DIRECT] `INIMPL24..29` were provisioned
  from this baseline SHA.

## 2. Existing Worktree Topology (Observed)

[DIRECT] Existing provisioned worktrees:
- `inimpl03/slope-parser`
- `inimpl04/soil-parser`
- `inimpl05/climate-parser`
- `inimpl06/management-parser`
- `inimpl11/pmetpara-parser`
- `inimpl12/irrigation-depletion-parser`
- `inimpl13/irrigation-fixeddate-parser`
- `inimpl14/frost-parser`
- `inimpl15/snow-parser`
- `inimpl16/weppui-parser`
- `inimpl19/watershed-structure-parser`
- `inimpl20/watershed-channel-parser`
- `inimpl21/watershed-impoundment-parser`

## 3. Wave 4 Registry (Current)

| Package | Planned branch | Planned worktree path | Status |
| --- | --- | --- | --- |
| `INIMPL24` | `inimpl24/chaninp-parser` | `/home/workdir/openWEPP/.worktrees/inimpl24-chaninp` | provisioned |
| `INIMPL25` | `inimpl25/tc-parser` | `/home/workdir/openWEPP/.worktrees/inimpl25-tc` | provisioned |
| `INIMPL26` | `inimpl26/gwcoeff-parser` | `/home/workdir/openWEPP/.worktrees/inimpl26-gwcoeff` | provisioned |
| `INIMPL27` | `inimpl27/tcr-parser` | `/home/workdir/openWEPP/.worktrees/inimpl27-tcr` | provisioned |
| `INIMPL28` | `inimpl28/phosphorus-parser` | `/home/workdir/openWEPP/.worktrees/inimpl28-phosphorus` | provisioned |
| `INIMPL29` | `inimpl29/lcwb-parser` | `/home/workdir/openWEPP/.worktrees/inimpl29-lcwb` | provisioned |
| `INIMPL30` | `main` | `/home/workdir/openWEPP` | active |

## 4. Provisioning Run Evidence (2026-05-22)

[RAN] Provisioning commands executed successfully:
1. `git worktree add .worktrees/inimpl24-chaninp -b inimpl24/chaninp-parser e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`
2. `git worktree add .worktrees/inimpl25-tc -b inimpl25/tc-parser e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`
3. `git worktree add .worktrees/inimpl26-gwcoeff -b inimpl26/gwcoeff-parser e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`
4. `git worktree add .worktrees/inimpl27-tcr -b inimpl27/tcr-parser e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`
5. `git worktree add .worktrees/inimpl28-phosphorus -b inimpl28/phosphorus-parser e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`
6. `git worktree add .worktrees/inimpl29-lcwb -b inimpl29/lcwb-parser e7f5cf2498aa434c43b0f3bfa2fc68f08e998f0f`

[RAN] Post-checks:
- `git branch --list 'inimpl24/*'..'inimpl29/*'` => six branches present.
- `.worktrees/inimpl24-*..inimpl29-*` => six worktree paths present.

## 5. Provisioning Procedure

Provision in order after shared scaffold baseline commit:
1. `git worktree add .worktrees/inimpl24-chaninp -b inimpl24/chaninp-parser <baseline-sha>`
2. `git worktree add .worktrees/inimpl25-tc -b inimpl25/tc-parser <baseline-sha>`
3. `git worktree add .worktrees/inimpl26-gwcoeff -b inimpl26/gwcoeff-parser <baseline-sha>`
4. `git worktree add .worktrees/inimpl27-tcr -b inimpl27/tcr-parser <baseline-sha>`
5. `git worktree add .worktrees/inimpl28-phosphorus -b inimpl28/phosphorus-parser <baseline-sha>`
6. `git worktree add .worktrees/inimpl29-lcwb -b inimpl29/lcwb-parser <baseline-sha>`

## 6. Registry Integrity Rules

1. Planned branch names and worktree paths are immutable once workers start.
2. Any drift between registry and actual topology is a `HOLD` condition.
3. Baseline SHA must be recorded in each worker handoff artifact.
4. No worker may rebase to a different baseline without integration-owner
   approval and explicit log entry.
