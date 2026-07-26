# CLAUDE.md — code-viz

Guidance for agents authoring or revising visualizations under `code-viz/`.
Read alongside root `AGENTS.md`. Nothing here is on a gate, comparator, or
science-contract path.

## Specification first

Each visualization directory owns a `specification.md` that is authority for
its generator, data schema, and player. Revise the specification before the
code, not after. A behavior that is not in the specification is not a feature —
it is drift.

## Hard constraints

- **No network at view time.** The rendered page must issue zero requests. No
  CDN, no external font, no remote image, no fetch. Data is inlined.
- **No frameworks.** No React/Vue/Svelte, no charting library, no Tailwind or
  any CSS framework. Plain HTML, one scoped `<style>` block, vanilla JS with
  `requestAnimationFrame` and `IntersectionObserver`.
- **Python standard library only.** Generators are `#!/usr/bin/env python3`
  with a one-line docstring and `argparse`, matching every script under
  `tools/`. The repository has no dependency manager; do not introduce one.
  Per root `AGENTS.md`, invoke repo-local tooling as `.venv/bin/python`;
  generators must also run under a bare `python3`.
- **Scoped CSS.** Every selector is prefixed by the widget's element `id`, so a
  fragment pasted into an arbitrary host page cannot collide with it.
- **Determinism.** Same repository state and same inputs must produce
  byte-identical output. No wall-clock reads outside an explicit
  `--generated-at`, no unstable sorts, no iteration-order dependence.

## Generated artifacts

`data/` and `dist/` are outputs. Never hand-edit them. Change the generator,
the template, or the config, re-run the generator, and commit the regenerated
files in the same change. Each generator supports `--check`, which fails when
the committed outputs no longer match the repository.

## Honesty rules

These artifacts are read as claims about the project, so they are held to the
same standard as any other claim here.

- openWEPP is a **work in progress**. No output may frame it as finished,
  shipped, or complete, and no bar or counter may read as progress toward a
  finish line.
- Every figure is a snapshot at a named commit, and the rendered output must
  display that commit and its date.
- When a visual encoding can mislead — a clamped bar, a normalized axis, an
  excluded path — say so in the caption, not only in the specification.
- Statistics belong in generated output. Do not hand-copy numbers into prose
  except in an appendix that is explicitly dated and marked as illustrative.

## Color

Color encodes a category, never a rank: a lane must not change color because
another lane overtook it. Palettes are validated against the widget's own dark
surface for lightness band, chroma, colorblind separation, and contrast before
they ship — not chosen by eye. Recorded values live in the visualization's
specification.
