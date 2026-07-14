# Gate Results

Status: `PASS`

Evidence class: **Ran** at frozen source
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.

| ID | Gate | Result | Evidence |
| --- | --- | --- | --- |
| 00 | freeze, authority anti-evasion | source manifest fixed; PASS | `logs/final-00-*` |
| 01 | AUTH11 required-suite obligations | 3/3 | `logs/final-01-auth11.log` |
| 02 | H2637 active owner | 1/1 in 495.225 s | `logs/final-02-*` |
| 03-05 | H2637 missing/mixed/malformed authority | 1/1 each | `logs/final-03-*` through `final-05-*` |
| 06-07 | p61 and p102 erosion | 1/1 each | `logs/final-06-*`, `final-07-*` |
| 08 | erosion profile | 368/368; 1,595 skipped by profile | `logs/final-08-*` |
| 09 | frost profile | 320/320; 1,643 skipped by profile | `logs/final-09-*` |
| 10 | W7R p102 jobs/publication | 1/1 | `logs/final-10-w7r-p102-publication.log` |
| 11 | MT3 hourly consumer | 7/7 | `logs/final-11-mt3-hourly-consumer.log` |
| 12 | totalwatsed3 | 17/17 | `logs/final-12-totalwatsed3.log` |
| 13 | watershed hourly | 30/30; 99 skipped | `logs/final-13-watershed-hourly-tests.log` |
| 14 | runner package | 214/214 | `logs/final-14-runner-package.log` |
| 15 | watershed package | 129/129 | `logs/final-15-watershed-package.log` |
| 16 | exact pinned-input release candidate | exit 0 in 50:29.56; full 1,960/1,960; deny, provenance, required authority, release artifacts/lint, main 1,166/1,166, watchlist 19/19 | `logs/final-16-release-candidate.*` |
| R | independent reconstruction/consumers | PASS; no blocker | `final-conservation-and-consumer-evidence.md`, `logs/final-reconstruction-*` |

The terminal release used no skip flags. Its stability JSON SHA-256 is
`6e855d94a5d1035c58db2942dbf2668e315d861a1bf1dd6de9a4d4daf5dee6ea`;
its authority-results SHA-256 is
`b6a3605bd899590e8d85f2a52e938ba518bbb2320832fe68910d4b53369dddea`.
Earlier nonzero and invalidated candidate logs remain immutable diagnostic
evidence and are not combined with this fixed-source result.
