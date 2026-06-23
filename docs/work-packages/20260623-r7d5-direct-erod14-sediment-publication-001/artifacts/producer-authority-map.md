# Producer Authority Map

Status: executed-held.

## Candidate Surfaces

- Static: `DirectPublicationDayRow::from_day_frame` previously populated
  `DirectPublicationErosionOperands::zero_authority()` for every direct row,
  so erosion-active direct production could emit zero detachment/deposition and
  zero class concentration without a producer.
- Static: direct production executes
  `DirectFrameExecutor::run_publication_capture_with_interleaved_day_inputs`
  from lane seed surfaces and bypasses the compatibility scheduler. The direct
  day-span list covers normalization, storage, WB14/R4K, WB18/R4M, ET, R4O,
  snow, R4L, R4A, R4B, R4PQZ, and R3B; it has no direct EROD13/EROD14/EROD15
  span and no direct erosion state/shadow projection.
- Static: the shared EROD14 Wave-2 producer exists only in the compatibility
  hydrology kernel path through `Wb11HydrologyKernel::run_peak_runoff`, where
  `run_erod13_wave1_core`, `run_erod14_wave2`, and `run_erod19...` consume
  `HillslopeKernelRequest` state symbols and write `total_detachment_kg`,
  `total_deposition_kg`, `particle_class_count`, and
  `sediment_concentration_kg_m3_{class}`.
- Static: direct production seed surfaces may contain EROD14 seed symbols, but
  direct carried lane state does not include EROD14 class state (`gend`,
  `frcflw`, `frac`, `fidel`, `tcf1`, `sedmax`, `sed_frac`) or a typed EROD13
  state-update producer. Copying compatibility runtime aliases would violate
  this package's protected boundary.

## Authority Classification

- Direct-authoritative:
  - Direct hydrology publication operands for WAT/PASS water fields after R7D4.
  - Direct `erosion_producer_required` guard bit derived from
    `erod14_wave2_enabled` in the direct production day-input builder.
- Compatibility-derived / prohibited as authority:
  - `execution.runtime_surface.total_detachment_kg`,
    `execution.runtime_surface.total_deposition_kg`, and
    `execution.runtime_surface.sediment_concentration_kg_m3_0001`.
  - `execution.wb13_rows`, compatibility HBP bytes, compatibility public-output
    builders, or aggregate stale runtime aliases.
- Absent:
  - Typed direct EROD13 producer outputs required by EROD14.
  - Typed direct EROD14 class state carry and direct sediment publication
    projection.
  - Typed direct EROD15 publication operands for HBP/PASS class arrays.

## Decision

- Accepted hold:
  `HOLD-R7D5-DIRECT-EROD13-EROD14-EROD15-TYPED-PRODUCER-ABSENT`.
- Implemented fail-closed behavior instead of publishing fabricated zero
  sediment authority: direct production now sets
  `DirectPublicationDayInput.erosion_producer_required` when
  `erod14_wave2_enabled` is true, and row publication fails with
  `MissingDirectUpstream { upstream: "R7D5 direct EROD14/EROD15 sediment producer" }`.
- First action for the next package: add a typed direct erosion phase family
  that owns EROD13 inputs/outputs, EROD14 class state carry, EROD15 publication
  operands, downstream MOFE sediment carry, and anti-alias fixtures before
  removing the R7D5 guard.
