# Source Provenance

Evidence class: `Executed source-preservation record`

## Commissioned archive

The three Elliot files were copied from the WEPPcloud work-package reference
archive. Independent SHA-256 calculations matched its ledger before and after
copying. The management files retain CRLF line endings and the report retains
its original PDF bytes. The project paid for this work and has permission to
redistribute it.

## Operator-supplied literature

The operator identified seven PDFs and one Markdown file in `~/Downloads`.
Each regular file was hashed and media-type checked, then moved under
`references/canopy_phenology/literature/` with a normalized bibliographic
filename. `.DS_Store` and the `wepp-figures/` image directory were excluded:
neither was requested and neither carries load-bearing canopy evidence.

The file originally named `59583.pdf` is Amatya and Trettin (2019), not the
2022 Amatya paper. Both are preserved under title-derived names. The 1987 WEPP
requirements PDF is an image-oriented scan; the operator-supplied Markdown is
retained as a searchable transcription, not represented as source-native text.

## Acquired citations

Two report citations absent from the supplied set were acquired from official
USDA Forest Service Treesearch records:

- Dun et al. (2009), Treesearch 34121,
  `https://research.fs.usda.gov/download/treesearch/34121.pdf`;
- Srivastava et al. (2020), Treesearch 63145,
  `https://research.fs.usda.gov/download/treesearch/63145.pdf`.

The downloaded objects identify the expected titles, authors, journals, page
ranges, and DOIs. Neither response was HTML or an error object. The report
omits the second dot in Dun’s DOI; the retained paper gives
`10.1016/j.jhydrol.2008.12.019`.

The Flanagan and Livingston (1995) WEPP User Summary was not duplicated. The
openWEPP repository already preserves the chapter set under
`references/50201000/`, and the current consolidated user summary under
`references/vendorable/usersum2024.pdf`.

## Hubbard Brook synthesis book

The operator’s clone was clean at
`3bb5b43e1429172b8d002e4b002d6a31db694ad1`, with origin
`https://github.com/hbr-lter/synthesisbook/`. The source-native forest
management, biomass/productivity, and physiology/phenology chapters,
bibliography, upstream README, and an exact identity record were copied.

No explicit `LICENSE` file was present at the admitted commit. This package
therefore records source identity but makes no license claim.

## Publication boundary

Local preservation and scientific admissibility do not by themselves imply
public redistribution permission. Bill's commissioned files are cleared by the
operator's rights statement. The independent review initially held the
third-party literature and Hubbard Brook snapshot; on 2026-07-26 the operator
explicitly confirmed redistribution permission for that retained set and
directed commit/push. The superseding authority is detailed in
`publication-rights-register.md`.

## Visual and structural inspection

The 25-page commissioned report was rendered and inspected at the pages carrying
site targets, iterative parameter changes, equilibrium charts, hydrology and
sediment comparisons, return-period tables, conclusions, and references. The
source PDFs supporting biomass, forest floor, litterfall, fuel load, and
watershed runoff were text-extracted and their target-bearing pages rendered.
The acquired Dun and Srivastava papers were metadata-checked and their first
pages rendered. PDF metadata reports no embedded JavaScript in any target-bearing
source inspected.

Temporary renders and extracted text were not committed.
