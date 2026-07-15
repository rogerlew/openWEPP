# Scientific Assurance Sources

This directory is the canonical source for public scientific assurance pages.
The ownership, lifecycle, and command contract is
`docs/governance/scientific-assurance-dossier-lifecycle.md`.

- `catalog.yaml` declares stable dossiers, sources, and generated outputs.
- `schemas/` contains compiler-bound strict version-1 source contracts.
- `templates/` controls generated Markdown shape.
- `methods/` owns evaluation design (how).
- `dossiers/` owns evidence characterization (what), limitations,
  agent-assisted authoring records, and structured review histories.
- `generated/wepppy-usersum.yaml` is a generated downstream handoff.

Do not edit `assurance/generated/` or `usersum/assurance/` by hand. Run:

```bash
cargo run -p openwepp-assurance -- build --all
cargo run -p openwepp-assurance -- check --all
```

Normal builds are deterministic, local, offline, and agent-free. Scientific
review and application decisions remain human-owned activities outside the
build.
