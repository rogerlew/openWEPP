# INIMPL02 Worktree Branch Registry

Evidence mode: `Static` + `Ran`

## 1. Canonical Registry

| Package | Branch name | Worktree path | Baseline branch | Provisioning status |
| --- | --- | --- | --- | --- |
| `INIMPL03` | `inimpl03/slope-parser` | `/home/workdir/openWEPP/.worktrees/inimpl03-slope` | `main` | provisioned |
| `INIMPL04` | `inimpl04/soil-parser` | `/home/workdir/openWEPP/.worktrees/inimpl04-soil` | `main` | provisioned |
| `INIMPL05` | `inimpl05/climate-parser` | `/home/workdir/openWEPP/.worktrees/inimpl05-climate` | `main` | provisioned |
| `INIMPL06` | `inimpl06/management-parser` | `/home/workdir/openWEPP/.worktrees/inimpl06-management` | `main` | provisioned |

## 2. Baseline Commit

`Ran` local check:
- Command: `git worktree list --porcelain`
- Observed baseline commit for all worktrees: `a905851311b71435667553130ac5dfe774e70286`

## 3. Provisioning Commands (Normative)

```bash
git worktree add .worktrees/inimpl03-slope -b inimpl03/slope-parser
git worktree add .worktrees/inimpl04-soil -b inimpl04/soil-parser
git worktree add .worktrees/inimpl05-climate -b inimpl05/climate-parser
git worktree add .worktrees/inimpl06-management -b inimpl06/management-parser
```

## 4. Registry Invariants

1. Every worker branch must map to exactly one worktree path.
2. Worker branch names are immutable for Wave 1.
3. All worker worktrees must be rooted from the same scaffold baseline commit before coding starts.
4. If a worker requires rebase, rebase target must be coordinator-designated scaffold/integration baseline.

Violation of any invariant is `HOLD` for worker start.
