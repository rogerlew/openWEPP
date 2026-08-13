# V3 Reference Acquisition Ledger

Status: `complete for selected authority`

Evidence mode: `Static`

| Reference | Selected use | Classification |
| --- | --- | --- |
| CLM5 Surface Albedos 2.3.1 | `L+S` two-stream coordinate, area-weighted leaf/stem optics, band/direction/sun/shade transport | `REFERENCE_MODEL_DEFINITION` |
| CLM5 Fluxes 2.5.117, 2.5.122 | incident leaf wind equals friction velocity; characteristic-dimension boundary transfer | `REFERENCE_MODEL_DEFINITION` |
| CLM5 Plant Hydraulics 2.11.2 | common root node, height/gravity hydraulic circuit, maximum-demand/vulnerability continuity | `REFERENCE_MODEL_DEFINITION` |
| CLM5 Photosynthesis 2.9.10--17 and reviewed Chapters 17/19 | Rd-specific response, net assimilation, Atkin leaf-N/acclimation respiration ownership | `REFERENCE_MODEL_DEFINITION` |
| ESCOMP/CTSM commit `8e1309ab0db671d884b80746cbae9bbaafbe78a7`, `src/biogeophys/PhotosynthesisMod.F90`, SHA-256 `e4c9ad718209af44fcfdfc1d591bd2729d345f9e422cf5d9c8a889525d6a1cdf`, lines 1318--1322 and 1441--1447 | Direct immutable transcription of leaf-N units, Atkin coefficients, Celsius T10 operand, positive-N branch, and `lmr25top` output basis | `REFERENCE_MODEL_DEFINITION` |
| Physical radiation/carbon/transaction closure | physical absorption ownership and exact-once debit | `INFERENCE` |

The V3 absorption partition, exact `K_eff` placement, bitwise-only migration,
accepted owner-uncapped coupling, and diagnostic DTO are explicitly labeled
`OPENWEPP_CANONICAL_SELECTION`; they are not misrepresented as verbatim CLM.

Acquisition evidence: `git ls-remote` resolved release-clm5.0.32 to the pinned
commit above; downloading the raw source path reproduced the recorded SHA-256.
