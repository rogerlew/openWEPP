# Implementation Evidence

Status: implemented and focused validation passed

Evidence mode: **Ran + Static**

The shared runoff reconciliation now activates snow coupling when any typed
hour contains material snowfall, independently of prior SWE and the daily
mean-temperature shortcut. The existing prior-pack and cold-precipitation
activation routes remain intact, including warm all-rain/no-pack inactivity.

First-round review exposed an upstream control duplicate. Production SIMIMPL28
now resolves its unchanged typed hourly phase provider for every material
precipitation day, and direct publication removes daily mean temperature from
its pre-provider gate. Warm dry/no-pack days still bypass hourly work; warm
all-rain days resolve phase and remain inactive at the shared consumer.

Before returning the public partition, the implementation independently sums
typed hourly snowfall as water equivalent at the contract-bound fixed
`100 kg m^-3` new-snow density and reconstructs

`SWE_before + typed_snow + rain_retained - snowpack_loss - sublimation - SWE_after`.

Non-finite or absolute residuals above `1e-9 m` return the existing typed
runoff reconciliation error. The snowbench real consumer performs the same
independent daily reconstruction. Structured kernel failures retain their
typed source as `SnowKernel`; consumer-only reconstruction failures use the
distinct `SnowStorageClosure` variant.

Ran: the pre-implementation integration target reproduced the defect with one
activation failure. After correction, all five EB-04W vectors passed, and the
snowbench boundary tests passed including rejection above and acceptance at
the exact tolerance. Added tests also prove exact/just-over activation and
closure thresholds, non-finite rejection, and the real direct-production
warm-mean snowfall path.
