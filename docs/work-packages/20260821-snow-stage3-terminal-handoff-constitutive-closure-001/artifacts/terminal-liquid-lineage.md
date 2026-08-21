# Terminal-liquid lineage

Status: `PARTIAL INDEPENDENT RECONSTRUCTION / RECEIVER CONSUMPTION BLOCKED`.

`Static:` `terminal_parcels_from_event` reconstructs

```text
event.start_liquid + event.external_liquid + event.melt
  - event.refrozen - event.terminal_liquid
```

and checks the event's liquid closure residual before mapping to the declared
OFE/tile topology. It uses 273.15 K and zero relative specific enthalpy for
the typed parcel and does not read `DirectDayFrame` summaries.

`Static:` The attachment does not yet credit the parcel to the actual
persistent surface-liquid owner or retain an exact-once consumed marker in an
additive restart. No terminal-liquid output or conservation closure claim is
made.
