# CLAUDE.md — commit-timeline

Local notes for agents working in this directory. Read
[`../CLAUDE.md`](../CLAUDE.md) first; [`specification.md`](specification.md) is
authority and outranks both.

## The config is the contract

`config/openwepp.json` decides what lanes exist. `gen_timeline.py` contains no
crate name, category name, or directory path — adding one to the script is a
defect, not a shortcut.

`meta.lanes` in the data file is the generator↔player interface. The player
indexes into it positionally and builds every bar row from it at load time, so:

- lane count and order may change freely between runs;
- `template.html` must never hardcode a lane, a count, or an order;
- a commit's `laneDeltas` are `[laneIndex, delta]` pairs against that array, so
  anything that reorders lanes must reorder the deltas in the same pass.

## After any change

Re-run the generator and commit `data/` and `dist/` with the source change.
`--check` fails when they drift. Never hand-edit generated files.

```bash
python3 code-viz/commit-timeline/gen_timeline.py
python3 code-viz/commit-timeline/gen_timeline.py --check
```

Then actually open `dist/commit-timeline.html` and watch a sweep. The generator
can be perfectly correct while the player is broken; the specification's
verification section lists what to confirm.

## Traps found the hard way

- **`git log` is newest-first.** The sweep runs oldest-first. The generator
  reverses; do not remove that.
- **Author timestamps are not monotonic.** Six inversions exist in the current
  history, the largest stepping back 61.7 hours. `spanMs` is a maximum, not the
  last row, and anything scanning by time must walk a high-water mark.
- **Lanes shrink.** Twelve of 36 peak above their snapshot value, one by 65%.
  Any assumption that cumulative values only rise is false — see specification
  §4.2 and the `DENOM` constant.
- **Work-package artifacts are 16M+ lines.** They are excluded from lanes and
  included in the header totals, deliberately. Do not "fix" the discrepancy by
  quietly changing one side; it is disclosed in the caption.
- **No CSS transition on bar width.** The sweep repaints every frame, so a
  transition only makes each bar lag the number printed beside it.
- **`subject` can contain `|`.** The commit header split is capped at three.

## Framing

This widget is read as a claim about the project. openWEPP is a work in
progress: no completion framing, no progress-toward-done reading of a bar, and
the snapshot commit stays visible in the output. The specification opens with
the prohibition list; verification step 9 checks it.
