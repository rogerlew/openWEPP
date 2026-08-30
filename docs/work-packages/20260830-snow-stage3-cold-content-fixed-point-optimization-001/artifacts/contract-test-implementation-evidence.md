# Contract-test implementation evidence

Status: `PASS`

Evidence mode: `Static + Ran`

Three contract-derived vectors cover the new invariant:

- support-scaled finalization contraction, valid fingerprint/domain and
  cumulative closure, plus raw behavior at the exact floor;
- refusal across a discrete terminal-event-model change with raw authentic
  candidate fallback;
- exactly one stabilization crossing when relaxation is enabled, with normal
  convergence behavior otherwise.

Ran: the covered convergence policy module passed 19/19 under nextest (run ID
`60d8d8ad-2e9a-4927-99a7-36e7e03e72a8`). The terminal affected-contract set
passed 47/47 across seven integration binaries (run ID
`952cfcd5-c43f-4923-a4d6-d03908e666d5`).
