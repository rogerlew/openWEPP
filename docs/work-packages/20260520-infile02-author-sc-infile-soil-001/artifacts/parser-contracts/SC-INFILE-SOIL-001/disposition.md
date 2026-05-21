# Disposition — SC-INFILE-SOIL-001

Evidence: Static

| finding_id | source | severity | decision | action_taken | artifact_ref | notes |
| --- | --- | --- | --- | --- | --- | --- |
| `SOL-A-001` | `review_agent_a.md` | `high` | `amend` | Expanded field table to explicit per-field rows for `7777+` / `9002+` / `9005` layer surfaces, including `theta_r`, `theta_s`, `alpha`, `npar`, `ks`, and appended Rosetta `wp`/`fc`. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:87`; `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:113` | High-severity closure complete. |
| `SOL-A-002` | `review_agent_a.md` | `high` | `amend` | Added explicit boundary-export mapping section for metadata, base OFE fields, extended layer fields, policy blocks, and restrictive-layer payloads. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:174` | High-severity closure complete. |
| `SOL-A-003` | `review_agent_a.md` | `medium` | `amend` | Added explicit mode-scoped topology authority rule defining `ntemp == nofe` (hillslope) vs `ntemp == nchan` (watershed/channel-scoped). | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:172` | Cross-file ownership ambiguity resolved in contract text. |
| `SOL-B1` | `review_agent_b.md` | `high` | `amend` | Closed by the same field-table completeness expansion implemented for `SOL-A-001`. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:87` | Duplicate high-severity finding resolved. |
| `SOL-B2` | `review_agent_b.md` | `medium` | `amend` | Added propagation-map rows for extended hydraulic/pedotransfer fields and disturbed/reveg policy payloads. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:124`; `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:127` | Ownership/phase/guard mapping now explicit for omitted fields. |
| `SOL-B3` | `review_agent_b.md` | `medium` | `amend` | Closed by explicit boundary-export mapping section added for `SOL-A-002`. | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:174` | Duplicate requirement resolved. |

## Unresolved / HOLD
- `SOL-GAP-001`, `SOL-GAP-002`, and `SOL-GAP-003` remain explicit HOLD items in the canonical contract.
