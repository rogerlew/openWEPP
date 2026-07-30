# Uncertainty And Scope

- Atmospheric forcing: the best Flerchinger et al. all-sky combinations had
  about `24.5 W m^-2` half-hourly/hourly RMSD and `14.9 W m^-2` daily RMSD.
  Cloud uncertainty and diurnal timing dominate; extreme-latitude winter is a
  stated limitation.
- View geometry: Rutter et al. identify canopy density/sky view as the
  first-order uncertainty. Hemispherical-image thresholding itself introduces
  uncertainty. EB-02 will derive sky view from existing canopy state using an
  admitted diffuse-transmission operator; remote observations may evaluate
  uncertainty but are not required inputs. A canopy-cover alias would still
  add unquantified structural error.
- Canopy temperature: measured sub-canopy air is an effective proxy in
  homogeneous stands. Open/above-canopy air misses clear, calm nighttime
  decoupling. Sunlit gaps and edges can have warm trunks and require more
  explicit treatment.
- Emissivity: the selected formulation uses effective canopy and snow
  emissivity of exactly one, matching the source simplification. It does not
  claim material emissivity is exactly one. A future non-unity formulation
  must include reflected radiation and multiple gray-surface exchange.
- Scale: the selected formulation is a stand-average model. It does not claim
  individual-tree wells, forest-edge gradients, ray-traced terrain, or
  horizontally heterogeneous canopy radiation.
- Phenology: Rutter et al. include leaf-off birch, which supports the
  two-component form. The derived mapping must preserve the native-forest
  structural canopy floor so leaf-off LAI does not imply a completely open
  hemisphere.
