# Scheduler State-Point Decision

Evidence class: `Static`

The Child-2 shadow freezes the complete freshly seeded `DirectDayFrame` before
the first hydrology span. This is the only inspected state point that both:

- exposes the real persistent production owner without semantic reduction;
- precedes current rainfall, runon, infiltration, runoff and drainage;
- satisfies the V8 rule that same-interval ingress cannot fund ET; and
- permits root and later ground requests to share one immutable owner snapshot.

Authorizations are therefore derived from beginning layer `theta_m` only.
Current ingress remains part of the later hydrology transaction and cannot
alter or replenish the fixed authorization.
