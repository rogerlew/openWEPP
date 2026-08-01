# Conservation And Physical-Domain Evidence

Status: `PASS`

Evidence class: `Ran`

The package-local consumer streamed complete trace vectors and reconstructed
every accepted ledger independently of producer summary residuals.

| Reconstruction | Population maximum | Acceptance |
| --- | ---: | --- |
| daily snow mass | `1.3322676295501878e-15 m` | `<=1e-9 m`: PASS |
| surface energy | `2.998858690261841e-7 J m^-2` | `<=1e-6`: PASS |
| cold-content change | `9.209616109728813e-7 J m^-2` | `<=1e-6`: PASS |
| hourly latent roundoff allowance ratio | `0.0013459374978745057` | `<=1`: PASS |
| daily latent versus hourly flux | `3.4458935260772705e-8 J m^-2` | `<=1e-6`: PASS |
| daily shortwave versus hourly flux | `9.592622518539429e-8 J m^-2` | `<=1e-6`: PASS |
| daily longwave versus hourly flux | `9.872019290924072e-8 J m^-2` | `<=1e-6`: PASS |
| daily versus hourly vapor mass | `7.993605777301127e-15 kg m^-2` | `<=1e-9`: PASS |
| vapor mass versus sublimation debit | `8.109983287707401e-8 kg m^-2` | `<=1e-6`: PASS |
| layer-vector SWE versus runtime SWE | `5.079270337660091e-15 m` | `<=1e-9 m`: PASS |
| layer-vector depth versus runtime depth | `8.881784197001252e-16 m` | `<=1e-9 m`: PASS |
| WAT SWE versus trace/layers | `5.079270337660091e-15 m` | `<=1e-9 m`: PASS |
| WAT depth versus trace/layers | `8.881784197001252e-16 m` | `<=1e-9 m`: PASS |
| resolved-layer cold content | `1.862645149230957e-9 J m^-2` | `<=1e-6`: PASS |

Every complete eight-field layer vector is finite and physically signed. The
minimum populated layer temperature is `-38.55559106811294 deg C`; no layer is
at/below absolute zero or above melting. Four cells contain 30 represented
layer occurrences with SWE `>1e-12 m` and `<=1e-9 m`.

Anti-alias self-checks reject reversed latent sign, mismatched latent heat,
omitted unused energy, layer SWE/depth unit substitution, and deletion of a
represented subnanometer-SWE fragment.

The terminal independent verifier also requires finite WAT/trace operands,
exact layer-count/vector identity, density coupling, cold-content coupling, and
exact 12x4 selector behavior. See `retained-output-evidence.md`.
