# V4 Shared-State Authority Selection

Status: `selected`

Evidence mode: `Static`

## Confirmed gap

V3 inherited a shared state with three C/N subpools per tissue but did not say
which leaf-C subpool owns exposed LAI. The implementation draft summed display,
storage, and transfer carbon. That makes future/donor carbon immediately
radiatively, physiologically, hydraulically, and interception-active. V1 E20's
display/storage/transfer identities instead require accepted movement into the
display pool before area appears.

V3 also inherited two previous offset-flux scalar fields. No admitted equation,
unit ledger, update order, or consumer uses them. Keeping them executable would
invite invented carry or numerical semantics.

## Selection

V4 selects displayed leaf carbon and nitrogen alone:

```text
LAI = leaf.display.carbon * SLA
SAI = LAI * sai_relation
RAI = (LAI + SAI) * root_to_leaf_area
Nleaf_area = leaf.display.nitrogen / LAI
```

The three serialized areas are integrity caches, not independent owners.
Storage C/N is non-display allocation state and transfer C/N is the E20 onset
donor; both contribute zero area and leaf capacity until accepted credit to display.

V4 removes `previous_leaf_offset_flux` and `previous_root_offset_flux`. It does
not replace them. Existing phase, timers, `previous_gsi`, and explicit tissue
pools retain the admitted phenology state.

## Rejected alternatives

| Alternative | Disposition |
|---|---|
| display + storage + transfer leaf C | rejected: future/donor C becomes exposed biomass |
| display + transfer leaf C | rejected: transfer donor bypasses accepted E20 movement |
| serialized leaf area as authority | rejected: cached area can diverge from elemental owner |
| carry legacy offset flux into V4 | rejected: no complete units/update/consumer authority |
| map offset flux into `previous_gsi` or timers | rejected: changes semantic identity |
| silently repair mismatched V3 area during migration | rejected: hides invalid source state |

## Evidence class and claim boundary

This selection is a schema and ownership correction derived from the already
admitted typed pool architecture and conservation/ownership invariants. It does
not introduce constitutive process physics, site values, calibration, runtime
activation, or consumer cutover.
