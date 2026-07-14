# Independent Terminal Verification B

Status: `PASS`

Evidence class: **Static + Ran evidence audit** at frozen source
`de520f1ff867ca5c65b1f82dfe32a19c213ae18c`.

Verdict: `PASS-INTEGRATED-VALIDATION`.

No current-scope blocker remains. This verification independently checked the
six amended contracts, their contract-derived red/green bindings, the final
real-consumer and reconstruction evidence, terminal review disposition, and
the documentation/diff surface after Reviews A and B.

## Contract And Red/Green Verification

| Contract | Authority and implementation verified | Red -> green evidence |
| --- | --- | --- |
| `SC-SED-001` revisions 54/55 | `profil.for` terminal-station normalization reaches an exact normalized toe while retaining declared physical length; the bounded `1e-15` trace-load floor is renormalized to authoritative `ldbot` before caps and cannot manufacture negative class mass. | terminal normalization `100 -> 0`; trace-load composition `100 -> 0`; exact OR-H0081/H0204 replays pass; p61/p102 and erosion 368/368 pass. |
| `SC-PLANT-001` revision 20 | Finite nonnegative `rtmmax` is admitted, and the perennial saturated-cap branch is selected before incremental mass/depth division; exact zero retains the root-depth envelope without division. | zero-cap vector participates in the three-family `100 -> 0` result; exact stability cohorts and full workspace pass. |
| `SC-PERC-001` revision 30 | Every strictly positive same-pass ingress is consumed; active exact-zero restrictive conductivity returns an impermeable lower boundary while negative/non-finite domains remain typed failures. | tiny-positive ingress and daily/hourly restrictive vectors participate in the three-family `100 -> 0` result; final cohorts pass. |
| `SC-INFILE-SOIL-001` revision 0.1.12 | The parser/runtime domain is consistent: `slflag=1`, positive finite thickness, and finite `kslast>=0`; zero remains present and impermeable rather than disabling the layer. | restrictive input/runtime vectors are red before the PERC correction and green afterward; authority and final release gates pass. |
| `SC-SNOWFREEZE-001` revision 117 | A material thaw-complete `frwatc` result retains the original coarse-layer basis until R4W applies layer projection and the post-handoff liquid scalar together; nonmaterial stale clearing remains available. | the exact `1.303248764 mm` debit vector changes `100 -> 0`; eight real watchlist replays, frost 320/320, runner 214/214, and focused production frost reconstruction pass. |
| `SC-GWBASEFLOW-001` revision 0.1.2 | Enabled execution provenance retains existing `S0`, `SN`, `QbN`, and `QsN` operands; disabled output leaves them absent. Daily recurrence, HBP/pass schemas, routing, and exports are unchanged. | real H2637 fails at missing initial storage with exit 100 and passes after publication; summary posture, real recurrence, final release, and complete restart pass. |

The source diffs are narrow contract-authorized corrections, not provisional or
surrogate physics. The final full-profile run reports 1,960 tests run and
passed with three configured skips; the release invocation uses no skip or
exclude flag, and the material ignored H2637 consumer is run explicitly.

## Reconstruction And Real-Consumer Verification

- H2637 surface routing independently closes to
  `3.3223343160721848e-9 m3`. Published groundwater operands are `S0=0`,
  `SN=126.01452784040274 m3`, `QbN=5.04058111361611 m3`, and `QsN=0`.
  Recurrence-to-`SN` and complete post-export residuals are respectively
  `-4.249045559845399e-11 m3` and `-4.250466645316919e-11 m3`. The distinct
  latest-event HBP baseflow rejects that event alias as terminal-day `QbN`.
- P61 independently closes HBP sediment, concentration-times-volume, runoff,
  and five nonnegative unit-sum classes. P102 independently closes chain
  sediment/runoff/class mass and public `sed_del`/runoff against EBE; its
  nonzero difference from raw hillslope export rejects the raw-export alias.
- Selected production snow and frost WAT rows close combined storage at
  roundoff scale while excluding physical depth and internal freeze/thaw
  transfer from external-water operands.
- All 14 p102 serial/four-worker Parquet products have equal decoded row order,
  values, null posture, field types/nullability, field metadata, and schema
  metadata. HBP and pass bytes are equal; noncanonical Parquet container hashes
  are not misrepresented as semantic differences.
- The W11B CLI and typed interval consumers distinguish equal-volume
  spike/spread timing shapes, consume water and sediment on the same interval
  and class grid, and close storage. The direct two-channel test injects
  `864 m3` of baseflow upstream only and proves no downstream local re-addition.

These are real runner, manifest, HBP/pass/WAT/Parquet, watershed contribution,
typed interval, EBE/totalwatsed3, and channel-balance surfaces. Producer-only,
shadow, compatibility, scalar, zero-fill, latest-event, and raw-export aliases
do not carry acceptance.

## Authority, Review, And Terminal Disposition

**Ran evidence audit:** every final domain/reconstruction exit record is zero.
The exact release log closes full workspace 1,960/1,960, deny, fixture
provenance, required authority, release binaries/sidecars/lint, main stability
1,166/1,166, and watchlist 19/19. The terminal stability and authority hashes
were independently recomputed as
`6e855d94a5d1035c58db2942dbf2668e315d861a1bf1dd6de9a4d4daf5dee6ea`
and `b6a3605bd899590e8d85f2a52e938ba518bbb2320832fe68910d4b53369dddea`.
Anti-evasion passes, AUTH11 passes 3/3, and the three named H2637
missing/mixed/malformed cases each pass their fail-closed selection.

Comparator agreement remains a diagnostic flag. Acceptance rests on canonical
authority, mechanism-specific red/green vectors, independent arithmetic, and
real consumers; the trace-load correction deliberately does not target invalid
negative legacy mass.

Review A has no correction finding. Review B findings `INTVAL-RB-001` and
`INTVAL-RB-002` are accepted and corrected: the scenario matrix retains exact
commands, fixtures, consumer paths, evidence, and logs, and says `no skip
flags`. During this verification, the same terminology was corrected in the DC
progress and integrated assessment; it now agrees with the observed three
configured skips. No finding is deferred or left for current-scope follow-up.

The final assessment/disposition, work-package catalog, and forward-only
roadmap consistently support `PASS-INTEGRATED-VALIDATION`; old HOLD and
invalidated restart artifacts remain historical only. Uncommitted terminal
changes are confined to the intended package/evidence/catalog/roadmap scope;
there is no post-freeze source edit.

## Scoped Checks And Blockers

**Ran:** `markdown-doc lint` over both package trees, the work-package catalog,
and the roadmap validated 43 files with zero errors and zero warnings.
**Ran:** `git diff --check` passed.

Blockers: none.
