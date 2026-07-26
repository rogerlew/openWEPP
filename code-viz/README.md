# code-viz

Static visualizations of the openWEPP repository itself — how it is built, not
what it computes. Nothing here is part of the simulation engine, and nothing
here is on any gate path.

## What "static" means here

Every visualization in this directory is a generator plus a pre-rendered
output. A Python script reads the repository once and writes a data file and an
HTML page with that data inlined. The rendered page:

- performs **zero network requests** at view time,
- loads **zero libraries** — no framework, no CDN, no external font,
- runs from `file://`, from a web server, or pasted into another page.

There is no build system, no package manager, and no node toolchain. The
repository has none, and none is introduced.

## Visualizations

| Directory | What it shows |
| --- | --- |
| [`commit-timeline/`](commit-timeline/) | Animated replay of the commit history — clock, `git log` ticker, one bar per crate and per documentation category, work-package counter |

## Layout convention

Each visualization is one self-contained directory:

```
code-viz/<name>/
├── specification.md   authority for the generator, data schema, and player
├── README.md          what it shows, how to regenerate, how to embed
├── CLAUDE.md          notes for agents working in this directory
├── gen_*.py           the generator
├── template.html      markup + scoped CSS + player, with {{PLACEHOLDERS}}
├── config/            repository-specific inputs (paths, labels, categories)
├── data/              generated — the interchange file
└── dist/              generated — the rendered page and embeddable fragment
```

`data/` and `dist/` are generated artifacts. They are committed so a checkout
can be viewed without running anything, and they are never hand-edited.

## Working here

Specification first. `specification.md` is authority; the generator, the
schema, and the player follow it. When behavior needs to change, revise the
specification, then the code.

These artifacts are snapshots of a repository that is still moving. Nothing
produced here may frame openWEPP as finished, and every figure carries the
commit it was measured at. See each visualization's `CLAUDE.md`.
