# Independent Terminal Review B

Status: `PASS`

Evidence class: **Static** artifact/source audit plus **Ran** checksum and log-
exit inspection. Frozen source:
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.

Verdict: `PASS-INTEGRATED-VALIDATION`.

No unresolved current-scope finding remains. This review does not replace the
package-required independent verification or final clean-worktree gate.

## Findings And Disposition

| ID | Severity | Finding | Disposition |
| --- | --- | --- | --- |
| `INTVAL-RB-001` | moderate, closure-blocking when found | The first terminal rewrite of `scenario-matrix.md` had removed the package-required exact command, fixture/output, producer-to-consumer, required-evidence, and log bindings. | `accepted` and fixed before this verdict. The matrix now records those fields for every terminal lane, including all five focused reconstruction selections and the complete pinned-input release invocation. |
| `INTVAL-RB-002` | low, evidence-exactness | The release row initially said `no skips`, although the final full profile reports 1,960 tests run and passed with three configured skips; the protected requirement is no skip flags. | `accepted` and fixed before this verdict. The matrix now says `no skip flags` and preserves the exact observed counts. |

There are no `deferred` or `follow-up` findings.

## Fixed-Source And Gate Audit

- `logs/final-00-freeze-manifest.log` records the exact frozen commit, tool
  versions, pinned `/workdir/wepp-forest` commit
  `375ccc296ed1ea491f599ff1b1a25b415d494a2a`, and both required CSV hashes.
- Every `final-00` through `final-16` exit record is zero. The domain logs
  support H2637 positive 1/1 and three negative 1/1 selections, p61 1/1,
  p102 1/1, erosion 368/368, frost 320/320, W7R 1/1, MT3 7/7,
  totalwatsed3 17/17, watershed hourly 30/30, runner 214/214, and watershed
  129/129.
- The exact release log closes full nextest 1,960/1,960, required authority,
  release artifacts/lint, main stability 1,166/1,166, and watchlist 19/19 in
  50:29.56 with no skip argument. The observed stability JSON SHA-256 is
  `6e855d94a5d1035c58db2942dbf2668e315d861a1bf1dd6de9a4d4daf5dee6ea`;
  the authority result SHA-256 is
  `b6a3605bd899590e8d85f2a52e938ba518bbb2320832fe68910d4b53369dddea`.
- `scenario-matrix.md` and `gate-results.md` now agree with the logs and keep
  `restart-*` plus earlier HOLD/candidate records explicitly non-terminal.
  No pre-correction groundwater conclusion or interrupted release result is
  combined into terminal acceptance.

## Independent Reconstruction And Consumer Audit

The final reconstruction reads fresh manifest, HBP, pass, WAT, and Parquet
outputs rather than a producer aggregation helper.

- H2637 surface routing closes with `3.3223343160721848e-9 m3` residual. The
  published groundwater operands are `S0=0`,
  `SN=126.01452784040274 m3`, `QbN=5.04058111361611 m3`, and `QsN=0`.
  The recurrence-to-`SN` residual is
  `-4.249045559845399e-11 m3`; the post-export full-run residual is
  `-4.250466645316919e-11 m3`. The distinct latest runoff-event HBP baseflow
  (`5.032033091000001 m3`) rejects that event alias as `QbN`.
- P61 closes HBP sediment, concentration-times-volume, runoff volume, and the
  five nonnegative unit-sum classes. P102 closes chain sediment and runoff,
  five-class mass, and routed public `sed_del`/runoff against EBE; the
  `0.04453260539128223 kg` difference from raw hillslope export rejects the
  raw-export publication alias.
- Selected production snow and frost WAT rows close combined storage at
  roundoff scale and keep physical snow depth, frost depth, and internal
  freeze/thaw transfer out of external-water operands.
- All 14 serial/four-worker Parquet products have equal decoded row order,
  values, null posture, field types/nullability, field metadata, and schema
  metadata. HBP and pass bytes are equal; differing Parquet container hashes
  are truthfully classified as noncanonical serialization metadata rather
  than semantic inequality.
- The real W11B CLI and typed interval consumers distinguish equal-volume
  spike/spread shapes, consume upstream egress on the same interval/class
  grid, and reconstruct water and sediment. The direct two-channel test injects
  `864 m3` of baseflow upstream only and proves no downstream local re-addition.

The producer-to-manifest/HBP/pass/WAT handoffs, runner intake, watershed
`HillslopeContribution`, typed interval state, EBE/totalwatsed3, and channel-
balance surfaces are all named. Rejected compatibility, diagnostic, scalar,
latest-event, raw-export, physical-depth, and zero-fill aliases are explicit.

## Contract And Governance Coverage

All six amended contracts have contract-first authority, a red/green binding,
and terminal frozen-source coverage:

| Contract | Terminal coverage |
| --- | --- |
| `SC-SED-001` revisions 54/55 | terminal-station and trace-load class regressions; p61/p102; erosion 368/368; final stability 1,185/1,185 across both suites |
| `SC-PLANT-001` revision 20 | zero-`rtmmax` saturated-cap regression; final full workspace and exact stability cohorts |
| `SC-PERC-001` revision 30 | tiny-positive ingress and zero-conductivity restrictive-boundary regressions; final full workspace and exact stability cohorts |
| `SC-INFILE-SOIL-001` revision 0.1.12 | exact-zero restrictive-input acceptance paired with the PERC boundary; final required authority/full/stability gates |
| `SC-SNOWFREEZE-001` revision 117 | material thaw-complete ordering regression; frost 320/320, runner 214/214, production frost reconstruction, and final full/stability gates |
| `SC-GWBASEFLOW-001` revision 0.1.2 | enabled/disabled summary posture and real H2637 publication of `S0`, `SN`, `QbN`, `QsN`; both recurrences independently reconstruct |

The final anti-evasion guard and AUTH11 3/3 pass. No suite deactivation,
fixture-result edit, tolerance relaxation, fallback, old symbol-map runtime,
compatibility wrapper, silent canonicalization, or skip flag carries closure.
Comparator agreement remains a flag: acceptance rests on canonical contracts,
contract-derived vectors, real-case execution, independent conservation, and
real downstream consumers. The trace-load class correction deliberately does
not reproduce invalid negative legacy mass.

## Administrative Closure Audit

The integrated assessment and disposition consistently recommend
`PASS-INTEGRATED-VALIDATION` at `de520f1f`; the work-package catalog records
both the completed campaign and terminal DC package; and the completed
`INTVAL-20260713` row has been removed from the forward-only roadmap. Earlier
HOLD review/verification artifacts remain historical provenance and are not
represented as terminal review evidence.
