# D2 Source Provenance Manifest

The D2 case fixtures derive from the Papanicolaou 2018 supplemental, which
lives in `references/copyrighted/` — a **gitignored local-cache** directory
(reference-vendoring policy: copyrighted material is not committed). The
fixtures are therefore not byte-reproducible from Git alone; this manifest
records the source files and their sha256 so the derivation is verifiable
against the operator's local cache.

Bibliography: **R-63** (Papanicolaou et al. 2018), local path
`references/copyrighted/Papanicolaou2018-supplemental/`.

| Source file (under `references/copyrighted/Papanicolaou2018-supplemental/wrcr23071-sup-0002-2017wr021109-ds01/`) | sha256 | Used for |
|---|---|---|
| `3.1_Validation_Input.docx` | `0aee14555a3f5394aef89c9b6623fc13644273a676bb316e76ca5b6e148f9362` | authoritative case inputs (all 4 cases) |
| `Figure_4.xlsx` | `2bf68787de6a715049ee635c154c640214936fd1181d08c8f7da7a34892d2fe8` | observed Enhanced_WEPP/Original_WEPP hydrograph series (referenced, not duplicated) |

Verify: `sha256sum <file>` against the values above. D-val (later stage)
that consumes the observed series must cite this manifest and the R-63 local
path; it must not vendor the copyrighted series into the repo.
