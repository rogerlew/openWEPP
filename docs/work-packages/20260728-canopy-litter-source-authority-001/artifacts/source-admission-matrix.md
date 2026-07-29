# Source Admission Matrix

Status: `prospective review candidate`

Evidence class: `Static`

Admission vocabulary:

- `ADMIT`: may govern the stated claim inside its stated envelope;
- `ADMIT_CONDITIONAL`: may govern only when its named operands and
  provenance are supplied;
- `DIAGNOSTIC_ONLY`: may challenge or contextualize a result but may not
  determine a production operand;
- `SUPPORTING_ONLY`: relevant bibliographic context, but no binding production
  claim depends on the unavailable or unauthenticated passage;
- `REJECT_FOR_LAW`: authentic source, but it does not authorize the proposed
  executable law; and
- `ARTICLE_REQUIRED`: a material claim cannot be adjudicated without an
  unavailable primary source.

No candidate remains `ARTICLE_REQUIRED`. The Lim et al. supplement was
obtained from the authors' SafeDeposit record, so no operator acquisition is
currently needed.

## Admission decisions

| Source | Decision | Admitted claim and material | Units/equation | Applicability | Binding limitation |
| --- | --- | --- | --- | --- | --- |
| `SC-PLANT-001`, CP-GSI02 | `ADMIT` | Existing modeled live deciduous foliar-pool decrease transfers once as same-day leaf litter. | `kg dry mass m-2 d-1`; existing exact daily pool ledger | Native GSI path already in production | Does not generate recurring evergreen needle litter or fine wood. It is not observational authority for a live-to-litter conversion. |
| `SC-RESIDUE-001` | `ADMIT` | A dry-mass surface source is added before decomposition, cover, depth, frost, and erosion consumption. | `kg dry mass m-2 d-1` at the daily boundary | All accepted tissue-specific surface inputs | Consumer/accounting authority only; it supplies neither source magnitude nor timing. |
| Pinned WEPP baseline `dac3c950...` and Chapters 8/9 | `ADMIT` for topology; `REJECT_FOR_LAW` for native generation | Generic residue addition/decomposition establishes the legacy material destination and ordering. | Legacy residue mass/rate operands | Static provenance and comparator interpretation | No general native-forest needle or fine-branch production law was found. Absence is not authority for zero. |
| White et al. (2000), DOI `10.1175/1087-3562(2000)004<0003:PASAOT>2.0.CO;2` | `DIAGNOSTIC_ONLY` | Evergreen leaf turnover frequency is commonly related to inverse leaf longevity. | No production equation or value admitted | Gross-turnover context | The access-controlled object is not an immutable local authority object, and no production path needs it. |
| Kloeppel, Harmon, and Fahey (2007), DOI `10.1093/acprof:oso/9780195168662.003.0005` | `ADMIT_CONDITIONAL` | At mature steady state, annual evergreen foliage production can be estimated from standing foliage biomass divided by median leaf longevity; long-term foliage litterfall is approximately production. | `kg dry mass m-2 yr-1` | Long-term gross foliar turnover/production estimate with measured standing foliage and longevity | Annual equivalence is approximate, not a daily deposition law. Live-foliage allometry can overstate litter dry mass because resorption changes mass before abscission. |
| CLM5 technical note, §11.3.5.4 | `DIAGNOSTIC_ONLY` | Documents one continuous evergreen background-litterfall implementation. | `r_bglf = 1 / (tau_leaf * 365 * 86400)` in `s-1` | Cross-model design comparison | Secondary model documentation, not independent primary authority. It explicitly identifies observed seasonal highs/lows and continuous timing as an improvement area; therefore it cannot authorize uniform daily timing here. |
| Bernier, Hanson, and Curtis (2008), DOI `10.1007/978-1-4020-8506-2_7` | `SUPPORTING_ONLY` | Bibliographically supports the measurement domain. | No production equation or value admitted | Context only | No authenticated downloadable object was retained; no binding claim depends on it. |
| Keane journal article (2008), DOI `10.1139/X08-003`, and RMRS-RP-70, DOI `10.2737/RMRS-RP-70` | `ADMIT` | Long-term collectors can separate deposited material into foliage, twigs, branches, and other classes and report oven-dry interval mass per horizontal area. | `kg dry mass m-2 observation-interval-1` | Authenticated measured boundary quantity at its actual interval and material definition | Monthly/semiannual visits do not establish exact deposition dates. Site rates are not defaults; source diameter classes must be preserved rather than relabeled. |
| Lim et al. (2024), DOI `10.1073/pnas.2401035121`, main text and Supplement Table S4/Fig. S9 | `ADMIT` for distinctions and required state; `REJECT_FOR_LAW` for current predictive publication | Branch turnover, attached-dead-branch storage, in-canopy mass loss, and branch litterfall are different quantities. Predictive turnover depends on crown/stand state. | Turnover `kg tree-1 yr-1` or fraction `yr-1`; models use annual height increment, stand or relative stand density, shade-tolerance class, live-crown ratio, and branch biomass | Evaluating whether current state can predict a fine-woody surface source | Current openWEPP carries aggregate structural biomass, not the required branch/crown/stand operands. Supplement Fig. S9 estimates substantial in-canopy loss, so turnover cannot be published as same-day surface deposition. Broad cross-site rates are not defaults. |
| Hubbard Brook `HBEF_fine_litter_1992-2024.csv` plus EML | `DIAGNOSTIC_ONLY` | Confirms collector area, total oven-dry sample mass, collection dates, and some species/tissue sorting context. | grams per `0.097 m2` trap/collection | Data-availability and future acquisition audit | Retained table does not consistently expose separated needle or twig dry mass. Total or broadleaf fields cannot be repurposed. |
| Harvard pooled foliar/nonfoliar litter-carbon series | `DIAGNOSTIC_ONLY` | Can challenge seasonal or total behavior at a pooled carbon level. | carbon mass flux | Retrospective diagnostics only | Nonfoliar is not isolated fine wood; carbon is not dry mass; no conversion or fit is authorized. |

## Adjudicated answers

1. Evergreen systems physically have recurring foliar turnover, but a
   production source is required only when the modeled boundary claims to
   represent that material. Missing authority must be exposed as
   source-incomplete, not encoded as zero.
2. Live evergreen foliar dry mass and leaf longevity can determine a gross
   annual turnover estimate. They cannot alone determine deposited needle dry
   mass or its daily timing.
3. Woody systems have branch turnover, but actual surface branchfall can be
   intermittent and differs from turnover. Current aggregate structural
   biomass is insufficient for a predictive fine-woody source.
4. A tissue-separated, dry-mass ground-boundary quantity is scientifically
   valid at its observed temporal support. It becomes executable daily input
   only when measurement truly resolves exhaustive daily deposition or when
   an operator supplies an explicitly non-observational prescribed scenario.
5. The accepted boundary flux enters the existing surface-residue source
   topology once. It is not also removed from an unmodeled canopy pool and is
   not duplicated into downstream residue pools.

## Rejected shortcuts

- `evergreen_foliar_stock / 3.8 yr` as deposited needle dry mass;
- a universal continuous or uniform-daily timing law;
- a `20–30%` universal resorption correction;
- a Keane site mean as a generic native-forest rate;
- `structural_biomass * broad_branch_turnover_rate`;
- branch turnover or mortality as immediate ground deposition;
- pooled nonfoliar carbon converted to fine-wood dry mass; and
- omitted forcing interpreted as an observed zero.

Exact objects, hashes, and page/table/line anchors are recorded in
`authority-source-ledger.md`.

## Acquisition disposition

`ARTICLE_REQUIRED = 0`. The current decision does not depend on an
inaccessible article. Future predictive needle deposition would require a
species/site-specific live-to-litter retention and timing authority. Future
predictive fine-wood deposition would require branch-pool/crown/stand state
authority plus an attached-dead/in-canopy/deposition model. Those are
successor research needs, not missing PDFs for this decision.
