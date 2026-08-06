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
- eleven package artifact/plan/prompt paths; and
- two focused snow integration tests.

The exact path list is recoverable with:

```text
for commit in 8e6007a9 3d4b6f46 478fa788 122c88af eb79ee9e 24676c6d; do
  git diff-tree --no-commit-id --name-only -r "$commit"
done | sort -u
```

No roadmap, assurance, Jennings, or terminal-meltout package change is
attributed to those six execution commits. Closeout documentation is separately
visible in the terminal diff. Mixed correction commit `2d035638` contains the
carrier snowfall/fusion corrections plus the unrelated
`tests/integration/assurance_v2_amendment_contract.rs` helper extraction needed
to clear the current-head Clippy gate.

The terminal quick gate then found that contract v8 remained stale in the
governed assurance identity. A typed `adopt-report-source` transaction added
the following closeout paths:

- `assurance/v2/identity.lock.json`;
- `assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json`;
  and
- `assurance/v2/transactions/ae69c6d2af715b5b24fd08aa3b75375671a7e50afb85f10599b0f6cdaee5a99c.json`.

Generation advanced from `f9884c05` to `910ab3d3`. The report remains
unpublished `DRAFT`, with no active event or approval roots.

## Governance Deviations

- `122c88af` added `lib.rs` and runner `00a_snow_frost_authority_impl.rs` to the
  declared write set in the same increment that first edited them.
- `24676c6d` did the same for two runner builder paths and
  `snow_surface_eb03_contract.rs`.
- Contract v8 landed at `478fa788`, but its binding guard remained at v7 until
  `24676c6d`.
- Contract v8 was not adopted into the assurance identity until the terminal
  closeout gate failed and the typed DRAFT transaction was applied.

Current source and binding tests are consistent. These are historical
prospective-sequencing defects, not authority for future same-commit write-set
widening or delayed contract guards.
