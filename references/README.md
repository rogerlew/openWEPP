# References Corpus

This directory holds scientific and engineering references used during
openWEPP architecture and kernel work.

## Policy summary

- Prefer vendoring source material into this repository **when redistribution
  is allowed** by license/copyright terms.
- Keep restricted/copyrighted source files in a local cache that is not
  committed.
- Maintain [annotated_bibliography.md](annotated_bibliography.md) as the
  canonical index of references, quality, and usage mapping.

## Layout

- `annotated_bibliography.md` (tracked): reference index and annotations.
- `vendorable/` (tracked): files cleared for redistribution.
- `copyrighted/` (gitignored): local-only cache for restricted files.

## Intake workflow

1. Add citation + quality + kernel mapping in `annotated_bibliography.md`.
2. Classify rights:
   - `redistributable`: place artifact in `vendorable/`.
   - `restricted/unknown`: place artifact in `copyrighted/`.
3. For restricted files, keep only metadata/annotations tracked in git.

## Sync from wepp-palimpsest / wepp-forest

Example local sync command:

```bash
rsync -a /workdir/wepp-forest/references/ ./references/copyrighted/
cp /workdir/wepp-forest/references/annotated_bibliography.md ./references/annotated_bibliography.md
```

If bibliography paths are repo-specific, rewrite local paths for this repo
before committing.
