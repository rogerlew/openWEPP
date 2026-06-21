# R6F Disposition

Status: executed-held.

## Final Verdict

`HOLD-R6F-WAT-DIRECT-PROCESS-PRODUCER-AUTHORITY-GAP`

## Summary

R6F closed the inherited near-zero HBP byte-identity blocker and advanced
`DirectPublicationFrameCutover` to WAT on the current fixture. The current WAT
blocker is reduced to specific fields and producers. R6F also added the
direct-runtime receiving surface needed for typed process inputs and
lane-carried layer state, but the production runner still has no canonical
parsed-input producer for the WAT ET/storage/profile operands. Copying WB13 rows
or runtime surfaces would violate the R6 architecture ledger, so the package
holds and scaffolds R6G. Full R6 HBP closure still needs nonzero peak-runoff and
distinct event-duration fixture coverage.

## Completed Scope

| Scope item | Evidence |
|---|---|
| HBP near-zero fixture byte identity | `r6f-hbp-byte-diff.md`; focused runner test. |
| Climate unit correction | `02_output_and_climate_helpers.rs`; WAT reduction test now accepts `P`/`RM`/`Q`/`QOFE`. |
| Direct runtime typed input slots | `DirectPublicationDayInput` optional process inputs. |
| Lane-carried layer state | `DirectLaneFrame.subsurface_layers`; direct runtime R6F test. |
| Profile projection fields | `DirectHydrologyProjectionInputs/State/Downstream/Shadow` profile depth/porosity fields. |
| Stable WAT hold marker | runner and CLI cutover tests. |
| No compatibility shortcut | `r6f-no-compatibility-proof.md`; architecture section 5.2.1. |
| Follow-on package | `20260621-r6g-direct-wat-producer-authority-001/package.md`. |

## Premature-Stop Audit

See `no-premature-stop-audit.md`. R6F did not stop at a generic mismatch:

- The inherited near-zero HBP fixture was reduced and fixed.
- WAT was reduced to exact fields.
- In-envelope structural direct-runtime work was implemented and tested.
- The remaining implementation requires a parsed-input producer with
  `SC-EVAP-001`/`SC-SYSTEM-001` authority and cannot be replaced by
  compatibility surfaces.

## Review Disposition

Review Agent A findings were accepted and fixed in this execution pass:

- Pending review/verification files were completed before final disposition.
- HBP language was narrowed to the inherited near-zero fixture.
- WAT marker logic was changed to computed reduced fields with an exact-field
  guard.

Review Agent B findings were accepted and fixed in this execution pass:

- The required clippy gate was brought back to green instead of deferred.
- Review/verification artifacts were completed before claiming hold legitimacy.
- The R6G scaffold was expanded with kickoff prompts, artifacts, and package
  self-governance.
- No-premature-stop evidence was narrowed to the WAT producer-authority boundary.

## Verification

Focused gates and repository-level Rust/docs gates passed for this executed-held
change set. Full R6 publication parity remains blocked by WAT producer
authority, nonzero HBP fixture coverage, PASS/loss/manifest parity, and public
direct-output writes. See `gate-results.md`.
