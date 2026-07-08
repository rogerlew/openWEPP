# Required Reading Map

Status: scaffolded.
Package: `20260708-groundwater-baseflow-srivastava-authority-001`

## Budget

Static byte count for the core authority set measured on 2026-07-08:
`8,920,167` bytes. Disposition: `REQUIRES-JUSTIFICATION`.

Justification: this is a contract-first authority package. The source set
includes primary/companion PDFs plus baseline Fortran authority required to bind
WEPP groundwater/baseflow equations and runtime surfaces. Execution should use
targeted `pdftotext` extraction for the groundwater/baseflow sections rather
than loading full copyrighted PDFs into artifacts.

## Core Governance

| Path | Bytes | Purpose |
|---|---:|---|
| `AGENTS.md` | 10,269 | root governance |
| `docs/work-packages/AGENTS.md` | 16,364 | package governance |
| `docs/specifications/science-contracts/AGENTS.md` | 5,599 | contract governance |
| `docs/standards/AGENTS.md` | 3,328 | standards governance |
| `docs/standards/prompt-wording-guidance.md` | 9,780 | kickoff prompt wording |

## Local Authority Docs

| Path | Bytes | Purpose |
|---|---:|---|
| `docs/ROADMAP.md` | 84,905 | M-T2A queue authority |
| `references/annotated_bibliography.md` | 125,785 | R-21/R-22/R-22A/R-70 entries |
| `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md` | 15,170 | parser-to-process linkage |
| `docs/specifications/wepp-input-files/specs/gwcoeff.spec.md` | 14,796 | input file schema/provenance |

## Literature Authorities

| Path | Bytes | Role |
|---|---:|---|
| `/workdir/wepp-forest/references/Srivastava_Diss2013_14.pdf` | 4,969,996 | primary dissertation authority |
| `references/copyrighted/Srivastava2013.pdf` | 779,756 | peer-reviewed linear-reservoir companion paper |
| `references/copyrighted/Srivastava2017_ToASABE_wepp_streamflow.pdf` | 2,016,841 | later nonlinear/baseflow extension lineage |
| `references/copyrighted/dun2009.pdf` | 689,246 | forest subsurface/deep-percolation context |

## Baseline Code Authority

| Path | Bytes | Role |
|---|---:|---|
| `/workdir/wepp-forest_260430_baseline/src/main.for` | 16,781 | `gwcoeff.txt` parse and `lr_bf` branch |
| `/workdir/wepp-forest_260430_baseline/src/contin.for` | 58,513 | groundwater storage/baseflow/deep seepage update |
| `/workdir/wepp-forest_260430_baseline/src/wshpas.for` | 20,650 | hillslope pass baseflow/deep seepage payloads |
| `/workdir/wepp-forest_260430_baseline/src/wshdrv.for` | 43,779 | watershed driver pass consumption |
| `/workdir/wepp-forest_260430_baseline/src/wshchr.for` | 23,349 | channel baseflow consumption |
| `/workdir/wepp-forest_260430_baseline/src/wshcqi.for` | 10,551 | channel inflow/baseflow/phosphorus coupling |
| `/workdir/wepp-forest_260430_baseline/src/watbalprint.for` | 4,709 | water-balance baseflow publication behavior |

## Conditional Reads

Read only when the execution touches the named authority boundary:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
