# Review Disposition

Review: `review-codex.md` (Codex, 2026-07-01, independent; role-inverted —
Claude implemented, Codex reviewed). Verdict there: no production-code
rejection for F2/F5/F6/F7; independent H2637 release identity run passed all
five protected hashes (45.89 s, 82,828 KiB). Dispositions below by the
implementer; every accepted item is actioned in this same closure pass.

| # | Finding | Disposition | Action taken |
|---|---|---|---|
| A1 | Exit gate 4 unmet as written (quiet-window 3-rep; backlog deltas not written back) | **accepted** | Quiet-window 3-rep + same-window legacy anchor executed once the watershed load cleared (results appended to `gate-log.md`); `docs/backlog/20260701-hillslope-sub5x-performance-assessment.md` updated on this branch with measured deltas and per-finding outcomes. |
| A2 | `finding-dispositions.md` F5 section stale ("in progress") | **accepted** | Section rewritten to landed state with commit, gate result, and the field-replacement description. |
| D1 | SC binding-exposure lint blocked (`SC-SNOWFREEZE-001` has no Binding Exposure Index) | **accepted as substitute-evidence disposition** | The BEI absence is a pre-existing SC-SNOWFREEZE gap (SCSTRUCT PASS-DEFERRED class), not introduced here. Substitute check recorded: Codex's `rg` (and the implementer's independent grep) find no contract or doc binding to the removed `monthly_max_c`/`monthly_min_c` field names; the only `DirectFrostThermalInputs` contract binding found is `residue_depth_m`. Codex's `check_sc_unit_compliance` / `check_raw_unit_conversions` failures reproduce pre-existing findings on the unmodified contract/files (also present at the branch base), not regressions from this package. Authoring the SC-SNOWFREEZE BEI is queued as a follow-up outside this write set. |
| A3 | Line-count governance missing for touched 4,141-line `00_builders_and_authority.rs` | **accepted** | `line-count-governance.md` authored: full touched-file table; the 3000+ file entered the package at 4,137 lines (pre-existing oversize), net +4 here; split dispositioned **follow-up** to the mechanical-refactor lane. |
| D2 | One residual eager symbol construction (`frost_entry.rs:956`, landuse proxy) makes the "all eager … converted" wording overbroad | **accepted (wording) / follow-up (site)** | Wording narrowed to the converted helper families with the residual site named inline. The site executes once per solve (not per hour/layer) so its cost is negligible; converting it now would invalidate the reviewed final state for no measurable gain — queued to WP-2's contingency tail. |
| R1 | pandas follow-up under-evidenced as attributed | **accepted (attribution), evidence superseding the static search** | The implementer's captured traceback (Ran) shows the chain the reviewer's `rg` of the harness file alone cannot see: `hphys0298_paired_lineage_partition.py` → cross-package import of `hphys0265_diagnostics.py` → `import pandas` at its line 18. Artifact updated to cite the exact module/line; the lock-file follow-up stands with correct attribution. |

No finding required a production-code change; the reviewed final commit
(`2398ed44`) is unchanged by this disposition pass (docs/artifacts only).
