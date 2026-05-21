# INIMPL18 Worktree Branch Registry

Evidence mode: `Static` + `Ran`

## 1. Canonical Registry

| Package | Branch name | Worktree path | Baseline branch | Provisioning status |
| --- | --- | --- | --- | --- |
| `INIMPL19` | `inimpl19/watershed-structure-parser` | `/home/workdir/openWEPP/.worktrees/inimpl19-watershed-structure` | `main` | provisioned |
| `INIMPL20` | `inimpl20/watershed-channel-parser` | `/home/workdir/openWEPP/.worktrees/inimpl20-watershed-channel` | `main` | provisioned |
| `INIMPL21` | `inimpl21/watershed-impoundment-parser` | `/home/workdir/openWEPP/.worktrees/inimpl21-watershed-impoundment` | `main` | provisioned |

## 2. Baseline Commit State

`Ran` checks:
- `git worktree list --porcelain`
- `git branch --list 'inimpl1*' 'inimpl2*'`
- `ls -d .worktrees/*`

Observed baseline state:
- `INIMPL19..21` are provisioned from baseline commit
  `214f3f79837a51f393b38c5ebe1e84a5e1c08890`.

## 3. Provisioning Commands (Normative)

```bash
git worktree add .worktrees/inimpl19-watershed-structure -b inimpl19/watershed-structure-parser
git worktree add .worktrees/inimpl20-watershed-channel -b inimpl20/watershed-channel-parser
git worktree add .worktrees/inimpl21-watershed-impoundment -b inimpl21/watershed-impoundment-parser
```

## 4. Registry Invariants

1. Every worker branch maps to exactly one worktree path.
2. Worker branch names are immutable for Wave 3.
3. All worker worktrees must be rooted from a single scaffold baseline commit
   before coding starts.
4. Rebase target for worker branches must be coordinator-designated
   scaffold/integration baseline.
5. `INIMPL22` may run intake while registry is partial, but may not execute
   final integration gates until all three worker streams are provisioned and
   complete.

Violation of any invariant is `HOLD` for affected worker start or integration
promotion.
