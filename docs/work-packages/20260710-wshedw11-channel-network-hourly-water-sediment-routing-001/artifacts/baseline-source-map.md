# Baseline Source Map

Status: `EXECUTED`

Evidence mode: `Static` plus `Ran` source searches against pinned commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

| Surface | Pinned source | Authority/result |
|---|---|---|
| Channel grid | `pmxchr.inc`, `wshinp.for:463-495` | `ntchr` intervals cover 86400 s exactly after `dtchr` normalization; `q1(0)` is the boundary state and `q1(1..ntchr)` routed output. |
| HBP-to-grid legacy reconstruction | `chrqin.for:82-170` | Consumes scalar volume/peak/duration and builds rectangular or double-exponential input. It does not consume arbitrary hourly bins. |
| HBP hourly-bin authority | ADR-0036 D2 | Each `V_h` is an hour-integrated volume and defines hour-mean `q_h = V_h/3600`; exact interval-overlap projection is the conservative contract route. |
| Channel dependency | `wshchr.for:231-252` | Upstream `q1(it,channel)` is added directly into downstream `qin(it)` on the shared grid. |
| Kinematic wave | `wshchr.for:395-469` | `ipeak=3`; stateful time/segment finite-difference routing plus final storage. |
| Muskingum-Cunge | `wshchr.for:473-615` | `ipeak=4` static and `ipeak=5` dynamic three-point coefficient routing; values above 5 lack named semantic authority. |
| Scalar peak branches | `wshpek.for` | `ipeak=1` Rational and `ipeak=2` CREAMS have no routed channel-series authority. |
| Routed daily water | `wshchr.for:618-720` | Peak/time/duration derive from `q1`; daily volume uses integrated outflow plus initial-minus-final storage. |
| Event sediment ingress | `chnrt.for:293-347` | Per-class event mass from runoff volume and concentration becomes constant flux over `rundur`; lateral loads are likewise event-level. |
| Event sediment solve/output | `wshdrv.for:1097-1114`, `chnrt.for:840-885` | Channel sediment executes once after water routing and publishes event class mass/concentration/fraction. No time-indexed sediment state exists. |

Conclusion: water-series routing and dependency propagation are source-
authorized for branches 3-5. Per-interval channel sediment state/mutation is not
defined by the pinned baseline and requires new canonical authority before code.

## Reproducible Commands

Ran these exact commands from `/home/workdir/openWEPP` on 2026-07-10:

```bash
git -C /workdir/wepp-forest_260430_baseline rev-parse HEAD
git -C /workdir/wepp-forest_260430_baseline diff --exit-code \
  dac3c950d8b16cc73774bf5ce2e7e11f80baac70 -- \
  src/wshchr.for src/chrqin.for src/wshscs.for src/wshpek.for \
  src/wshrun.for src/wshirs.for src/wshpas.for src/chnrt.for src/wshdrv.for
rg -n 'dtchr|ntchr|q1\(|qin\(|ipeak|Muskingum|kinematic' \
  /workdir/wepp-forest_260430_baseline/src/wshinp.for \
  /workdir/wepp-forest_260430_baseline/src/wshchr.for \
  /workdir/wepp-forest_260430_baseline/src/wshpek.for \
  /workdir/wepp-forest_260430_baseline/src/chrqin.for
rg -n 'gpart|gstu|rundur|sedcon|frcflw|q1\(|ntchr' \
  /workdir/wepp-forest_260430_baseline/src/chnrt.for \
  /workdir/wepp-forest_260430_baseline/src/wshdrv.for \
  /workdir/wepp-forest_260430_baseline/src/wshpas.for
git -C /workdir/wepp-forest_260430_baseline show \
  dac3c950d8b16cc73774bf5ce2e7e11f80baac70:src/wshchr.for >/dev/null
git -C /workdir/wepp-forest_260430_baseline show \
  dac3c950d8b16cc73774bf5ce2e7e11f80baac70:src/chnrt.for >/dev/null
```

The revision command returned
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; the scoped diff and both pinned-
object reads exited zero; the searches located the grid/water and event-only
sediment lines summarized above.

Static interpretation of those source locations is recorded separately from
the fact that the searches/revision check ran.
