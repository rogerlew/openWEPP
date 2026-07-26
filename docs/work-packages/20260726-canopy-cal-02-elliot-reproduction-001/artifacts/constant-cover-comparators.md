# Constant-Cover Comparator Admission

Evidence class: `Source inspection plus commissioned-report method`

## Decision

CAL-02 will use the recovered, source-native managements directly as the
site-specific constant-cover comparators:

| Site | Comparator | SHA-256 | Initial residue |
| --- | --- | --- | --- |
| Hubbard Brook | `hubbard_brook_unassailable_sensuousness/inputs/p1.man` | `839407d2cc294722a1053e7a6e172d93d12b508355d5378d88c41a4118c8595e` | `1.00 kg/m2` |
| Santee | `santee_clean_burning_griddle/inputs/p2.man` | `e71e883556f60785da429cfb77324db22cc00be09a3e5a5f3035176070335a6d` | `1.45 kg/m2` |

Both identities are already covered by their site `SHA256SUMS` manifests.
The machine-readable fixture index is
`tests/fixtures/canopy_phenology/elliot_reproduction/constant-cover-comparators.json`.
No duplicate or regenerated management file is needed.

## Semantic basis

Both files explicitly declare:

- `With no Senescence or decomposition`;
- `For no growth, no decomp, no senescence`;
- no operation, surface-effect, contour, or drainage scenarios; and
- fixed initial canopy and ground-cover operands.

The files are identical except for initial residue mass. This preserves each
site's WEPPcloud mature-forest condition while isolating the management
comparison against Bill's perennial hardwood or mixed-forest file.

Bill's report describes the constant approach as setting growth, leaf drop,
and decomposition to zero so canopy, rill cover, and interrill cover remain
constant. For Hubbard he says he transcribed the initial-condition and plant
variables from the WEPPcloud `p1.man`; the Santee section says the management
was then changed to constant cover. The recovered source files provide those
operands directly.

## Claim boundary

This admission reproduces Bill's analytical constant-cover design with exact
site-specific WEPPcloud management inputs. It does not claim that these source
files are byte-identical to a management file serialized by the WEPP Windows
builder, nor that a new run must match Bill's unavailable Windows outputs
byte-for-byte. Those are historical implementation-equivalence questions, not
prerequisites for the canopy-phenology comparison.
