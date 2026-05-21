# INIMPL10 Worktree Branch Registry

Evidence mode: `Static` + `Ran`

## 1. Canonical Registry

| Package | Branch name | Worktree path | Baseline branch | Provisioning status |
| --- | --- | --- | --- | --- |
| `INIMPL11` | `inimpl11/pmetpara-parser` | `/home/workdir/openWEPP/.worktrees/inimpl11-pmetpara` | `main` | provisioned |
| `INIMPL12` | `inimpl12/irrigation-depletion-parser` | `/home/workdir/openWEPP/.worktrees/inimpl12-irrigation-depletion` | `main` | provisioned |
| `INIMPL13` | `inimpl13/irrigation-fixeddate-parser` | `/home/workdir/openWEPP/.worktrees/inimpl13-irrigation-fixeddate` | `main` | provisioned |
| `INIMPL14` | `inimpl14/frost-parser` | `/home/workdir/openWEPP/.worktrees/inimpl14-frost` | `main` | provisioned |
| `INIMPL15` | `inimpl15/snow-parser` | `/home/workdir/openWEPP/.worktrees/inimpl15-snow` | `main` | pending |
| `INIMPL16` | `inimpl16/weppui-parser` | `/home/workdir/openWEPP/.worktrees/inimpl16-weppui` | `main` | pending |

## 2. Baseline Commit State

`Ran` checks:
- `git worktree list --porcelain`
- `git branch --list 'inimpl1*'`
- `ls -d .worktrees/inimpl1*`

Observed baseline state:
- `INIMPL11..14` are provisioned from baseline commit `191f09c32a94b68bcae237384eb393cc7bc628a4`.
- `INIMPL15..16` are not yet provisioned.

## 3. Provisioning Commands (Normative)

```bash
git worktree add .worktrees/inimpl15-snow -b inimpl15/snow-parser
git worktree add .worktrees/inimpl16-weppui -b inimpl16/weppui-parser
```

## 4. Registry Invariants

1. Every worker branch maps to exactly one worktree path.
2. Worker branch names are immutable for Wave 2.
3. All worker worktrees must be rooted from a single scaffold baseline commit before coding starts.
4. Rebase target for worker branches must be coordinator-designated scaffold/integration baseline.
5. `INIMPL17` may run intake while registry is partial, but may not execute final integration gates until all six worker streams are provisioned and complete.

Violation of any invariant is `HOLD` for affected worker start or integration promotion.
