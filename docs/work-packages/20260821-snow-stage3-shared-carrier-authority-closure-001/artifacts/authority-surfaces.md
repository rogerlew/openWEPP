# Child 2C authority surfaces

Status: complete / terminal dual verification PASS

Evidence mode: Static + Ran

Canonical authority is currently in review at `SC-COUPLEDTIME-001@3`,
`SC-LANDSURFACEENERGY-001@7`, `SC-SNOWENERGY-001@14`,
`SC-VEGETATION-001@26`, and `SC-VEGETATIONTRANSACTION-001@15`. The package
closed with two independent reviews and two terminal verifications passed.

## Shared snow--canopy turbulent carrier

Required topology: sealed reference atmosphere -> one shared canopy-air node ->
both V11 canopy surfaces and the Stage 3 ground snow surface. The canonical
equations are the shared-node residuals `R_T=H_ref+sum(H_i)=0` and
`R_q=V_ref+sum(V_i)=0`, with surface-to-node bulk transfer terms and
equal/opposite owner-facing fluxes. Longwave uses the existing reciprocal V11
rank recurrence plus `L_can=sum_j(w_j*sigma*T_can,j^4)`,
`L_snow,down=f_sky*L_atm+(1-f_sky)*L_can`, and
`L_snow,net=L_snow,down-sigma*T_s^4`.

Rejected alternatives: raw 10 m wind as direct subcanopy wind, fixed attenuation
multiplier, independent canopy-air nodes, duplicate flux, post-event snow flux,
and canopy-intercepted snow.

## Event-boundary coalescing

Required receipt fields: `proposed_event_tick`, `accepted_event_tick`, parent
support, pre/post active-adopter support, candidate list or digest, tie-break
decision, event-time error, snow-mass error, liquid-mass error, energy error,
and rollback/retry identity.

For `[a,b)`, event tick `t`, and minimum supports `dt_pre`, `dt_post`:

```text
t-a == 0 or t-a >= dt_pre
b-t == 0 or b-t >= dt_post
```

The promoted contract defines deterministic candidate selection by displacement,
normalized mass/energy error, then tick; no-candidate retry;
zero-duration custody transition; successor admission; and owner preservation
on rejection.

## Active participant aggregation

```text
common_minimum_support = max(minimum support of every active physical participant)
```

The artifact lists participant sets before and after terminal snow events and
the independent restart/rollback vectors prove that the structural clock
remains independent of constitutive support. Runtime consumer activation remains
outside this package.
