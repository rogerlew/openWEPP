# Conservation And Reconstruction Evidence

Ran:

Across every successful daily trace row preceding the 24 rejected steps:

| Reconstruction | Maximum absolute residual | Acceptance |
| --- | ---: | --- |
| Snow mass ledger | `1.3322676295501878e-15 m` | `<= 1e-9 m`: PASS |
| `shortwave + longwave + latent - applied - unused` | `2.421438694000244e-7 J m^-2` | `<= 1e-6 J m^-2`: PASS |
| Hourly `latent_flux * 3600 - signed_mass * effective_latent_heat` | `3.3613953960317133e-10 J m^-2` | `<= 1e-6 J m^-2`: PASS |
| Producer-carried latent/mass residual | `1.3339729e-10 J m^-2` | `<= 1e-6 J m^-2`: PASS |
| Daily latent energy minus 24 hourly flux operands | `1.6298145055770874e-8 J m^-2` | `<= 1e-6 J m^-2`: PASS |
| Daily signed mass minus 24 hourly operands | `5.329070518200751e-15 kg m^-2` | `<= 1e-9 kg m^-2`: PASS |
| Daily signed mass plus sublimated ice debit | `6.880170683957326e-8 kg m^-2` | `<= 1e-6 kg m^-2` (`1e-9 m` SWE): PASS |
| Daily shortwave minus hourly reconstruction | `7.82310962677002e-8 J m^-2` | `<= 1e-6 J m^-2`: PASS |
| Daily longwave minus hourly reconstruction | `8.195638656616211e-8 J m^-2` | `<= 1e-6 J m^-2`: PASS |

The reconstruction is package-local and consumes published hourly operands
rather than relying on the producer’s precomputed residual. The signed-mass
surface is independently checked against both its hourly sum and the
sublimated-ice storage debit. Full per-run results and hashes are in
`diagnostic-replay.json`.

Passing pre-rejection ledgers do not admit the rejected state and do not prove
a corrective mechanism.
