# Owned File And Increment Manifest

Evidence class: Static, reconciled 2026-08-06.

## Execution Increments

| Commit | Purpose |
| --- | --- |
| `8e6007a9` | Scaffold and prospective freeze. |
| `3d4b6f46` | Carrier seam inventory and initial input-authority hold. |
| `478fa788` | Contract v8 turbulent geometry authority. |
| `122c88af` | Typed runner-to-kernel geometry implementation. |
| `eb79ee9e` | Default-off complete-carrier shadow. |
| `24676c6d` | Within-day sequential cold-content/fusion shadow and initial Snowbird evaluation. |
| `2d035638` | Closeout correction for snowfall SWE mass, fusion heat, and an unrelated current-head Clippy test structure defect; package claims narrowed to executed HOLD. |

The corrected result-bearing binary is bound to full source commit
`2d035638a9819961a393207cd4813712d64bddcf`. The first six package commits
contain `1,364` added and `54` deleted lines across the following 21 paths:

- six Rust runner/orchestrator paths;
- two canonical snow-energy contract/index paths;
- ten package artifact/plan/prompt paths; and
- two focused snow integration tests.

The exact path list is recoverable with:

```text
for commit in 8e6007a9 3d4b6f46 478fa788 122c88af eb79ee9e 24676c6d; do
  git diff-tree --no-commit-id --name-only -r "$commit"
done | sort -u
```

No roadmap, assurance, Jennings, or terminal-meltout package change is
attributed to those six execution commits. Closeout documentation and the
current-head assurance test-helper extraction are separately visible in the
terminal closeout diff.

## Governance Deviations

- `122c88af` added `lib.rs` and runner `00a_snow_frost_authority_impl.rs` to the
  declared write set in the same increment that first edited them.
- `24676c6d` did the same for two runner builder paths and
  `snow_surface_eb03_contract.rs`.
- Contract v8 landed at `478fa788`, but its binding guard remained at v7 until
  `24676c6d`.

Current source and binding tests are consistent. These are historical
prospective-sequencing defects, not authority for future same-commit write-set
widening or delayed contract guards.
