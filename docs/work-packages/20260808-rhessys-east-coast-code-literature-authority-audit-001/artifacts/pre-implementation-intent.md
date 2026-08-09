# Pre-Implementation Intent (Audit-Only)

Status: `frozen`

Evidence mode: `Ran + Static`

Frozen on: `2026-08-08`

## Exact Identities

| Surface | Identity | Intake state |
| --- | --- | --- |
| openWEPP | `86faf6fd22421372c6d9874b7bd0b7e1cabd439f`, branch `main` | clean |
| RHESSysEastCoast | `375c75b1cd2202217651dff43aa113d80b9c1118` | clean, read-only |
| GIS2RHESSys | `6b20883dea7c9fd92f71ec69eaca015ebf6dfe18` | clean, read-only |

The audit intent is scientific-authority characterization, authority-gap
closure where direct evidence permits it, bounded canonical gap/ownership
amendment, and prospective successor planning. It is not implementation,
calibration, validation, parity, activation, publication, or cutover.

## Frozen Population

The population is the vegetation-relevant transitive source closure beginning
at `construct_stratum_defaults`, `construct_canopy_strata`, the patch layer
loop, `canopy_stratum_daily_F`, `canopy_stratum_hourly`, and
`canopy_stratum_growth`; their scientific callees for radiation, interception,
aerodynamic and surface resistance, conductance, Penman-Monteith,
photosynthesis, phenology, respiration, allocation, turnover, mortality, root
state, and litter/C/N transfers; all 71 rows and 32 profile columns in the
pinned `vegCollection.csv`; and the exact R read/write path that serializes
selected columns to `stratum_*.def`.

## Evidence And Rights Posture

Source expressions, comments, and profile cells are discovery evidence only.
Scientific authority requires a primary-source locator, a reviewed derivation,
a physical invariant, or an existing canonical openWEPP authority. Copyrighted
or rights-ambiguous full text remains in the ignored
`references/copyrighted/` cache. Only affirmatively redistributable material
may enter `references/vendorable/`.

## Validation Intent

This is a documentation/reference/contract-only increment. Required checks are
the package-declared Markdown, rights, source-identity, contract/unit, and diff
checks selected from the exact terminal diff. Rust, Cargo, test-source,
runtime, dataset, deployment, and external-message changes are prohibited.
