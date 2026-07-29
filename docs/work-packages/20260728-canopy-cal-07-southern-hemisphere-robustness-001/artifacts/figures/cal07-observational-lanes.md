# Admitted PhenoCam Greenness Observations in the Two CAL-07 Lanes

## Caption

Quality-admitted raw GCC90 observations for the Beza Mahafaly deciduous
broadleaf ROI `DB_1000` (top) and Alerce Costero evergreen ROI `EN_1000`
(bottom). Beza contributes 934 camera days from 2023-07-04 through 2026-07-24;
Alerce contributes 925 days from 2023-01-24 through 2026-07-24.

## How to read it

The vertical scales differ between panels and are shown explicitly. The lines
display the camera greenness signal in its original GCC90 units; they are not
normalized and should not be compared as absolute canopy amount. Gaps longer
than seven days are left disconnected so missing camera coverage is not
mistaken for an observed trajectory.

## Plain-language takeaway

Both planned observational lanes contain clear multi-season information and
remain valuable. Beza shows a pronounced deciduous seasonal cycle; Alerce has
a smaller but real color cycle within an evergreen-class ROI. The forcing
failure—not a lack of camera evidence—prevents their planned comparison with
the frozen model ensemble.

## Methods and source binding

Rows are admitted only when `image_count > 0`, raw `gcc_90` is finite, and
`outlierflag_gcc_90 == 0`. Source-smoothed values and pre-camera smoothing
extensions are excluded. The embedded SVG metadata binds
`inputs/observations.csv`, SHA-256
`82c970c334da0fdbd951e762961e5ce055097867658017d32341628f86f4ab36`.
ROI identities, provisional status, acknowledgements, archive URLs, and
source digests are retained in `artifacts/source-authority.md` and
`artifacts/source-manifest.csv`.

## Limitations

GCC90 is a relative camera-color proxy. It is not absolute LAI, foliar mass,
canopy cover, or a measured evergreen foliage floor. Both sites share the
PhenoCam processing method, and the products are provisional. The two sites
are geographically and climatically independent assignments, not independent
measurement technologies.

## Accessibility

The panels use different high-contrast line colors and name the site and ROI
inside each panel. Missing intervals are represented as gaps. Axes show dates
and panel-specific numeric ranges, and the SVG includes title, description,
and source metadata.
