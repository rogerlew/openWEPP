# Operand lineage

Status: `EXECUTED / TEST-ONLY`.

| Operand | Units | Lineage | Check |
|---|---|---|---|
| beginning pack/frost/liquid | `kg m^-2` | immutable trajectory beginning | finite/nonnegative; exact solid/liquid reconstruction |
| cold content | `J m^-2` | snow beginning/ending | change retained in the ledger for phase-energy reconstruction |
| signed vapor mass | `kg m^-2` | unsealed research forcing-segment DTO | split once into deposition/sublimation; overdraw typed |
| surface latent heat | `J kg^-1` | separate segment operand | finite/nonnegative and strictly positive for nonzero vapor |
| vapor latent energy | `J m^-2` | separate segment operand | reconstruct `m_v L_s`; mismatch poison |
| external liquid | `kg m^-2` | segment precipitation transfer | liquid credit exactly once |
| complete energy | `J m^-2` | segment carrier total | cold/fusion/unallocated identity |
| pack/frost tag | enum-like state | research snow envelope | canonical byte replay and schema poison |

Static: synthetic `L_s` is `ASSUMED_FOR_EXECUTION`, not calibration or new
physical authority. Ran: the real fixture separately checks its produced
mass/energy ratio against the canonical meteorology latent heat at 0 C.
