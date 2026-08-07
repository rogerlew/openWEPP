# Disposition

Status: `package complete / persistence HOLD`.

Evidence mode: `Ran`.

The exact four-site attempt completed successfully with valid evidence at
`e07cdbdf976b9cfeeb3d8ac825411ee41ad1b737`. Terminal scientific classes are
`WIND_FORCING_EXPOSURE_UNRESOLVED` and
`MULTIFACTOR_OR_INCONCLUSIVE`.

Persistence is `HOLD`. Decisive blockers are:

1. no per-site physical exposure/reference-height authority for CLI/GRIDMET
   wind;
2. physical magnitude envelopes and stability-geometry equation choice remain
   `NOT_EVALUABLE`;
3. Paradise WY2015 exceeds the frozen support-omission threshold; and
4. canopy aerodynamic authority is missing, but current evidence does not prove
   a canopy operator is required.

Raw and bounded vapor agree tuple-by-tuple without capacity truncation. This
eliminates `VAPOR_OPPORTUNITY_TRANSFER_MISMATCH` for the retained cohort; it
does not validate the absolute turbulent magnitude. The crossing is associated
with numerically dominant sensible and latent terms in the reported,
separately reduced within-day evolution, but available authority
cannot identify a unique physical cause.

Protected boundaries pass: no Rust production/schema, fixture, observation,
default, WAT, HBP, PASS, CoE ownership, persistence, promotion, or cutover
change occurred. Assurance remains DRAFT.

Both terminal verifiers returned `GO` after finding correction. At exact clean
head `2719512a125445be267e08ca2cc797bb28c1e0ef`, full workspace correctness
passed `2,288/2,288` and quick passed `2,239/2,239`. The package closes; the
scientific persistence hold transfers to the wind source-custody/exposure
authority follow-on.
