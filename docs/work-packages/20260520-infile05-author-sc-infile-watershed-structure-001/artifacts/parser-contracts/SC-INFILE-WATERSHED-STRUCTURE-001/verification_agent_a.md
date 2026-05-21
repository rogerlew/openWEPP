# Verification Agent A — SC-INFILE-WATERSHED-STRUCTURE-001

Evidence: Static

## Per-Finding Verification

| finding_id | source | severity | disposition decision | verdict | evidence (file:line) | verification note |
| --- | --- | --- | --- | --- | --- | --- |
| `WST-A-001` | `review_agent_a.md` | high | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:91`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:99` | Contributor propagation is now explicit per canonical symbol (`nhleft..nitop`) instead of one grouped contributor row. |
| `WST-A-002` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:81`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:105`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:140` | `nhmax` is now an explicit derived field with explicit propagation and corresponding taxonomy linkage. |
| `WST-A-003` | `review_agent_a.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:143`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:176`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:195` | Compatibility no-datver acceptance now has typed warning outcome and guard mapping (`STR-W-001`, `G-STR-012`). |
| `WST-B-001` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:53`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:142`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:194` | Strict row-count closure rule is now explicit with dedicated syntax error and guard (`STR-E-011`, `G-STR-011`). |
| `WST-B-002` | `review_agent_b.md` | medium | amend | closed | `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:66`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:89`, `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md:193` | `nhill` is now an explicit external-authority field with propagation and guard coverage. |

## Package Verdict
PASS-WITH-NOTES

## Remaining High-Severity Open Items
- None from reviewed findings.

Notes:
- Contract remains `HOLD` due to explicit gap-register items (`STR-GAP-*`) outside this finding-closure gate.
