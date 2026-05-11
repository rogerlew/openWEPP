# Interface Contracts

Pinned cross-component contracts. Changes require coordinated updates across producers and consumers.

| Contract | Authoritative source | Notes |
|---|---|---|
| `.run` formalized model-flag schema | (to be authored) | Replaces legacy stdin-driven `.run` plus sidecar `.txt` files (e.g. `frost.txt`, `wepp_ui.txt`). All flags and sidecar input files must be explicit in `.run`. Schema language and exact field set are not yet decided. |
| HBP (hillslope binary pass) | wepp-palimpsest `docs/contracts/hillslope-binary-pass-format.md` | openWEPP consumes and produces HBP shards per the upstream specification. Magic, header, day directory, and footer must match. |
| Parquet hillslope-trajectory schema | wepppy / wepppyo3 interchange | openWEPP emits via the existing consumer-side schema; no new schema authoring on this side until coordinated evolution is needed. |
| openWEPP runner boundary | [openwepp-runner-contract.md](openwepp-runner-contract.md) | openWEPP owns `openwepp_runner`; engine selection is explicit (`legacy_wepp` vs `openwepp`); no silent fallback across engines/contracts. |
| openWEPP binary release + sidecar | [openwepp-binary-release-contract.md](openwepp-binary-release-contract.md) | `openwepp_YYMMDD*` naming, mandatory sidecars, schema validation, and blocking release lint gate. |
| WEPP soil file format | legacy WEPP / wepp-palimpsest | openWEPP parses; format pinned to existing producer compatibility. |
| WEPP management file format | legacy WEPP / wepp-palimpsest | Same. |
| WEPP climate (cligen) file format | legacy WEPP / wepp-palimpsest | Same. |
| Watershed structure file format | legacy WEPP / wepp-palimpsest | Same. |
| `.json` release manifest | wepp-palimpsest release tooling | openWEPP binaries ship with manifest sidecars per the upstream convention. |

## .run formalization (planned)

Today the legacy WEPP `.run` is a multi-line stdin recipe paired with sidecar `.txt` files scattered through the run directory (`frost.txt`, `wepp_ui.txt`, and similar). openWEPP replaces this with a declarative, schema-versioned `.run` that:

- enumerates every model flag explicitly (no implicit sidecar `.txt` reads)
- references every sidecar input file by path
- carries a schema version so consumers can detect upgrades

Backward compatibility with legacy stdin `.run` is not a goal.

## Parquet schema source-of-truth

Schemas are owned by wepppy / wepppyo3. openWEPP imports schema definitions or generates conforming output from a co-located schema crate (decision deferred). Schema evolution is coordinated through the wepppy repo.

## Failure posture

Contract mismatch is a hard error:

- unsupported/missing engine selector for dual-engine boundaries,
- missing or invalid release sidecar,
- invalid binary name relative to release policy,
- mixed release-pair capability declarations.
