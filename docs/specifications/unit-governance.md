# Unit Governance Standard

Status: Active
Last updated: 2026-06-03
Scope: openWEPP science contracts, runtime boundary symbols, conversion
helpers, tests, and publication metadata

## Purpose

HPHYS0272 showed that a numerically finite value can still be scientifically
invalid when it crosses a runtime seam with the wrong unit. This standard makes
unit ownership explicit so future packages do not rely on symbol names,
comments, or downstream residuals to discover unit defects.

This document is canonical governance authority for unit handling in openWEPP.
It complements:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/architecture/unit-safe-boundary-types.md`
- `docs/specifications/science-contracts/unit-safe-boundary-types-contract.md`

## Authority Model

Unit authority is resolved in this order:

1. Canonical `SC-*` science contract variables, units, invariants, and alias
   maps.
2. Machine-readable boundary-symbol unit registry.
3. Typed boundary wrappers in `openwepp-unit-boundary` and
   `BoundaryValue` variants.
4. Named directional conversion helpers with provenance.
5. Publication schema metadata.

If these authorities conflict, the package must stop in `HOLD` until the
conflict is resolved. Runtime code must not silently reinterpret, rescale,
clip, or default dimensional values to make them plausible.

## Canonical Internal Units

The canonical runtime unit is the unit consumed by openWEPP process code after
parser-side projection and before publication. Legacy WEPP symbols remain the
canonical symbol names unless a science contract explicitly aliases them.

| Boundary class | Canonical internal unit | Notes |
| --- | --- | --- |
| Daily water depth/storage | `mm` | Includes `P`, `RM`, `Q`, `Ep`, `Es`, `Er`, `Dp`, `Pe`, `Snow-Water`, `Total-Soil`, and `SoilWaterTotal` publication-family depths. |
| Layer thickness and profile depth | `m` | Layer symbols such as `dg_####` and normalized profile depth surfaces remain meters unless publication explicitly converts to `mm`. |
| Volumetric water content | `m^3/m^3` | Legacy theta symbols and `mm/mm` publication labels must be explicitly aliased when both are used. |
| Hydraulic conductivity and process rates | contract-declared `m s^-1`, `mm h^-1`, or `mm d^-1` | Packages must preserve the contract-declared time base and may not infer it from suffix alone. |
| Solar radiation, daily | `Ly d^-1` at `radly` parser seam; `MJ m^-2 d^-1` after `radly * 0.04184` conversion | HPHYS0272 authority requires exactly one conversion before `hr_tmp` hourly synthesis. |
| Solar radiation, hourly | `MJ m^-2 h^-1` | Hourly publication symbols such as `winter.hourly.rad_mj_m2_####` must not carry Langley-scale values under `MJ` labels. |
| Temperature | `degC` | Contract alias maps must distinguish Celsius temperatures from dimensionless flags. |
| Wind speed | `m s^-1` | Any daily/hourly aggregation convention must be contract-declared. |
| Area | `m^2` | Boundary conversions from depth to volume require strictly positive area. |
| Volume | `m^3` | Watershed/channel/impoundment storage and routed volume surfaces use cubic meters unless publication says otherwise. |
| Flow | `m^3 s^-1` | Watershed/channel discharges and routed peak flow surfaces use cubic meters per second. |
| Sediment concentration | `kg m^-3` | Particle arrays and HBP serialization must preserve per-class units. |
| Mass | `kg` | Sediment detachment/deposition and routed mass surfaces use kilograms unless contract-declared otherwise. |
| Fractions and flags | dimensionless | Must be bounded or enumerated by contract; do not use dimensional wrappers. |

## Boundary Symbol Unit Registry

A machine-readable boundary-symbol unit registry is mandatory for dimensional
runtime symbols. The registry is implemented by HPHYS0274 and must record, at
minimum:

- canonical symbol,
- boundary/API aliases,
- unit label,
- dimension class,
- domain class,
- producer and consumer scope,
- owning `SC-*` contract and invariant,
- typed wrapper requirement or scalar exception,
- publication metadata mapping when the symbol is published.

The active registry implementation is
`crates/openwepp-sim-contract/src/units.rs`; the human-readable schema and
coverage report is
`docs/specifications/units/boundary-symbol-unit-registry.md`. Run
`tools/release/check_unit_registry.sh` as the mandatory local gate for packages
that add, change, or publish dimensional boundary symbols.

Packages must record registry gaps explicitly in their work-package artifacts
and keep affected production migrations in `HOLD` when unit ambiguity can
change process behavior.

## Typed Boundary Values

`BoundaryValue::scalar` is dimensionless by default. It is allowed for:

- enumerations, flags, counters, indexes, and branch selectors,
- pure fractions or dimensionless coefficients,
- tests that explicitly exercise scalar compatibility,
- legacy compatibility seams listed as scalar exceptions by contract or
  registry.

`BoundaryValue::scalar` is not acceptable final closure for new or migrated
high-risk dimensional runtime surfaces. High-risk surfaces include water
depth/storage, radiation, temperature, wind, hydraulic conductivity, process
rates, area, volume, flow, sediment concentration, mass, and layer/profile
geometry.

Dimensional runtime surfaces should use typed `BoundaryValue` variants or
typed constructors from `openwepp-unit-boundary`. If a required wrapper does
not exist, the package must either add it within scope or record a follow-up
package before claiming closure.

## Conversion Policy

Dimensional conversions must be named, directional, provenance-backed, and
tested. Production paths must not use unexplained raw conversion literals such
as `1000.0`, `0.001`, `3600.0`, `86400.0`, or `0.04184` for dimensional
changes.

Every conversion helper must state:

- source unit,
- target unit,
- formula,
- provenance anchor,
- expected domain,
- failure behavior for non-finite or out-of-domain inputs.

Correct constants used in the wrong direction are defects. Packages must avoid
"rounding into plausibility" by clipping, defaulting, or double-converting.

### Conversion Helper Authority

`openwepp-unit-boundary` owns first-wave dimensional conversion helpers for
runtime/kernel seams. Helpers must be named by source unit and target unit, not
by numeric constant. HPHYS0276 establishes the following canonical first-wave
helper classes:

- length/depth conversions: `m <-> mm`, `m <-> cm`, legacy `m <-> inch`,
- time/rate conversions: `h <-> s`, `m s^-1 -> cm h^-1`, legacy
  `m s^-1 -> mile h^-1`,
- radiation conversions: `Ly d^-1 -> MJ m^-2 d^-1` and uniform
  `MJ m^-2 d^-1 -> MJ m^-2 h^-1`,
- snow density/depth conversions: `snow depth m + density kg m^-3 <->
  water-equivalent m`, `kg m^-3 -> g cm^-3`.

Production code in guard-enforced paths must call these helpers instead of
spelling literals such as `0.04184`, `1000.0`, `0.001`, `3600.0`, `39.37`, or
`0.0254`. Existing production files that still contain raw conversion literals
must either be migrated by a follow-up package or documented as an explicit
guard exception with rationale and provenance.

### Raw Conversion Literal Guard

`tools/release/check_raw_unit_conversions.py` is the source-level guard for
raw dimensional conversion literals. The first HPHYS0276 enforcement wave
covers the high-risk SIMIMPL28/SIMIMPL29 winter radiation, snowpack/melt, and
WB19 drainage conversion seams. The guard must:

- fail on unauthorized raw conversion literals in enforced production files,
- ignore test modules and non-Rust fixtures by default,
- require a documented allow marker for any intentional raw literal exception,
- support explicit path arguments for contract-derived lint fixtures,
- report remaining all-production conversion inventory for follow-up planning.

## Publication Metadata

Output writers must publish unit metadata from the same authority as runtime
symbols. Until HPHYS0278 aligns writer schemas with the registry, output
metadata edits must include a local evidence note linking the column to its
canonical `SC-*` symbol and unit.

Publication column names may preserve legacy labels, but legacy naming does not
override canonical unit authority. If a publication column uses a different
unit from the runtime symbol, the conversion must be explicit and named.

HPHYS0278 establishes `openwepp-sim-contract` output-unit metadata as the
canonical writer-facing authority. Output schemas that attach a `units`
metadata key must resolve the `(schema_id, column_name)` pair through the
output-unit registry. Registry-backed rows must cross-check against the
boundary-symbol registry unit. Publication-only rows must carry an explicit
rationale rather than silently bypassing registry governance.

Dynamic key/value publication schemas that store the physical unit in a
row-level sibling column must not publish a fake fixed `units` value on the
numeric field. They must instead declare `unit_source = "units"` field metadata
and resolve the dynamic value column through the output-unit registry with
explicit publication-only rationale.

## Contract Requirements

Every kernel-affecting `SC-*` contract must include:

- a `Variables and Units` section for all externally relevant dimensional
  symbols,
- alias-map rows with unit checks whenever canonical and runtime/publication
  names differ,
- guard-map rows for unit invariants and invalid dimensional states,
- conversion provenance for every unit transformation in the touched scope,
- scalar-exception rationale for dimensional surfaces that cannot yet use typed
  wrappers.

Missing unit declarations, ambiguous aliases, or unguarded conversion seams are
non-promotable gaps unless the package is explicitly docs-only and records a
follow-up implementation package.

## Work-Package Gates

Every work package that adds, changes, or publishes dimensional symbols must
record unit-governance evidence:

1. contract authority updated before production code,
2. registry entry or explicit registry gap recorded,
3. typed boundary requirement or scalar exception recorded,
4. conversion helper use or raw-literal exception recorded,
5. output metadata linkage recorded when publication is touched,
6. validation evidence distinguishes `Static:` from `Ran:`,
7. dual reviews disposition all findings before closure.

Docs-only governance packages may close with static evidence if they create
the authority needed by follow-up implementation packages and do not change
runtime behavior.

## Follow-Up Remediation Queue

HPHYS0273 establishes authority only. Enforcement proceeds in this order:

1. HPHYS0274: implement boundary-symbol unit registry.
2. HPHYS0275: expand and apply typed dimensional `BoundaryValue` variants.
3. HPHYS0276: implement named conversion helpers and raw-literal guard.
4. HPHYS0277: add high hourly radiation physical flux guard.
5. HPHYS0278: align output unit metadata with registry authority.
6. HPHYS0279: lint `SC-*` unit sections and alias unit checks.

The queue may be split further, but it must not skip registry/conversion
authority when production physics behavior depends on unit interpretation.
