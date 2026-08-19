# Live-owner source map

Evidence class: `Static`.

| Operand family | Required owner | Repository result | Disposition |
| --- | --- | --- | --- |
| Meteorology/radiation/precipitation | sealed half-hour receipt | present and opaque | ready |
| GSI | accepted CP-GSI01 daily receipt | present | ready |
| CO2/reference height | static forcing configuration | present and digest joined | ready |
| transaction/cadence | V10 lineage/fixed 1800 s scheduler | present | ready |
| snow/frost posture | staged hydrology/winter owner | present; snow-free predicate exists | ready |
| soil water/frozen | staged direct hydrology | present per OFE/layer | ready |
| soil temperature | staged soil thermal | present per OFE/layer; global collapse forbidden | ready |
| hydraulic conductivity | staged subsurface layer | `conductivity_m_s * 1000` | ready |
| matric potential | admitted per-root-layer hydraulic owner | current hydrology plus immutable Brooks--Corey configuration | ready |
| root identity/lateral geometry | V10 configuration | layer/root fractions and lateral length present | ready |
| soil-to-root path/gravity | admitted explicit geometry owner | ordered layer depth plus immutable stratum geometry | ready |
| accessibility | root membership + live frozen/domain state | projected and guarded | ready |
| ground optics | per-tile LSE configuration | present | ready |
| upward longwave | coupled LSE state/solver | V8 contract removes scalar forcing; removable from seam | ready |
| WB14 | immutable hydrology configuration | values exist in day/static inputs but need retained owner projection | implementable |
| internal runon | accepted routing publication receipt | the surface transaction can route internally, but its publication receipt is not installed at the default-off V10 seam | excluded; all nonempty runon fails closed |
| external runon | accepted upstream parcel owner | no general provider; typed unsupported is valid when boundary requires it | supported-domain guard |

WB14 wetting-front suction is rejected as a matric-potential source because it
is nonnegative infiltration-capacity geometry in metres, whereas vegetation
consumes a signed root-zone soil water potential in millimetres. Equal names do
not establish equation or owner equivalence.
