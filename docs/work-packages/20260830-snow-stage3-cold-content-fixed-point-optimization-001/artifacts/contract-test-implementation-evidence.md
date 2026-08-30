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

Review correction extends these vectors with bitwise authentic-candidate
density at finalization and a stateful set/retain/consume seam proving that
intervening nonconvergence retains the pending crossing, the first
otherwise-converged relaxed revisit is consumed once, the next revisit may
finalize, and a raw exact-floor restart creates no pending crossing.

Ran: the covered convergence policy module passed 19/19 under nextest (run ID
`60d8d8ad-2e9a-4927-99a7-36e7e03e72a8`). The terminal affected-contract set
passed 47/47 across seven integration binaries (run ID
`952cfcd5-c43f-4923-a4d6-d03908e666d5`).

Review-correction reruns pass 19/19 (run ID
`45648dea-a2a7-4272-b5c5-81b0e2764cee`) and 47/47 (run ID
`ea732fe3-013a-4ca1-b1cb-3914d5a013ea`).
