# Authority Law and Operand Lineage

Status: `prospective re-review candidate`

Evidence class: `Static`

## Authority disposition

Three claims remain separate:

| Claim | Science implementation | Calibration evidence | Identifiability | Mode/limit |
| --- | --- | --- | --- | --- |
| Predictive evergreen needle ground deposition | `AUTHORITY_MISSING` | `NOT_CALIBRATION_READY` | `NOT_ASSESSED` | Missing live-to-litter dry-mass and temporal deposition authority |
| Predictive fine-woody ground deposition | `AUTHORITY_MISSING` | `NOT_CALIBRATION_READY` | `NOT_ASSESSED` | Missing branch/crown/stand state and turnover-to-deposition authority |
| Authenticated daily external boundary | `NOT_IMPLEMENTED` | `NOT_APPLICABLE` | `NOT_APPLICABLE` | `PRESCRIBED_BOUNDARY_ONLY` or genuinely exhaustive `MEASURED_DAILY_BOUNDARY` |

The third row is the only implementation candidate. It is an exogenous
boundary interface, not predictive canopy biology, a calibration parameter,
or evidence that native litter sources are complete.

## Candidate boundary model

Each material has an independent declaration:

```text
surface_litter_forcing:
  vegetation:
    functional_classes: <typed non-empty set>
    authority: <authenticated classification record>
  needle:
    status: complete | not_represented | not_applicable
    payload: <required only for complete>
  fine_woody:
    status: complete | not_represented | not_applicable
    payload: <required only for complete>
```

`not_represented` means model-envelope incompleteness and carries no numeric
series. `not_applicable` also carries no numeric series and requires an
authority-backed vegetation/material classification: needle litter can be
not-applicable only when all represented foliage classes are non-needleleaf;
fine wood can be not-applicable only when woody above-ground vegetation is
authoritatively absent. Neither state is inferred from `evergreen_fraction`
or aggregate `structural_biomass_kg_m2`.

A `complete` payload contains:

```text
mode: prescribed_scenario | measured_daily
support_start: YYYY-MM-DD
support_end: YYYY-MM-DD
calendar: proleptic_gregorian
material:
  class: needle_foliage | fine_woody
  species_or_functional_type: <non-empty>
  included_material: <non-empty>
  excluded_material: <non-empty>
  maximum_diameter_mm: <required for fine_woody>
  bark_treatment: included | excluded | separated
mass_basis:
  state: oven_dry | dry_to_constant_mass
  drying_temperature_c: <finite>
  drying_duration_hours: <optional positive>
  constant_mass_criterion: <required when applicable>
  horizontal_area_basis: true
  units: kg_dry_mass_m2_day
spatial_support:
  site_or_plot: <non-empty>
  ofe_binding: <non-empty>
authority:
  source_identity: <non-empty>
  source_uri_or_path: <non-empty>
  access_or_version_date: YYYY-MM-DD
  claim_anchor: <page/table/field/method>
  digest_algorithm: sha256
  source_digest: <64 lowercase hex>
original_observation:
  support_start: YYYY-MM-DD
  support_end: YYYY-MM-DD
  resolution: exact_daily | interval
  interval_definition: <required for interval>
  units: <typed original units>
executable_forcing:
  path: <immutable UTF-8 CSV object>
  digest_algorithm: sha256
  executable_digest: <64 lowercase hex>
derivation: none | <derived-payload metadata>
```

Rules:

- supports are inclusive, explicit, per tissue, and independent of first/last
  nonzero events;
- simulation dates claiming `complete` must lie wholly inside that tissue's
  support; outside-support access, calendar mismatch, or leap-day ambiguity
  fails closed;
- dates are unique, strictly increasing, and within support;
- `measured_daily` requires an exhaustive daily acquisition method. An
  omitted date inside support is an authenticated measured zero only when the
  source records exhaustive observation for that date;
- `prescribed_scenario` entries are operator-asserted exact-day boundary
  conditions. Omitted dates inside a declared complete support are scenario
  zeros, never observed natural zeros;
- interval trap totals are retained as interval observations and are not
  executable in this daily interface. Assignment to collection date,
  interpolation, uniform division, repetition, extrapolation, or
  climatological wrap is prohibited without separately admitted temporal
  authority;
- undocumented carbon, wet-to-dry, area, material, or temporal conversions
  fail closed; and
- unused entries, partial overlap, and every `not_represented` material are
  reported in outputs.

The executable forcing object has the exact byte grammar:

```text
date,deposited_kg_m2\n
YYYY-MM-DD,<finite nonnegative base-10 decimal>\n
...
```

It is UTF-8 without BOM, uses LF newlines including one final LF, has the
header exactly once, and orders unique dates strictly increasingly. SHA-256
`executable_digest` covers every raw byte of that file. The runtime resolves
`executable_forcing.path` under the same authenticated input root as the
management object, reads those exact bytes, verifies the digest before
parsing, and treats this file—not an in-memory reserialization—as the object
that authenticates executable entries.

`source_digest` likewise covers every raw byte of the immutable object
resolved exactly by `source_uri_or_path`; directory URIs and mutable query
results are invalid. `original_observation` always records the source
object's temporal support, resolution, and units, including for a direct
scenario.

Transformation presence depends on derivation, not boundary mode:

- For an original executable payload, `derivation: none` is required. The
  source object and executable forcing object must resolve to the same file,
  and `source_digest == executable_digest`. No transformation identity or
  transformed digest is present. A directly authored prescribed scenario is
  this identity case: its original support is
  `exact_daily`, its original units are `kg_dry_mass_m2_day`, and it remains
  explicitly non-observational.
- For a derived executable payload in either mode, `derivation` requires
  transformation identity, version, all input object identities/digests,
  transformation algorithm authority, and the executable forcing object's
  `executable_digest`. The source and executable objects remain distinct.
  Interval-to-daily derivation is rejected unless the transformation metadata
  cites separately admitted temporal-disaggregation authority; metadata alone
  never authorizes disaggregation.

Implementation disposition: this increment deliberately implements only the
original identity case. Every derived or interval object fails closed until a
successor admits typed input-object identities/digests and separately
authenticated transformation/temporal authority.

Every `complete` payload must match the authority-backed top-level vegetation
classification: material class and species/functional type must be compatible
with the represented vegetation. Its site/plot, spatial support, lane, and
OFE binding must resolve to the active simulation lane/OFE. Any material,
vegetation, site, plot, lane, or OFE mismatch fails closed.

## Daily source and open-system ledger

For lane `i` and date `d`, let:

- `L(i,d)` be the existing internally debited CP-GSI02 leaf-off transfer;
- `N(i,d)` be authenticated external needle deposition or zero only under the
  complete-support rules above;
- `W(i,d)` be authenticated external fine-woody deposition under the same
  rules; and
- `E(i,d) = N(i,d) + W(i,d)` be external boundary influx.

```text
Q(i,d) = L(i,d) + E(i,d)
```

All terms are finite and nonnegative in `kg dry mass m-2 d-1`. There is no
stock-times-turnover calculation. Internal plant/residue closure cancels the
leaf debit and credit:

```text
delta(plant_foliar + represented_residue)
  = -L + L + E - authorized_losses + other_authorized_transfers
  = E - authorized_losses + other_authorized_transfers
```

`N` and `W` have no openWEPP canopy debit because they cross an external
system boundary. They must be labeled external input, not generated mass.

## Parallel residue recurrences

Let `S`, `I`, and `R` be the prior surface, interrill-ground, and rill-ground
areal states; `f` the same authorized surface decay factor; and
`A_s`, `A_i`, and `A_r` the existing typed post-decay management/action
operators. Other already-authorized additions are shown as `O_*`.

```text
S_pre = S + Q + O_s
I_pre = I + Q + O_i
R_pre = R + Q + O_r

S_next = A_s(S_pre * f)
I_next = A_i(I_pre * f)
R_next = A_r(R_pre * f)
```

`S`, `I`, and `R` are parallel `kg m-2` contract representations. The same
areal source projection into each is required by `INV-RESIDUE-020/021`; the
three values are never summed as three independent global masses. `Q` is
applied exactly once inside each recurrence, with negative tests for both
pre-addition plus downstream re-addition and omitted projection.

The surface state `S_next` drives residue partition/mass-to-depth and frost.
`I_next` and `R_next` drive interrill/rill cover derivation and erosion; their
area-weighted ground representation is
`G_next = w * I_next + (1 - w) * R_next`, not `I_next + R_next`.

Independent reconstruction must recover each `*_pre`, decay loss
`*_pre * (1-f)`, action/removal loss, `*_next`, the weighted ground state,
residue cover, residue depth, frost input, and erosion inputs from published
operands. Producer-only agreement is insufficient.

## Rejected predictive equations

### Evergreen foliage

`evergreen_live_foliar_stock / leaf_longevity` may describe gross annual
production/turnover under qualified steady-state assumptions. Deposited
needle dry mass additionally needs authoritative pre-abscission mass
retention and deposition timing. Neither exists here.

### Fine woody material

Lim et al. models turnover using branch biomass, annual height increment or
crown ascent, stand/relative density, shade-tolerance class, and crown state.
openWEPP carries none of that branch-resolved state, and:

```text
branch turnover != branch litterfall != same-day ground deposition
```

Attached-dead storage and in-canopy loss prohibit substitution of aggregate
structural biomass or a broad turnover rate.

## Contract-derived rejection vectors

Tests must reject:

1. interval observations passed as exact-day deposition;
2. unsampled dates treated as measured zeros;
3. missing, unrepresented, or inapplicable material encoded as numeric zero;
4. needle applicability inferred from evergreen fraction;
5. fine-wood applicability inferred from aggregate structural biomass;
6. incomplete authority, material, dry-mass, spatial, support, or checksum
   metadata;
7. undocumented conversions or temporal transformations;
8. outside-support, partial-overlap, or leap/calendar ambiguity;
9. source/executable digest byte-scope mismatch, contradictory derivation
   metadata, or noncanonical forcing-file bytes;
10. payload material, functional type, site, plot, lane, or OFE mismatch;
11. a source applied zero or twice within any parallel recurrence;
12. summing parallel surface/interrill/rill states as global mass;
13. external `N`/`W` portrayed as internally generated canopy mass; and
14. source-complete output when either applicable tissue is
    `not_represented`.

## Permitted claim after implementation

```text
science_implementation_status = IMPLEMENTED
calibration_evidence_status = NOT_APPLICABLE
identifiability_status = NOT_APPLICABLE
implementation_mode = PRESCRIBED_BOUNDARY_ONLY
# exclusively for prescribed_scenario

or

implementation_mode = MEASURED_DAILY_BOUNDARY
# exclusively for authenticated exhaustive measured_daily
```

The ADR-0042 triple is identical for either mutually exclusive boundary mode.
That claim applies only to the authenticated external boundary interface.
Predictive needle and fine-wood rows remain
`AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`.
