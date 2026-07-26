# Commit Timeline Visualization Specification

Status: Draft
Last updated: 2026-07-24
Scope: `code-viz/commit-timeline/` generator, data interchange file, HTML
template, and rendered distribution artifacts

## Purpose

openWEPP is built by agents under human direction, in the open, at a cadence
that is hard to convey in prose. This visualization replays the repository's
commit history as a single animated sweep: the clock advances through real
commit timestamps, a `git log` ticker scrolls actual commit subjects with their
line counts, per-crate and per-documentation-category bars fill as those
surfaces grow, and the work-package counter climbs.

The artifact is **static**. A generator reads git history once and emits a JSON
data file; the player is pre-rendered HTML with that data inlined. The rendered
page performs zero network requests, loads zero libraries, and runs from
`file://`.

This document is authority for the generator, the interchange schema, and the
player. Implementation must not diverge from it; when a change is needed, this
document is revised first.

## Work-in-Progress Framing

openWEPP is an active work in progress. Every figure this visualization
produces is a snapshot at a named commit, and the artifact must say so.

The following are **prohibited** in the generator, the template, the rendered
output, and the surrounding documentation:

- Any framing that implies the project is finished, shipped, or complete.
- Any "progress toward a goal" reading of a bar. A bar shows how a surface grew
  to its size at the snapshot commit, not how close it is to done.
- Elapsed-time bragging ("built in N days") as the headline claim.
- Hand-copied statistics anywhere except a clearly dated appendix.

The rendered widget must display the snapshot commit and its date, and must
carry a caption stating that bar lengths are scaled to each lane's size at that
commit.

## Non-Goals

- Interactivity beyond a replay control. No scrubbing, no tooltips, no
  hover states, no per-file drill-down.
- Light theme. The widget is dark-only by design.
- Any build system, package manager, bundler, or CSS framework.
- Commit-density heatmaps, contributor breakdowns, or language breakdowns.
  These are plausible sibling visualizations under `code-viz/`, not part of
  this one.

## Terminology

| Term | Meaning |
| --- | --- |
| Lane | One animated bar. Backed by a set of path prefixes. |
| Group | An ordered set of lanes rendered as one labeled column. |
| Counter | A scalar that accumulates over the sweep and renders as a wide bar plus `N / total`. |
| Sweep | One full playthrough of the commit history. |
| Batch | A contiguous run of commits applied to the DOM in a single frame. |
| Snapshot commit | The revision the data file was generated from. Recorded as `meta.head`. |

## 1. Data Source and Extraction

### 1.1 Command

History is read from the local repository, never from the GitHub API. The
history is thousands of commits; a per-commit REST call would be rate-limited,
slow, and would require network access at generation time for data that is
already on disk.

The generator invokes exactly one git command:

```
git log --no-merges --numstat --format=C|%H|%at|%s <rev>
```

- `--no-merges` — merge commits duplicate the churn of their parents.
- `--numstat` — machine-readable per-file added/deleted counts.
- `%at` — author timestamp, seconds since epoch, UTC.
- The `C|` prefix disambiguates commit-header lines from numstat lines.

`git log` emits newest-first. The generator must reverse the commit sequence
before accumulating, so that cumulative values increase over the sweep.

### 1.2 Numstat parsing rules

- A numstat line is exactly three tab-separated fields: `added`, `deleted`,
  `path`.
- Binary files are reported as `-` for both counts and must be recorded as
  `0` added and `0` deleted.
- Rename detection stays **enabled**. A pure rename is reported as `0 0` with
  a brace path form, which correctly contributes no churn.
- Brace path forms must be normalized to the **new** path before lane
  matching. Both shapes occur:
  - `dir/{old => new}/file.rs` → `dir/new/file.rs`
  - `{old/path => new/path}` → `new/path`
  - An empty side is legal: `dir/{ => sub}/f.rs` → `dir/sub/f.rs`
- A commit subject may contain any character including `|`. The commit header
  line must be split on `|` with a maximum of three splits so the subject is
  preserved verbatim.
- Commits with no numstat lines (empty commits) are retained in the commit
  sequence with zero deltas, so commit indices stay faithful to history.

## 2. Configuration

All repository-specific knowledge lives in a JSON config file. The generator
contains no hardcoded crate names, directory names, or category names, and
must be usable against another repository by supplying a different config.

### 2.1 Schema

```json
{
  "title": "openWEPP",
  "repo": "rogerlew/openWEPP",
  "tz": "America/Los_Angeles",
  "groups": [
    {
      "key": "crates",
      "label": "crates",
      "lanes": [
        { "key": "tests-integration", "label": "tests/integration",
          "paths": ["tests/integration/"] }
      ],
      "autoDiscover": { "kind": "dir-children", "path": "crates/" },
      "sharedLaneLabel": "crates (shared)"
    },
    {
      "key": "docs",
      "label": "documentation",
      "lanes": [
        { "key": "usersum", "label": "usersum", "paths": ["usersum/"] }
      ],
      "autoDiscover": { "kind": "dir-children", "path": "docs/" },
      "sharedLaneLabel": "docs (loose files)"
    }
  ],
  "counters": [
    {
      "key": "workPackages",
      "label": "work packages",
      "kind": "dir-count",
      "path": "docs/work-packages/",
      "exclude": ["series"]
    }
  ],
  "excludeFromLanes": ["docs/work-packages/"],
  "labelStrip": ["openwepp-"]
}
```

### 2.2 Field semantics

| Field | Required | Meaning |
| --- | --- | --- |
| `title` | yes | Display name in the widget header. |
| `repo` | no | `owner/name`, recorded in `meta` only. |
| `tz` | yes | IANA zone used by the widget clock. |
| `groups[].key` | yes | Stable identifier; also selects the color family. |
| `groups[].label` | yes | Column heading. |
| `groups[].lanes` | no | Explicitly declared lanes, in declaration order. |
| `groups[].autoDiscover` | no | Lane discovery rule; see §3.1. |
| `groups[].sharedLaneLabel` | no | Label for the residual lane; see §3.2. |
| `counters[]` | no | Scalar accumulators; see §5. |
| `excludeFromLanes` | no | Path prefixes excluded from every lane. |
| `labelStrip` | no | Prefixes stripped from auto-discovered lane labels. |

A group may declare `lanes`, `autoDiscover`, or both. Config validation is
fail-closed: an unknown key, a missing required field, or a group with neither
`lanes` nor `autoDiscover` is an error, not a warning.

## 3. Lane Resolution

### 3.1 Auto-discovery

`{"kind": "dir-children", "path": "crates/"}` enumerates every immediate child
directory of that path observed **anywhere in the requested history**, not
merely at the snapshot commit. Discovery over history means:

- A directory added after this document was written appears on the next
  generator run with no code or config edit.
- A directory that was removed is not silently erased from the history it
  contributed to.

Both groups use it: the crates group discovers `crates/`, and the
documentation group discovers `docs/` while declaring `usersum/` explicitly,
since end-user documentation lives outside the `docs/` tree.

Discovery honors `excludeFromLanes`, so an excluded subtree never becomes a
lane at all. Leaving it to be discovered and then dropped by the zero-value
filter in §3.4 would reach the same output by accident rather than by rule.

Discovery uses the same numstat stream as accumulation; no second git
invocation is required.

Auto-discovered lane labels are the directory name with each `labelStrip`
prefix removed, longest match first.

### 3.2 Residual lane

Files that sit directly under an auto-discovery `path` and therefore belong to
no child directory (for example `crates/AGENTS.md`) accumulate into a residual
lane labeled by `sharedLaneLabel`. The residual lane is emitted only when its
value at the snapshot commit is positive.

### 3.3 Matching

- A path matches a lane when it starts with any of that lane's path prefixes.
- `excludeFromLanes` is evaluated first and wins. A path matching an exclusion
  prefix contributes to no lane.
- Explicit lanes are evaluated before auto-discovered lanes.
- Matching stops at the first lane that matches; lanes must not double-count.
- A path matching no lane contributes only to the global totals in §4.3. This
  is expected for `tests/`, `tools/`, `assurance/`, `gate-policy/`,
  `references/`, `papers/`, `.github/`, and repository-root files.

`docs/work-packages/**` is excluded from all lanes. Its `artifacts/` subtrees
hold checked-in run logs measured in millions of lines, which would dominate
every other lane by more than two orders of magnitude. Work-package activity is
represented by the counter in §5 instead.

### 3.4 Ordering

- Groups render in config order.
- Lanes within a group render in descending order of their value at the
  snapshot commit.
- A lane whose value at the snapshot commit is less than or equal to zero is
  omitted from the output and reported on stderr, with its key and value.

## 4. Metrics and Scaling

### 4.1 Lane metric

A lane's value is **cumulative net lines**: the running sum of additions minus
deletions across the lane's matched paths. The value at the snapshot commit is
therefore that surface's approximate line count in the working tree, which
makes the number on each bar independently checkable against `wc -l`.

### 4.2 Bar scaling

Each bar is normalized to its own value at the snapshot commit:

```
width = clamp(cumulative / denominator, 0, 1) * 100%
```

Consequences that must be documented in the widget caption and in the README:

- Every bar reaches 100% at the end of the sweep. The end state is the
  snapshot, not completion.
- Bars are not comparable to each other in length. The numeric label carries
  magnitude; the bar carries timing.

**Lanes that shrink.** A lane can exceed its snapshot value mid-history and
then fall back, when a refactor removes more than it adds. This is not
hypothetical: at `497d76d0`, 12 of 36 lanes peak above their snapshot value.
The largest proportionally is `docs (loose files)`, which peaks 65% above its
snapshot as root documents were promoted into subdirectories; the largest
absolutely is `hillslope-orchestrator`, peaking at 83,227 against a snapshot of
70,313.

The generator therefore records `peak` alongside `head` for every lane, and the
template exposes a `DENOM` constant:

| `DENOM` | Denominator | Behavior for a lane that shrank |
| --- | --- | --- |
| `head` (default) | value at snapshot | Bar pins at full while its number falls back |
| `peak` | `max(peak, head)` | Bar recedes visibly and ends short of full |

`head` is the default: the great majority of lanes never shrink, and every bar
arriving at full is the intended reading of the end state. The cost is that a
pinned bar briefly overstates a shrinking lane. The number beside the bar is
always the truth, and the sanity table flags every lane whose peak exceeds its
snapshot value so the tradeoff is never invisible.

The template exposes a `SCALE` constant with values `per-lane` (default) and
`per-group`. Under `per-group` all lanes in a group share one axis scaled to
the largest lane in that group. This is a one-line change and is not the
default because openWEPP's largest crate is roughly 200 times its smallest,
which would render most lanes as invisible slivers.

### 4.3 Global totals

Additions and deletions are accumulated across **all** paths, including paths
that match no lane and paths under `excludeFromLanes`. The header therefore
reports total repository churn, while the bars report lane churn. The README
must state this asymmetry.

## 5. Counters

A counter of `kind: "dir-count"` accumulates the number of distinct immediate
child directories of `path` observed so far, crediting each directory to the
commit where it first appears. Names listed in `exclude` are skipped, as are
files sitting directly under `path`.

For openWEPP this yields the count of `docs/work-packages/<id>/` directories:
one increment per work package, at the commit that created it. It renders as a
full-width bar plus `N / <value at snapshot commit>`.

## 6. Data Interchange Schema

Version 1. The generator writes `data/commit-timeline.json`.

```json
{
  "schemaVersion": 1,
  "meta": {
    "title": "openWEPP",
    "repo": "rogerlew/openWEPP",
    "branch": "main",
    "head": "497d76d0c29d2f711f4b0ac3f63454960793fe97",
    "headShort": "497d76d0",
    "headDate": "2026-07-24T13:05:59-07:00",
    "firstDate": "2026-05-11T09:37:52-07:00",
    "generatedAt": "2026-07-24T21:00:00Z",
    "rev": "HEAD",
    "commitCount": 2077,
    "mergesExcluded": true,
    "t0": 1778517472000,
    "spanMs": 6413287000,
    "tz": "America/Los_Angeles",
    "totals": { "add": 21935670, "del": 346315 },
    "groups": [
      { "key": "crates", "label": "crates", "laneRange": [0, 21],
        "color": "#d4794a", "head": 236138, "peak": 236149 },
      { "key": "docs", "label": "documentation", "laneRange": [21, 26],
        "color": "#6a93e0", "head": 53635, "peak": 53635 }
    ],
    "lanes": [
      { "key": "openwepp-hillslope-orchestrator",
        "label": "hillslope-orchestrator",
        "group": "crates", "head": 70313, "peak": 83227 }
    ],
    "counters": [
      { "key": "workPackages", "label": "work packages", "head": 959 }
    ]
  },
  "commits": [
    [3600000, 412, 87, [1], [[0, 325], [21, 87]], "feat(hillslope): …"]
  ]
}
```

### 6.1 Field rules

- `t0` — author timestamp of the oldest commit, in milliseconds.
- `spanMs` — the largest `dtMs` in the sequence. It is computed as a maximum,
  not read off the last row: author timestamps are **not** monotonic in history
  order, because a rebase or cherry-pick can carry an older author date
  forward. At `497d76d0` there are 6 such inversions in 2,076 transitions, the
  largest stepping back 61.7 hours. The clock is allowed to step backwards at
  those commits — that is what the history says — but nothing that scans by
  time may assume sorted input.
- `laneRange` — half-open `[start, end)` index range into `meta.lanes`.
- `groups[].head` / `groups[].peak` — the group's own cumulative total and its
  largest extent over history, carried explicitly so a consumer of the data
  file does not have to re-derive them by summing lanes. `head` equals the sum
  of its lanes' `head`; `peak` is the peak of the **group sum**, which is not
  the sum of the lanes' peaks, because lanes peak at different commits.
- `meta.lanes` order is the **contract** between generator and player. The
  player indexes into it positionally and builds its bar rows from it at load
  time. Lane count and order may change freely between runs; the template must
  never hardcode either.
- A commit tuple is `[dtMs, add, del, counterDeltas, laneDeltas, subject]`:
  - `dtMs` — author timestamp minus `t0`, in milliseconds.
  - `add`, `del` — whole-commit totals over all paths.
  - `counterDeltas` — dense array parallel to `meta.counters`.
  - `laneDeltas` — **sparse** array of `[laneIndex, netDelta]` pairs, omitting
    lanes this commit did not touch. Sparse because the median commit touches
    one or two of twenty-five lanes; a dense array would be almost entirely
    zeros.
  - `subject` — truncated to 72 characters, with a trailing `…` when truncated.

### 6.2 Size

The openWEPP snapshot produces roughly 200 KB of JSON and a comparably sized
standalone HTML file. This is accepted: the artifact must work offline, from
`file://`, with no fetch. Any future size reduction must not introduce a
network request.

## 7. Player Behavior

### 7.1 Playback

- One `requestAnimationFrame` loop drives the sweep. Elapsed time maps linearly
  to a target commit index.
- `DUR` is a fixed sweep length of approximately 26 seconds, independent of
  commit count.
- Application is **frame-coalesced**: every commit between the cursor and the
  target is folded into the numeric model, then the DOM is painted once, and
  only for the lanes and counters that actually changed. Per-frame DOM cost is
  therefore bounded by the number of lanes, not by the number of commits, and
  stays flat as history grows.
- Progress through the sweep is linear, not eased. An ease curve would make the
  commit rate misrepresent itself — the sweep is a reading of history, and its
  only honest tempo is a constant one.
- `PACING` is a named constant with values `index` (default) and `time`. Under
  `index`, progress is linear in commit number. Under `time`, progress is
  linear in `dtMs`.

  `index` is the default because openWEPP's commit rate is strongly
  front-loaded away from the start: 372 commits in the first month against
  1,037 in the third. Time pacing leaves the opening third of the sweep nearly
  static. The clock label carries the true chronology in both modes.

### 7.2 Controls and lifecycle

- A single `<button>` control. Label is `▶ replay` before the first play and
  `↻ replay` afterwards. Its `min-width` must be pinned to the measured width
  before the first label swap so the change does not reflow the header.
- The sweep autoplays once when the widget first scrolls into view, via
  `IntersectionObserver`. Subsequent plays are user-initiated only.
- `rewind()` must fully reset lane values, counters, ticker rows, clock, and
  header numbers. Clicking replay mid-sweep must cancel the running sweep
  before rewinding; no two sweeps may run concurrently.
- On completion the player must set the exact snapshot values from `meta`
  rather than the accumulated running values, so floating point or batching
  cannot leave the end state off by a line.

### 7.3 Reduced motion

When `prefers-reduced-motion: reduce` matches:

- The widget must not autoplay.
- The button remains functional, and activating it applies all batches
  immediately and lands on the snapshot state with no animation.
- CSS transitions on bars and counters must be suppressed.

The widget must be fully legible in its pre-play state, since a reduced-motion
visitor who never clicks will see only that state. The pre-play state therefore
renders all lane labels and the snapshot commit identity.

### 7.4 Rendering surfaces

| Surface | Content |
| --- | --- |
| Header badge | `title`, `work in progress`, commit count, first date → snapshot date |
| Commit position | `commit N / total` |
| Churn | `+A −D`, cumulative, additions green, deletions red |
| Clock | `Intl.DateTimeFormat` in `meta.tz`, weekday, month, day, time |
| Ticker | Full-width scrolling `git log` pane above the group columns, most recent commit at the bottom, each row `+A −D subject` |
| Group heading | Group label, plus the unit its values are measured in, right-aligned over the value column |
| Lane bars | One row per `meta.lanes` entry: label, track, fill, value |
| Group total | A closing row in each group, in the same three columns as its lanes, carrying the group's running total against `groups[].head` |
| Counter bars | One full-width row per `meta.counters` entry |
| Caption | Scaling disclosure and snapshot identity, per §"Work-in-Progress Framing" |

A lane with no activity yet renders dimmed — label and empty track — and
un-dims on its first nonzero delta. Lanes are never hidden or evicted; the
appearance of a new crate mid-sweep is part of what the visualization shows.

The ticker spans the full width above the group columns, and its height is
**derived, not fixed**: it is `LOG_RATIO` of the measured height of the group
columns, recomputed through a `ResizeObserver`. Lane count comes from the data,
so the tables' height is not knowable at authoring time; a hardcoded ticker
height would go wrong the moment a crate is added or the layout reflows. The
ticker is bottom-anchored so anything that does not fit overflows off the top,
never hiding the newest commit, and the clipped edge is faded so a partial top
row reads as history scrolling past rather than as a rendering fault.

## 8. Presentation Constraints

- No JavaScript framework, no CDN, no external font, no external stylesheet,
  no image request. The rendered page must issue zero network requests.
- All CSS ships in one `<style>` block, with every selector prefixed by the
  section's `id`, so the fragment can be pasted into an arbitrary host page
  without collision and without inheriting host styles.
- Dark-only. The section declares `color-scheme: dark` and sets its own
  background rather than relying on the host page.
- Color encodes **group**, not lane. Twenty-five lanes is far beyond the number
  of hues a reader can distinguish categorically, so each group is one flat
  hue and the always-present lane label carries identification.

  Fill color must not vary with a lane's rank within its group. Rank is a
  property of the current snapshot, not of the lane; tying color to it means a
  lane changes color when an unrelated lane overtakes it. Color follows the
  entity.

- The palette is validated, not chosen by eye. Every categorical fill must pass
  the OKLCH lightness band, chroma floor, CVD adjacent-pair separation, and
  contrast checks against the widget surface in dark mode. Every text color
  must clear 4.5:1 against the same surface. Validated values:

  | Role | Hex | Check |
  | --- | --- | --- |
  | Surface | `#0b0c10` | reference surface |
  | Crates fill | `#d4794a` | L 0.63, 6.17:1 |
  | Documentation fill | `#6a93e0` | L 0.63, 6.39:1 |
  | Counter fill | `#38a86f` | L 0.62, 6.51:1 |
  | Primary ink | `#e6e8ee` | 15.96:1 |
  | Secondary ink | `#9aa1b2` | 7.55:1 |
  | Muted ink | `#8b93a7` | 6.36:1 |
  | Accent / clock | `#f0d9a8` | 14.14:1 |
  | Additions | `#7ee2ad` | 12.44:1 |
  | Deletions | `#f4816a` | 7.63:1 |

  The three fills pass all checks together as a categorical set (worst adjacent
  pair ΔE 17.1 deutan, 20.4 normal vision). Tritan separation between the
  documentation and counter fills is below the ΔE 8 target; this is admissible
  because every lane and counter is permanently direct-labeled, which is the
  documented secondary-encoding condition. Any change to these values must be
  re-validated before it ships.
- Responsive: the ticker and the group columns stack on narrow viewports; no
  horizontal page scroll at any width.
- Accessibility: the section carries `role="img"` and an `aria-label`
  summarizing the snapshot in one sentence. Rapidly churning numeric nodes are
  `aria-hidden` so assistive technology is not flooded. The replay control is a
  real `<button>` with an accessible name.

## 9. Generator Interface

```
code-viz/commit-timeline/gen_timeline.py
    [--repo PATH]           # default: repository root, resolved from __file__
    [--config PATH]         # default: config/openwepp.json beside the script
    [--rev REV]             # default: HEAD
    [--out-json PATH]       # default: data/commit-timeline.json
    [--out-html PATH]       # default: dist/commit-timeline.html
    [--out-fragment PATH]   # default: dist/commit-timeline.fragment.html
    [--template PATH]       # default: template.html beside the script
    [--generated-at ISO]    # override meta.generatedAt for reproducible output
    [--check]               # verify committed outputs are current; write nothing
    [--quiet]
```

Conventions, matching every script under `tools/`: `#!/usr/bin/env python3`,
a one-line module docstring, `argparse`, Python 3 standard library only. The
repository has no dependency manager and none is introduced. Per root
`AGENTS.md`, repo-local Python tooling is invoked as `.venv/bin/python`; the
script must also run under a bare `python3` since it imports nothing external.

Every non-quiet run prints a sanity table: commit count, date range, global
additions and deletions, each counter's snapshot value, and each lane's
snapshot value grouped by column. Omitted zero-value lanes are listed
separately.

### 9.1 Rendering

`template.html` contains the markup, the `<style>` block, and the player
script, with placeholders substituted by the generator:

| Placeholder | Substituted with |
| --- | --- |
| `{{DATA_JSON}}` | The data file contents, escaped for a single-quoted JS string literal |
| `{{TITLE}}` | `meta.title` |
| `{{SECTION_ID}}` | Stable widget element id |
| `{{ARIA_LABEL}}` | Generated one-sentence summary |

The escape must be safe in both contexts simultaneously: JS string escaping for
backslash and single quote, plus `<` escaped so the payload cannot terminate
the enclosing `<script>` element. Data is embedded as a `JSON.parse('…')` call
rather than an object literal, which parses faster and keeps escaping rules
uniform.

The fragment output is the `<section>` element with its `<style>` and
`<script>`, and nothing else. The standalone output wraps the same fragment in
a minimal HTML document.

## 10. Determinism and Drift

- Given the same repository state, config, and `--generated-at`, the generator
  must produce byte-identical JSON. No wall-clock reads, no dictionary
  iteration order dependence, no unstable sorts. Ties in lane ordering break on
  lane key.
- `--check` regenerates in memory and compares against the committed outputs,
  ignoring `meta.generatedAt`, and exits non-zero on any difference without
  writing. This mirrors the `tools/release/check_*.sh` pattern already used in
  the repository.
- `data/` and `dist/` are generated artifacts. They are committed so the
  visualization is viewable directly from a checkout, and they must never be
  hand-edited.
- Because the snapshot goes stale as the repository advances, the README must
  state the regeneration command and the expectation that outputs are
  refreshed alongside significant pushes.

## 11. Verification

A change to this component is not complete until all of the following hold.

1. `gen_timeline.py` runs clean and its sanity table matches the repository:
   commit count from `git rev-list --count --no-merges <rev>`, and date range
   from the first and last non-merge commits.
2. Lane values reconcile against the working tree. For each lane, the
   git-derived net must agree with `git ls-files <paths> | xargs wc -l` within
   1%. Exact agreement is expected for most lanes; small gaps are legitimate,
   arising from files without a trailing newline and from binary entries that
   numstat reports as `-`. Any lane exceeding 1% must be investigated, not
   waived.
3. The standalone output opens from `file://` and plays: autoplay on first
   scroll into view, every lane reaching 100%, counters landing on exact
   snapshot values, and the clock ending at the snapshot commit's local time.
4. Replay works both mid-sweep and after completion, rewinding cleanly with no
   residual state and no concurrent sweeps.
5. Under emulated `prefers-reduced-motion: reduce`, the widget does not
   autoplay, and activating replay lands on the snapshot state immediately.
6. The browser console reports no errors and the network panel records no
   requests.
7. Auto-discovery is exercised: adding a directory under the discovery path in
   a scratch clone and re-running produces a new lane with no code or config
   edit.
8. `--check` exits zero against the committed outputs on a clean tree.
9. The rendered output contains no completion framing, per the
   "Work-in-Progress Framing" section.

## Appendix A — Snapshot at `497d76d0`

Recorded 2026-07-24 for review of this specification. These figures are
illustrative of scale and ordering only; the generator is authority, and this
appendix is not a fixture. Net values are cumulative additions minus deletions
over non-merge history.

Globals: 2,077 commits, 2026-05-11 → 2026-07-24, `+21,935,670 / −346,315`
lines, 959 work packages.

Crates group — 22 lanes, totalling 284,937. The group carries the workspace
crates, the integration-test tree, and a residual lane for files loose under
`crates/`. Its label stays `crates` while `tests/integration` sits in it; the
lane's own label says exactly what it is, so no reader is misled about the
contents:

| Lane | Net | Lane | Net |
| --- | ---: | --- | ---: |
| hillslope-orchestrator | 70,313 | management-schema | 1,750 |
| tests/integration | 48,799 | plant-phenology | 1,534 |
| runner | 44,401 | unit-boundary | 1,378 |
| gate-planner | 28,225 | legacy-bridge | 1,190 |
| input-contract | 21,989 | topology | 1,155 |
| watershed-orchestrator | 20,149 | climate-runtime-adapter | 1,151 |
| assurance | 18,223 | comparator-metadata | 337 |
| sim-contract | 6,250 | crates (shared) | 71 |
| kernel-contract | 5,292 | | |
| landuse-migrate | 3,587 | | |
| watershed-output | 3,084 | | |
| meteorology | 2,303 | | |
| hillslope-output | 1,977 | | |
| summary-accumulator | 1,779 | | |

Documentation group — 14 lanes, totalling 64,794:

| Lane | Net | `wc -l` | Delta |
| --- | ---: | ---: | ---: |
| specifications | 36,010 | 36,010 | 0 |
| planning | 4,964 | 4,964 | 0 |
| standards | 4,165 | 4,165 | 0 |
| architecture | 3,989 | 3,999 | 10 |
| decisions | 3,925 | 3,925 | 0 |
| backlog | 3,161 | 3,161 | 0 |
| governance | 1,735 | 1,735 | 0 |
| usersum | 1,531 | 1,531 | 0 |
| docs (loose files) | 1,425 | — | — |
| audits | 1,280 | 1,280 | 0 |
| contracts | 1,186 | 1,186 | 0 |
| dev-guide | 1,094 | 1,094 | 0 |
| prompt_templates | 246 | 246 | 0 |
| numerics | 83 | 83 | 0 |

The 21 workspace-crate lanes sum to 236,138, matching
`git ls-files crates | xargs wc -l` exactly, and `tests/integration` matches
its own `wc -l` of 48,799 exactly. Thirteen of fourteen documentation lanes
match `wc -l` exactly. The
single exception is `architecture` at 0.25%, ten lines, from the generated
`.dot`/`.json` exports under `docs/architecture/generated/`; it is within the
tolerance in §11.2.
