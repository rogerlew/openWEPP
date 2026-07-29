# CAL-06 Figures

Evidence class: `Ran`

All SVGs are deterministic, accessible, plot-only vector artifacts rendered by
`../../tools/plot_results.py`. Each SVG has a same-basename Markdown sidecar
containing its caption, source links, units, interpretation, and scientific
boundaries.

| Plot | Sidecar | Plotted source |
| --- | --- | --- |
| `cal06-canopy-chronology.svg` | `cal06-canopy-chronology.md` | `../daily-climatology.csv` |
| `cal06-seasonal-ordering-amplitude.svg` | `cal06-seasonal-ordering-amplitude.md` | `../ensemble-summary.csv` |
| `cal06-snow-response.svg` | `cal06-snow-response.md` | `../daily-climatology.csv` |
| `cal06-litter-residue-frost.svg` | `cal06-litter-residue-frost.md` | `../daily-climatology.csv` |
| `cal06-downstream-consequences.svg` | `cal06-downstream-consequences.md` | `../daily-climatology.csv` |
| `cal06-congruence-verdict-matrix.svg` | `cal06-congruence-verdict-matrix.md` | `../verdict-matrix.csv` |

Every SVG embeds the exact SHA-256 identity of each source table listed above;
the terminal validator rejects stale or incomplete bindings. Sidecars carry
the ancillary scientific information previously displayed inside the SVGs.
Machine-readable tables and terminal validation carry the quantitative result.
