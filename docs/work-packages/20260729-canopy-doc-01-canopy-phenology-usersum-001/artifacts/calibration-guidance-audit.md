# Calibration Guidance Audit

Status: `pass`

Evidence mode: `Static`

| Requirement | Status | Evidence in narrative |
| --- | --- | --- |
| Composition and persistent structure first | PASS | Calibration begins with functional composition, winter foliage, and woody cover observations. |
| Full-leaf biomass, LAI, cover, and height second | PASS | These are constrained at matching horizontal-area and plot/OFE scales before timing. |
| Seasonal thresholds fitted jointly | PASS | The six thresholds are explicitly correlated; multiple years and all acceptable combinations are retained. |
| Litter source separated from decay | PASS | Tissue-separated repeated input and stock observations follow canopy calibration; a terminal stock is called nonidentifying. |
| Independent transfer without refit | PASS | Sites or years are reserved, and Harvard/Bezà results are not used to reopen fitting. |
| Units and observation operators | PASS | Every coefficient table row gives units; dry-mass, horizontal-area, exact-daily, LAI, cover, and height scales are distinguished. |
| Hard domains separated from empirical ranges | PASS | Range vocabulary is defined; only the Hubbard timing ensemble and Hubbard mature-LAI source interval receive numeric scoped ranges. |
| Search/example values not promoted | PASS | `bb`, `bbb`, `hmax`, foliar/structural partition, evergreen fraction, and both decay constants remain `NOT_ESTABLISHED`. |
| Identifiability and equifinality | PASS | GSI covariance, biomass partition, cover closure, and source/rate ridge are explicit. |
| Immediate residue controls | PASS | `cf` is calibrated only to paired residue mass/cover; current native `diam` branch inactivity and the exact initial cover/mass-to-derived-depth lineage are explicit. |
| No downstream compensation | PASS | The narrative prohibits tuning canopy parameters to hide snow, frost, runoff, erosion, sediment, litter-source, or decomposition residuals. |
| Application boundary | PASS | Northern temperate evidence is separated from unsupported Harvard transfer and the tropical dry-forest ecosystem-model limitation. |

The guide recommends no universal default or typical starting vector. Users
must supply explicit native inputs supported by observations at their
application scale.
