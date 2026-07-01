# Implementation Blocker

Evidence class: Static code reading.

## Blocker

`hydrology_phase_runoff_reconciliation.rs` cannot be honestly migrated as a
small runoff-only typed result because its output is not only runoff:

- regular state and flux updates through `KernelWritebackPayload`;
- indexed state and flux updates through `IndexedKernelWritebackPayload`;
- snow state, snow hourly state, same-day snow publication fluxes;
- frost scalar state, layer state, fine-layer state, and hourly frost state;
- irrigation event state;
- MOFE hourly saturation and lateral carry arrays;
- runoff reconciliation state and closure diagnostics.

Any first pass that returns a typed struct but still obtains all values by
holding `&HillslopeKernelRequest` or immediately converts that struct back to a
payload as the only consumer would be a false typed boundary. It would move code
around without changing the authority surface.

## Required Pre-Cut Implementation Shape

The next implementation package needs a typed mutation vocabulary before
cutover:

- typed scalar state/flux mutation entries for the runoff family;
- typed snow hourly mutation groups;
- typed frost scalar/layer/fine/hourly mutation groups;
- typed irrigation mutation group;
- typed MOFE hourly carry mutation group;
- typed indexed-writeback adapter that consumes the typed mutation groups only
  while scheduler support remains compiled.

Only after that vocabulary exists can the phase extract owned typed inputs from
the compatibility edge and run a typed core without carrier refs.
