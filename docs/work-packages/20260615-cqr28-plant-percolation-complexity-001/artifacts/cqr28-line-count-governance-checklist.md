# CQR28 Line-Count Governance Checklist

Ran: line counts and suppression census were checked with `wc -l` and `rg`.

Line counts:

| File | Before | After | Status |
|---|---:|---:|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs` | 1313 | 1541 | Under 3000 |
| `docs/work-packages/README.md` | 646 | 651 | Under advisory 1200 |
| `docs/work-packages/cqr-burndown-execplan.md` | 735 | 735 | Unchanged in package commit |

Suppression census in target file:

- Before:
  - line 1: `#[allow(clippy::wildcard_imports)]`
  - line 13: `#[allow(clippy::too_many_lines)]`
  - line 733: `#[allow(clippy::too_many_lines)]`
- After:
  - line 1: `#[allow(clippy::wildcard_imports)]`
  - line 49: `#[allow(clippy::too_many_lines)]`
  - line 769: `#[allow(clippy::too_many_lines)]`

Conclusion: no new clippy suppressions were added. Line-number shifts are from
private helper structs and extraction.
