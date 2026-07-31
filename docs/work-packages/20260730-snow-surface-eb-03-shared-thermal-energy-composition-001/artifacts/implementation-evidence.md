# Implementation Evidence

Status: `complete`

Evidence mode: `Static + Ran`

Static: typed meteorology helpers implement the Dilley-Unsworth atmospheric
longwave estimator, contract-bound cloud mapping, effective-cover sky view,
additive canopy emission, outgoing snow emission, SNOBAL ice saturation, and
temperature-dependent sublimation latent heat.

Static: the hillslope kernel composes absorbed shortwave, optional longwave,
and optional latent flux hourly against one Stage 3 top-layer temperature and
cold-content state. Sublimation removes ice from that same layer, exports its
proportional cold content, promotes an exhausted surface layer, reconstructs
aggregate state, and never enters routed liquid. The existing CoE melt boundary
is preserved when the new sublimation selector is disabled.

Static: the direct runner parses independent default-off selectors and carries
daily solar/extraterrestrial radiation plus daylight into the real Stage 3
consumer. Unknown values fail closed.

Ran: focused EB-03 tests, the 21-test meteorology crate, and the 412-test
hillslope-orchestrator crate passed during implementation review. The real
consumer evidence is recorded separately.
