# Verification Agent A — SC-INFILE-WATERSHED-CHANNEL-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
| --- | --- | --- | --- | --- | --- | --- |
| `CHN-A-001` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:105`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:132` | Propagation map is now explicit at symbol level (header, comments, mode, `chn*`, `ctl*`, `rc*`, and derived fields), not grouped bundles only. |
| `CHN-A-002` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:96`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:130`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:193` | Derived `has_rating_curve` and `control_override_applied` now have explicit propagation and boundary export representation. |
| `CHN-A-003` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:169`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:205`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:229` | Typed compatibility outcomes are now explicit in taxonomy/policy/guards (`CHN-W-001..005`, `G-CHN-015`). |
| `CHN-B-001` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:129`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:130` | Duplicate derived-field propagation concern is closed by explicit derived rows. |
| `CHN-B-002` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:98`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:131`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:193` | `sidecar_required` is now explicit in field spec, propagation, and boundary export. |
| `CHN-B-003` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:138`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:194`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md:228` | Non-mutating `tcr.txt` overlay now has explicit ownership surface, boundary export mapping, and dedicated guard. |

## Package Verdict
PASS-WITH-NOTES

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- Contract remains `HOLD` due to explicit gap-register items (`CHN-GAP-*`) outside this finding-closure gate.
