# Verification Agent B — SC-INFILE-IRRIGATION-FIXEDDATE-001

Evidence: Static

## Per-finding verification

| finding_id | source | disposition decision | verdict | verification evidence |
| --- | --- | --- | --- | --- |
| `FDIR-A-001` | `review_agent_a.md` | `amended_closed_with_hold` | `closed` | Mode-complete ordering behavior is explicit in constraints/policy/guards with strict `FDIR-E-010` and compat `FDIR-W-006` at `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:182`, `:203`, `:210`, and `:220`. |
| `FDIR-A-002` | `review_agent_a.md` | `amended_closed` | `closed` | Boundary mapping now uses concrete boundary surfaces and field-level mappings at `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:190-194`. |
| `FDIR-A-003` | `review_agent_a.md` | `amended_closed_with_hold` | `closed` | `iryr_interpretation_mode` is now explicit in field spec, propagation, and cross-file constraints at `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:97`, `:124`, and `:184`. |
| `FDIR-B-001` | `review_agent_b.md` | `amended_closed` | `closed` | Strict/compat contour/non-cropland furrow policy is now guard-linked through `G-FDIR-013` (`FDIR-E-009`/`FDIR-W-005`) at `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:230`. |
| `FDIR-B-002` | `review_agent_b.md` | `amended_closed_with_hold` | `closed` | Datver-floor authority conflict is now explicitly carried in HOLD register (`FDIR-GAP-004`) while policy is marked provisional at `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-IRRIGATION-FIXEDDATE-001.md:212` and `:248`; paired spec still marks this unresolved at `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/irrigation-fixeddate-file.spec.md:43` and `:201`. |

## Remaining high-severity open items

- None from review A/B findings.

## Notes

- Governance HOLD items remain by design: `FDIR-GAP-002`, `FDIR-GAP-003`, `FDIR-GAP-004`.

## Package verdict

PASS-WITH-NOTES
