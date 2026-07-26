# Canopy Phenology Pre-calibration Corpus

This directory is the normalized, immutable-role corpus installed by
`CANOPY-CAL-03`. It contains no newly acquired research object and no fitted
native parameter. Every record is transcribed from a retained source or from
the completed CAL-01/CAL-02 evidence identified in
`source-object-manifest.csv`.

## Files

- `records.csv` is the tidy record ledger. Blank numeric cells mean that the
  retained source provides a qualitative statement, range, dataset object, or
  diagnostic definition rather than a scalar. Blank never means zero.
- `source-object-manifest.csv` binds each input object to its SHA-256 digest.
- `source-metadata.csv` is the relational metadata table joined by
  `source_object_id`; it records coordinate availability/binding, controlling
  license or terms, exact transformations, and missing-value semantics for
  every record.
- `SHA256SUMS` binds the two corpus tables and this README after generation.

The five record classes are deliberately disjoint:

- `OBSERVATION`: retained field measurement or exact source summary;
- `FITTED_OPERAND`: an operand selected in Bill Elliot's legacy workflow;
- `DERIVED_DIAGNOSTIC`: a frozen calculation definition, not measured data;
- `LEGACY_COMPARISON`: a report value against which legacy reproduction was
  assessed; and
- `MODEL_OUTPUT`: a CAL-02 machine result.

Only `OBSERVATION` records may be eligible for calibration or holdout. The
package-level `calibration-holdout-ledger.csv` freezes each record's role.
`FITTED_OPERAND`, `DERIVED_DIAGNOSTIC`, `LEGACY_COMPARISON`, and
`MODEL_OUTPUT` cannot be promoted to observations by numerical agreement.

## Normalization and limits

Units and signs are source-facing. A negative elevation response means the
event occurs earlier with increasing elevation. Calendar descriptions such as
“early May” and “late October” remain text; they are not converted to invented
dates. The Hubbard LAI landscape range preserves the retained synthesis'
`3.5–8` extrema and separately notes that reference W6 averages “a little less
than 6”; it does not promote Bill's rounded `5.8`.

The Harvard and Marcell rows identify already-retained daily snow observation
objects. Their row-level values remain in the parent `sites/` and `profiles/`
tables and are not duplicated here. They are downstream evaluation authority,
not canopy-parameter fitting authority.

Hubbard forest-floor mass is loss-on-ignition organic mass; Santee forest-floor
mass is oven-dry bulk mass. They are intentionally not treated as
interchangeable measurements. Regional Hobcaw litterfall is context only and
cannot validate Santee leaf-, needle-, or fine-woody flux.

## Rebuild

From the repository root:

```sh
sha256sum -c tests/fixtures/cancov_forest/observations/canopy_phenology/SHA256SUMS
```

CSV parsing must preserve all text exactly and reject duplicate `record_id`
values. No generated run output belongs in this fixture directory.
