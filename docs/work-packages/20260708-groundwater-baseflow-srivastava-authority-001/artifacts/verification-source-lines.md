# Verification: Source Lines

Status: PASS.

Verifier: subagent `019f43b5-3582-71a3-85ea-90b75f3c250f`.

Read-only verification. No files edited.

## Commands Used

```bash
git -C /workdir/wepp-forest_260430_baseline rev-parse HEAD

for f in src/main.for src/cchrt1.inc src/contin.for src/wshpas.for \
  src/wshdrv.for src/cstore2.inc src/wshchr.for src/wshcqi.for \
  src/watbalprint.for; do
  test -f "/workdir/wepp-forest_260430_baseline/$f" && printf 'EXISTS %s\n' "$f"
done

nl -ba /workdir/wepp-forest_260430_baseline/src/main.for | sed -n '120,136p;450,465p'
nl -ba /workdir/wepp-forest_260430_baseline/src/cchrt1.inc | sed -n '7,17p;31,52p'
nl -ba /workdir/wepp-forest_260430_baseline/src/contin.for | sed -n '1088,1120p'
nl -ba /workdir/wepp-forest_260430_baseline/src/wshpas.for | sed -n '220,227p;236,245p;255,265p;386,414p;466,505p;530,532p'
nl -ba /workdir/wepp-forest_260430_baseline/src/wshdrv.for | sed -n '515,520p;845,875p'
nl -ba /workdir/wepp-forest_260430_baseline/src/cstore2.inc | sed -n '7,15p;29,32p'
nl -ba /workdir/wepp-forest_260430_baseline/src/wshchr.for | sed -n '133,148p;183,189p;205,225p;260,262p;696,704p'
nl -ba /workdir/wepp-forest_260430_baseline/src/wshcqi.for | sed -n '86,159p;199,207p'
nl -ba /workdir/wepp-forest_260430_baseline/src/watbalprint.for | sed -n '87,96p;101,124p'
```

## Results

| Check | Result |
|---|---|
| Baseline repository SHA | PASS: `/workdir/wepp-forest_260430_baseline` is `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. |
| Required source files | PASS: all requested files exist under `/workdir/wepp-forest_260430_baseline/src/`. |
| `main.for` branch selection | PASS: listed lines support `gwcoeff.txt` open/read/error branch claims. |
| `cchrt1.inc` symbols and units | PASS: listed lines define common-block symbols and comments for coefficients/threshold. |
| `contin.for` recurrence | PASS: listed lines support daily update, `sep` recharge, `gwbfv`/`gwdsv`, carry, and disabled zeroing claims. |
| `wshpas.for` pass fields | PASS: listed ranges write/read `gwbfv` and `gwdsv` through pass paths. |
| `wshdrv.for` and `cstore2.inc` staging | PASS: listed lines support `chntyp.txt`, `tmpgwbfv`/`tmpgwdsv`, reset, and temporary volume-unit claims. |
| `wshchr.for` channel routing | PASS: listed lines support `cbase` separation, generated `tmpgwbfv`, `bftharea`, `86400` conversion, and `qbase=0` under `lr_bf=1`. |
| `wshcqi.for` channel inflow/quality | PASS: listed lines support generated side/top baseflow consumption, phosphorus coupling, `bftharea`, and `qBase`/`rvotop` claims. |
| `watbalprint.for` publication | PASS: listed lines support `baseflow` from `qBase` only under `lr_bf=0`, with `baseflow=0.0` under `lr_bf=1`. |

Disposition: file:line claims in `baseline-code-map.md` are supported by the
pinned baseline source.
