# Child 2C authority surfaces

Status: queued

Evidence mode: not-run

This artifact will record the promoted canonical contract sections for the two
coupled surfaces. The package cannot close from this summary alone.

## Shared snow--canopy turbulent carrier

Required topology: sealed reference atmosphere -> one shared canopy-air node ->
both V11 canopy surfaces and the Stage 3 ground snow surface. The promoted
authority must bind sensible heat, vapor exchange, longwave, temperature,
humidity, wind/exposure, roughness, emissivity, albedo, and support ownership.

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

The promoted contract must define deterministic candidate selection, no-candidate
retry, zero-duration custody transition, successor admission, and owner
preservation on rejection.

## Active participant aggregation

```text
common_minimum_support = max(minimum support of every active physical participant)
```

The artifact must list participant sets before and after terminal snow events
and prove that the structural clock remains independent of constitutive support.
