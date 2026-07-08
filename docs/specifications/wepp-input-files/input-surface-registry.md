# Parser Input Surface Registry

Status: Active
Last updated: 2026-05-21

Purpose: canonical registry of parser-governed input surfaces across hillslope,
watershed, and sidecar domains.

Disposition meanings:
- `active`: governed by an authored `SC-INFILE-*` contract
- `planned`: contract ID assigned; authoring pending
- `deferred`: intentionally postponed with rationale
- `unsupported`: explicitly rejected with typed error behavior

| Surface ID | Category | File surface | Disposition | Target contract |
|---|---|---|---|---|
| `infile-climate-cli` | hillslope | `.cli` | active | `SC-INFILE-CLIMATE-001` |
| `infile-soil-sol` | hillslope | `.sol` | active | `SC-INFILE-SOIL-001` |
| `infile-management-man` | hillslope | `.man` | active | `SC-INFILE-MANAGEMENT-001` |
| `infile-management-yaml` | hillslope | openWEPP management `.yaml` | planned | `SC-INFILE-MANAGEMENT-YAML-001` |
| `infile-slope-slp` | hillslope | `.slp` | active | `SC-INFILE-SLOPE-001` |
| `infile-watershed-structure-str` | watershed | `.str` | active | `SC-INFILE-WATERSHED-STRUCTURE-001` |
| `infile-watershed-channel-chn` | watershed | `.chn` | active | `SC-INFILE-WATERSHED-CHANNEL-001` |
| `infile-watershed-impoundment-imp` | watershed | `.imp` | active | `SC-INFILE-WATERSHED-IMPOUNDMENT-001` |
| `infile-hillslope-binary-pass-hbp` | interchange | `H<hillslope_id>.hbp` | active | `SC-INFILE-HBP-001` |
| `infile-irrigation-depletion` | sidecar | legacy unit `15` depletion irrigation file | active | `SC-INFILE-IRRIGATION-DEPLETION-001` |
| `infile-irrigation-fixeddate` | sidecar | legacy unit `14` fixed-date irrigation file | active | `SC-INFILE-IRRIGATION-FIXEDDATE-001` |
| `infile-pmetpara` | sidecar | `pmetpara.txt` | active | `SC-INFILE-PMETPARA-001` |
| `infile-snow` | sidecar | `snow.txt` | active | `SC-INFILE-SNOW-001` |
| `infile-frost` | sidecar | `frost.txt` | active | `SC-INFILE-FROST-001` |
| `infile-gwcoeff` | sidecar | `gwcoeff.txt` | active | `SC-INFILE-GWCOEFF-001` |
| `infile-phosphorus` | sidecar | `phosphorus.txt` | active | `SC-INFILE-PHOSPHORUS-001` |
| `infile-wepp-ui` | sidecar | `wepp_ui.txt` | active | `SC-INFILE-WEPPUI-001` |
| `infile-channel-tc` | sidecar | `tc.txt` | active | `SC-INFILE-TC-001` |
| `infile-channel-tcr` | sidecar | `tcr.txt` | active | `SC-INFILE-TCR-001` |
| `infile-channel-lcwb` | sidecar | `lcwb.txt` | active | `SC-INFILE-LCWB-001` |
| `infile-channel-contrast` | sidecar | `chan.inp` | active | `SC-INFILE-CHANINP-001` |
| `infile-observe-on` | sidecar | `wepp_observe.on` | unsupported | `N/A (moved to first-class observability subsystem)` |
| `infile-observe-frost-on` | sidecar | `wepp_observe_frost.on` | unsupported | `N/A (moved to first-class observability subsystem)` |
| `infile-observe-probe-target` | sidecar | `wepp_observe_wb05e_target.dat` | unsupported | `N/A (moved to first-class observability subsystem)` |
| `infile-firedate` | sidecar | `firedate.txt` | unsupported | `N/A (wepp-forest-revegetation out-of-scope)` |
| `infile-cancov` | sidecar | `cancov.txt` | unsupported | `N/A (wepp-forest-revegetation out-of-scope)` |
| `infile-simfire` | sidecar | `simfire.txt` | unsupported | `N/A (wepp-forest-revegetation out-of-scope)` |

Notes:
- This registry is the required source for completeness checks in parser
  contract governance.
- Any newly discovered sidecar must be added here before parser implementation
  proceeds for that surface.
- `unsupported` entries are explicit non-carry-forward decisions for parser
  sidecar compatibility and must cite their successor subsystem or rationale.
