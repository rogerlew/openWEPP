# Gate 1: Schema, Profile, And Initial-State Authority

Status: `PARTIAL PASS / selected profile and state BLOCKED`

Evidence mode: `Ran + Static + primary-source inspection`

## Reproduced Population

The candidate columns are exact CSV fields 16 and 18: chestnut oak ID `805`
and eastern white pine ID `807`. Each has one profile-name record plus all 71
parameter values. The required exact 71-by-2 ledger is
`selected-field-ledger.md`. Rectangular completeness is not authority.

The first header row supplies one column-level bibliography bundle rather than
a row-to-source mapping. It does not identify which paper, table, species,
measurement, location, unit conversion, aggregation, or calibration produced
any particular cell.

Targeted follow-up read White et al. (2000), Reich et al. (1999) Table 2,
Hwang et al. (2009) Tables 2-3, Ford et al. (2010), and the Coweeta/Harvard
replacement-state routes. They recover partial lineage but not a consistent
cell map. Hwang is particularly discriminating: several oak cells match
catchment-level values, other cells disagree with the species row, and the
paper explicitly identifies composition weighting and non-species allocation
and phenology inputs.

Material examples demonstrate why finite values cannot be admitted wholesale:

| Field | Chestnut oak | Eastern white pine | Terminal issue |
| --- | ---: | ---: | --- |
| K optical sum | `0.5+0.31+0.22=1.03` | `0.8+0.1+0.1=1.0` | oak violates exact component closure |
| `ustar_overu` | `-999.9` | `-999.9` | explicit unsupported sentinel |
| `max_heat_capacity` | `0` | `0` | cannot authorize the source storage-heat law |
| `epc.max_lai` | `6` | `20` | Ford stand peaks are `6.2` and `7.2`, not these cells |
| `epc.vpd_close (x1000)` | `19127` | `3100` | mismatched parser key and unscoped transformation |
| allocation flag | `dickenson` | `waring` | selected branches and hidden coefficients lack authority |

## Schema And Acquisition Decisions

- Preserve all exact raw bytes, keys, occurrences, and lexical values.
- Prohibit every one of the 53 parser-only defaults.
- Treat the four decorated keys for SLA, LAI ratio, and VPD thresholds as
  candidate aliases only after reviewed source and contract authority proves
  semantic identity, units, cadence, and basis. Implementation may later prove
  parsing and byte identity but cannot establish scientific meaning.
- Reject `mortality -> epc.daily_mortality_turnover` as an alias because cadence
  and semantics are not established.
- Accept only caller-supplied local bytes bound to repository, immutable commit,
  repository-relative path, and SHA-256.
- Reject mutable `master` acquisition, network schemes, search paths, and all
  runtime fallback.
- Keep immutable raw evidence, resolved typed parameters, and dated initial
  state as three separate versioned objects.

These requirements admit the schema-form portion of `AUTH-RHEC-001` and close
`AUTH-RHEC-016` acquisition authority in `SC-VEGETATION-001` version 3. The
complete consumed-field declarations and reviewed aliases remain missing.
Nothing supplies the selected profile values or initializer, and no runtime
implementation is authorized by this package.

## Initial-State Boundary

The two GIS generator paths derive pools and roots from fixed row positions,
hard-coded `333.33` deadwood C:N and `0.05` leaf allocation, nonfinite-to-zero
behavior, and unproven root depths. Initialization can use a profile SLA that
the runtime parser later ignores.

Ford et al. supplies dated 2005-2006 stand/species observations for chestnut
oak in Coweeta WS18 and eastern white pine in adjacent WS17, including peak LAI,
basal area, density, DBH, height, sapwood area, and leaf area. Day/Monk and
related Coweeta sources add biomass/NPP evidence. These sources disprove a
global “no observations exist” claim, but they do not provide one compatible
state surface containing every required leaf/root/wood C/N pool. More
fundamentally, this package selected profile identities but no simulation
stand, plot, date, topology, area, or age/size distribution. Initial state is
not intrinsic to a species profile.

## Verdict And Exact Lift Condition

`AUTH-RHEC-016` passes at the authority level. `AUTH-RHEC-001` is partial:
schema form is admitted, while the complete selected manifest and aliases are
missing. `AUTH-RHEC-002` and `AUTH-RHEC-015` remain authority-missing. Gate 1 can lift
only with either:

1. a field-to-primary-locator/domain map plus a selected stand, date, topology,
   and complete compatible initial-state observations; or
2. replacement parameter and initializer sets whose every consumed value,
   equation, unit, ecosystem domain, and observation/synthesis step is
   independently admitted.

The inspected partial lineages cannot satisfy either route. Gate 1 therefore
blocks selected-profile promotion and the package cannot close `complete`.
