# CAL-07 Source Authority and Limitations

Evidence class: `Static: retained source objects`

## PhenoCam

Both lanes are geographically and climatically independent sites not used in
CAL-04B fitting. They share the PhenoCam measurement and processing system, so
they are not independent measurement methods. The retained ROI records bind
`DB_1000` to “Foreground tropical dry forest” and `EN_1000` to “EN tree tops
to avoid shadows.” Daily products are provisional.

The processing and method lineage is PhenoCam Dataset V3, DOI
`10.3334/ORNLDAAC/2389`, with Young et al. (2025), DOI
`10.5194/essd-17-6531-2025`. That curated release ends in 2023; the retained
2024--2026 provisional products are not represented as curated V3 records.
The live fair-use page retrieved 2026-07-28 states CC BY 4.0. The generated
site metadata contains stale Dataset V1
CC BY-NC-SA wording. For this retained V3/provisional acquisition the current
live provider statement governs; the conflict is preserved here rather than
silently erased.

Requested acknowledgements:

- Beza Mahafaly: Ankoatsifaka Initiative for Dry Forests, 501c3.
- Alerce Costero: National Agency of Research and Development (ANID, Chile),
  grants FONDECYT 1211652 and FONDAP 15110009.

## NASA POWER

The retained daily point responses are gridded/reanalysis evidence, not
on-site meteorological observations. Both report API v2.9.5, LST time basis,
sources GEOSIT/MERRA2/POWER, Celsius units, and `-999` fill semantics.

- Beza site elevation is 165 m; returned grid elevation is 277.79 m.
- Alerce site elevation is 840 m; returned grid elevation is 99.4 m.

The Alerce mismatch is material. Timing residuals cannot be attributed solely
to hemisphere logic or GSI transferability, and no unauthorized lapse-rate or
downscaling correction is applied.

## Exact acquisition endpoints

- `https://phenocam.nau.edu/data/archive/bezamahafaly/ROI/bezamahafaly_DB_1000_provisional_data.zip`
- `https://phenocam.nau.edu/data/archive/alercecosteroforest/ROI/alercecosteroforest_EN_1000_provisional_data.zip`
- `https://power.larc.nasa.gov/api/temporal/daily/point?parameters=T2M_MAX,T2M_MIN,T2MDEW&community=AG&longitude=44.6289&latitude=-23.6558&start=20220101&end=20260724&format=JSON`
- `https://power.larc.nasa.gov/api/temporal/daily/point?parameters=T2M_MAX,T2M_MIN,T2MDEW&community=AG&longitude=-73.4439&latitude=-40.1726&start=20220101&end=20260724&format=JSON`
- `https://phenocam.nau.edu/webcam/fairuse_statement/`
