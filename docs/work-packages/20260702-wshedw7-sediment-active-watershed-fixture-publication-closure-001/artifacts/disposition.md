# Disposition

Status: `EXECUTED-HOLD-HILLSLOPE-SEDIMENT-PRODUCTION-MISSING`

Evidence mode: `Ran:` fixture probes, focused validation, and docs lint;
`Static:` source/package review.

## Summary

W7 is executed and held, not complete.

What landed:

- Fixed generated hillslope child input path resolution for relative public
  `--run-dir` invocations.
- Added
  `wshedw7_watershed_cli_generated_mode_accepts_relative_run_dir`.
- Rebuilt release CLIs and reran the committed carnivorous public watershed
  fixture successfully after the path fix.
- Probed committed and local sediment candidates and recorded that current
  openWEPP-generated pass sediment remains zero.
- Scaffolded W7DC01 as the hold-lift package.

Hold blocker:

`WSHED-W7-HOLD-001`: production hillslope HBP sediment emission remains zero
for inspected sediment-active source substrates. W7 cannot adopt a committed
sediment-active watershed fixture, run serial/parallel sediment output identity,
or perform independent nonzero sediment reconstruction until this is fixed or
held under canonical sediment authority.

Why hold is legitimate:

- W7 explicitly excludes changing hillslope erosion/sediment physics for
  convenience.
- Legacy source evidence for `/wc1/runs/in/insensible-aliquot/wepp/output/H1.loss.dat`
  is sediment-active, but current openWEPP HBP pass parquet probes for
  multi-OFE hillslopes `1`, `21`, `172`, `297`, `333`, `390`, and `437` all
  produced zero `tdet`, `tdep`, and `sedcon_*`.
- Closing W7 on zero-only fixtures would violate the package objective and the
  Gate Evidence Non-Deferral Rule.

Next package:

- `docs/work-packages/20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/`
