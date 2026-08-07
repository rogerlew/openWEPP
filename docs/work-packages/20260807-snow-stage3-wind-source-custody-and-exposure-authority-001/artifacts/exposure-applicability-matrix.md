# Exposure Applicability Matrix

Status: `complete / scientific authority hold`.

Evidence mode: `Static`.

The fixture manifests identify each modeled hillslope land use as forest, but
do not classify the GRIDMET cell's effective wind exposure or provide an
authoritative forcing-to-target linkage. SNOTEL station coordinates/metadata
do not turn gridded wind into a station anemometer and cannot supply that link.

| Site | Forcing exposure | Target | Linkage | Disposition |
| --- | --- | --- | --- | --- |
| Mica Creek | `UNRESOLVED` | modeled NLCD evergreen forest; `cancov=0.9`; physical aerodynamic class unresolved | absent | `AUTHORITY_MISSING` |
| Niwot | `UNRESOLVED` | modeled NLCD evergreen forest; `cancov=0.9`; physical aerodynamic class unresolved | absent | `AUTHORITY_MISSING` |
| Paradise | `UNRESOLVED` | modeled NLCD evergreen forest; `cancov=0.9`; physical aerodynamic class unresolved | absent | `AUTHORITY_MISSING` |
| Snowbird development | `UNRESOLVED` | selected hillslope modeled NLCD evergreen forest; `cancov=0.9`; physical class unresolved; non-decisive | absent | `AUTHORITY_MISSING` |

No site is labeled `APPLICABLE` or `INAPPLICABLE`: both require two-sided
authority, and absence does not prove conflict. Numeric winds, carrier
residuals, and the neutral height bound were not exposure evidence. No canopy
operator or attenuation is licensed. Paradise WY2015 remains outside this gate.
The model landuse/canopy parameter establishes target intent only; it does not
classify real aerodynamic exposure or link watershed-centroid GRIDMET wind to
the modeled control volume.
