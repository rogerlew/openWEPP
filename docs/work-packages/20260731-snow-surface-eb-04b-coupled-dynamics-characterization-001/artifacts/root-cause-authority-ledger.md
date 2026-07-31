# Root-Cause And Authority Ledger

Evidence: `Static + Ran`

| Defect family | Proven mechanism | Current authority | Correction owner | First action | Forbidden shortcut |
| --- | --- | --- | --- | --- | --- |
| `DEFECT-EB04B-THERMAL-001` (17 cases) | Positive cold content divided by vanishing ice mass implies `T <= 0 K` before the next carrier is evaluated; the exact crossing driver is not retained. | `SC-SNOWENERGY-001` requires one-minute continuation below `1 kg m^-2`, bounded vapor mass, exact latent/mass coupling, no clamp, and typed rejection, but does not define thin-pack extinction or a phase/energy outcome at this boundary. | EB-04C | Close the thin-pack energy-domain/extinction authority gap: reconcile Marks/SNOBAL/libsnobal treatment, CoE snow-existence authority, negative surface energy, mass depletion, and bounded sublimation before amending the contract and implementing authoritative behavior. | Tuning sublimation, clamping temperature, replacing it with air temperature, loosening the guard, deleting cold content, or merely rejecting one substep earlier. |
| `DEFECT-EB04B-THERMAL-002` (5 cases) | A valid-Kelvin but extreme-cold state makes the SNOBAL ice-vapor-pressure term underflow to zero inside effective conductivity. | The contract requires `T > 0 K` and the SNOBAL formulation but does not state a positive numerical lower bound for `e_si(T)` or disposition underflow. | EB-04C | Determine whether authoritative thin-pack treatment excludes the state or whether the constitutive/numerical formulation requires an authority-backed domain treatment. | Relabeling it as an impossible temperature, substituting a positive epsilon, or loosening the positive-pressure type. |
| `DEFECT-EB04B-GEOMETRY-001` (2 cases) | A fragment excluded by a `1e-9 m` SWE filter has physical depth greater than the independent `1e-9 m` depth tolerance. | `INV-SNOWENERGY-021` requires exact active/lower mass and depth reconstruction; bounded normalization requires explicit dimensional provenance. | EB-04D | Close the filter/reconstruction inconsistency with a dimensionally coherent fragment lifecycle or explicitly authorized bounded normalization, then prove mass, depth, liquid, cold-content, and layer-transition closure. | Increasing the tolerance, ignoring the layer, filtering by an unrelated unit, or canonicalizing and proceeding without contract authority. |

## EB-04C Decision Boundary

The analysis does not authorize a production fix. EB-04C must determine whether
existing Marks/SNOBAL/libsnobal authority supplies a thin-pack/no-snow or
energy-limited transition compatible with CoE mass authority. If it does,
EB-04C is a contract-first defect closure. If it does not, EB-04C must perform
or open focused authority research and may adjudicate a named model limitation
only after the authority routes are exhausted.

The corrective surface must cover both the S/LS-associated population and the
two L-only cases. A sublimation coefficient round is not indicated; terminal
amplification was not identifiable from the retained boundary.

## EB-04D Decision Boundary

The two geometry cases share one deterministic numerical mechanism and do not
need a broad science search. EB-04D should reconstruct the fragment lifecycle
around split, coalesce, melt, liquid, and scalar aggregation and select a
dimensionally coherent correction under invariant 021. The correction must
retain the existing typed failure for states outside any authorized bound.
