# EB-02 Operand Readiness

Caption: Readiness of the eight principal operands needed by the selected
two-component stand-scale longwave formulation. Green operands already exist
in typed runtime forcing; amber items need a science-contract decision or
active-state binding. Sky view is amber because EB-02 must bind its internal
derivation from canopy state, not because users must supply another input.

- Question: After resolving the science equation, what still prevents EB-02
  implementation?
- Population: Current openWEPP direct winter runtime and the selected
  homogeneous-stand formulation.
- Units: Categorical readiness; no numeric axis.
- Processing: Statuses come from `operand-readiness-ledger.csv`.
- Interpretation: Air temperature and dewpoint are present and an atmospheric
  estimator is authoritative. Solar/cloud forcing exists, but its legacy cloud
  fraction still needs reconciliation with the selected source mapping. The
  next structural step is a deterministic hemispherical sky-view mapping
  derived from existing canopy cover, LAI, structural cover, and height where
  the governing geometry makes height relevant. FSM2 diffuse transmission
  supplies the admitted Beer-law base; EB-02 must define the effective
  vegetation-area composition and cannot silently equate canopy cover with sky
  view.
- Uncertainty: “Available” means a runtime surface exists, not that its
  measurement height or uncertainty is ideal. Hemispherical photographs,
  LiDAR, and other observations may test the derived mapping but are not
  implementation prerequisites.
- Limitation: This figure does not imply production authorization; a canonical
  contract amendment and implementation package remain required.
