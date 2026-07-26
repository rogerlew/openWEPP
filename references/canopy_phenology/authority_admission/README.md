# Canopy Authority Admission Sources

These immutable source objects were downloaded on 2026-07-26 from the public
Environmental Data Initiative PASTA API for the CAL-04/CAL-05 authority
admission package.

The directory-local `.gitattributes` treats retained CSV and XML research
objects as binary so Git preserves their source-native bytes and line endings.

- Hubbard Brook packages `knb-lter-hbr.51.16`, `.49.11`, and `.50.10` are
  CC BY 4.0.
- Harvard Forest packages `knb-lter-hfr.3.37`, `.161.20`, and `.324.5` are
  CC0 1.0.

The EML files are the source-native metadata and method authority. CSV files
retain their original downloaded bytes. `SHA256SUMS` binds the retained copy.
These objects must be cited under the instructions in their EML metadata.

CAL-04 admits the Hubbard Brook phenology series as calibration evidence and
Harvard HF003 as an independent site holdout. CAL-05 uses HF161/HF324 and
Hubbard fine/coarse litter only as partial authority because their published
material classes do not isolate fine woody mass from all other nonfoliar
material.
