# Forcing Transformation Evidence

Status: `PASS`

Evidence mode: **Ran**.

| Check | Result |
|---|---:|
| synthetic transformer self-check | `PASS` |
| real-fixture preflight cells | `20 / 20` |
| executed transformed fixtures | `20 / 20` |
| maximum `scaled - source * multiplier` residual | `5.684341886080802e-14 mm` |
| protected daily-token mismatches | `0` |
| non-daily-line mismatches | `0` |
| changed-file inventory | climate file only: five `p1.cli`, ten `p2.cli`, five `p8.cli` |
| extension return-code failures | `0` |

Every extension cell retains source/scaled climate identities, source/scaled
fixture-tree identities, sanitized environment keys, command, binary/tool/freeze
identities, and output hashes in its `eb04w2-cell-provenance.json`. Event
duration, intensity shape, temperature, radiation, wind, dew point, and every
non-climate input remain protected.
