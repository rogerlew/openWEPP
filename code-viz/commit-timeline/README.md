# Commit Timeline

An animated replay of openWEPP's commit history, rendered as a single static
HTML page. The sweep runs the repository forward from its first commit: the
clock advances through real author timestamps, a `git log` pane scrolls actual
commit subjects with their line counts, one bar fills per crate and per
documentation category, and the work-package counter climbs.

- **Authority:** [`specification.md`](specification.md)
- **View:** open [`dist/commit-timeline.html`](dist/commit-timeline.html) in a
  browser — it works straight from `file://`
- **Embed:** paste [`dist/commit-timeline.fragment.html`](dist/commit-timeline.fragment.html)
  into any page; it carries its own scoped styles and script

The rendered page makes **no network requests** and loads **no libraries**.

## Regenerate

```bash
python3 code-viz/commit-timeline/gen_timeline.py
```

That rewrites `data/commit-timeline.json` and both files in `dist/`, and prints
a sanity table of commit count, date range, totals, and every lane's value.

The output is a snapshot of a repository that keeps moving, so it goes stale.
Refresh it alongside significant pushes. To find out whether it already has:

```bash
python3 code-viz/commit-timeline/gen_timeline.py --check
```

Exits non-zero when the committed outputs no longer match the repository, and
writes nothing. Useful as a CI step.

Other flags: `--repo`, `--config`, `--rev`, `--out-json`, `--out-html`,
`--out-fragment`, `--template`, `--generated-at`, `--quiet`. Run with `--help`
for the full list.

## What the bars mean

A lane's value is **net lines** — cumulative additions minus deletions — so the
number beside each bar is that surface's approximate line count at that point
in history, checkable against `wc -l`.

Each bar is scaled to **its own** size at the snapshot commit. That has three
consequences worth stating plainly:

1. Every bar reaches full at the end. That end state is a snapshot of ongoing
   work, **not** progress toward a finish.
2. Bar lengths are not comparable between lanes. The largest crate is roughly
   200 times the smallest; the numbers carry magnitude, the bars carry timing.
3. A lane that shrank after a refactor pins at full while its number falls
   back. Twelve lanes do this today, the largest by 65%. The generator reports
   every one of them, and `DENOM = "peak"` in `template.html` switches to a
   denominator that makes the recession visible instead.

The `+/-` totals in the header cover **every path in the repository**. The
lanes cover crates and documentation only. Work-package artifacts — millions of
lines of checked-in run logs — are therefore counted in the header totals and
excluded from the bars, which is why the header is far larger than the sum of
the lanes. Merge commits are excluded throughout.

## Adding or changing a lane

Lanes come from [`config/openwepp.json`](config/openwepp.json), not from code.

- **A new crate or `docs/` subdirectory needs nothing.** Both groups discover
  their lanes from history, so a directory added tomorrow appears on the next
  run. Files sitting loose at either root fall into that group's shared lane.
- **Excluding a subtree** is one entry in `excludeFromLanes`; it is skipped at
  discovery, so no lane is created for it.
- **A lane outside a discovered tree** — `usersum/` is the example — is one
  entry under its group's `lanes`.
- **Retargeting another repository** is a different `--config`; the generator
  hardcodes no path, crate, or category name.

The player builds its rows from the data file at load time, so lane count and
order can change freely without touching `template.html`.

## Known limits

- Dark theme only, by design.
- Not interactive beyond replay — no scrubbing, tooltips, or drill-down.
- The data file is ~180 KB and the standalone page ~200 KB. That is the price
  of working offline with no fetch, and is accepted.
- Author timestamps are not strictly increasing — a rebase can carry an older
  date forward — so the clock steps backwards at six commits in the current
  history. That is what the history says, and it is not corrected.
