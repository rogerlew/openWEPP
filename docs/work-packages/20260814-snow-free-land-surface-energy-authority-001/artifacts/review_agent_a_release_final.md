# Final Release Land-Surface Science And Numerics Review

Evidence class: `Static + Ran`; fresh independent review of the exact current
worktree. Prior failed reviews were read as historical evidence, not reused as
acceptance evidence.

Verdict: **NO-GO / FAIL**.

## Exact reviewed bytes

- LSE definition:
  `2ee9a7b87c8d22d270900a09312629a0f799475e45c41d0927d7ce18d3679915`;
- joint canopy-ground core:
  `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`;
- top-level generator:
  `f28d6105077f89a9bfc29b09ee416ce5cd699900d3674842ef4f68ace4f71f31`;
- committed vectors:
  `3c249fc201896db27b3cdba3fe468c241934e949122060230866b8074486391b`;
- `SC-LANDSURFACEENERGY-001.md`:
  `adf1bcd3b95a2e20f55a4f7a449426f31ed14b0595ba79db6d6d4a374b8cee20`;
- `SC-VEGETATION-001.md`:
  `1d7ec3699085fdf5d2f29e01b3c1d76b8a2a5ad8ce22340df2e066cb39f1fb1a`;
- `SC-VEGETATIONTRANSACTION-001.md`:
  `44b77e5d9854ce9c4b64214f2a669e1b09e2fdc5bdb6d1391ef25a909f186e73`.

The V8 definition remains
`622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`.
All six schema hashes match the immutable LSE definition and fixture manifest.

## Commands run

Ran:

```text
.venv/bin/python .../reference_calculator.py \
  --write /tmp/lse-child1-release-final-USTYIK/vectors.json
sha256sum /tmp/lse-child1-release-final-USTYIK/vectors.json \
  .../openwepp_snow_free_lse_v1_vectors.json
cmp -s /tmp/lse-child1-release-final-USTYIK/vectors.json \
  .../openwepp_snow_free_lse_v1_vectors.json
.venv/bin/python <independent Draft 2020-12 validation of every failure DTO>
cargo nextest run --test land_surface_energy_balance_authority_contract \
  --profile quick
```

The one independent regeneration produced SHA-256 `3c249fc2...` and compared
byte-identically with the committed fixture. All eleven failure diagnostic
objects independently pass `lse_v1_diagnostics_schema.json`; each has six
unchanged vegetation/hydrology/LSE/soil-thermal/BGC/envelope rollback hashes.
The focused Rust gate passed 7/7.

## Confirmed release-review corrections

`A4-HIGH-001` is corrected. The exact review probe freezes the two-rank
ground authorization at
`0.00015581562596770875 kg m^-2 tile-ground s^-1`. Under the reproduced
negative centered temperature perturbation, the unfrozen constitutive law is
`0.00015581538846473828`, while both the frozen multirank surface residual and
the frozen one-rank residual retain the authorization-active branch and the
bit-identical fixed cap value. `_raw_residual()` fixes both branch and value;
the covered-column reconstruction passes the same frozen map into
`first_full`. Centered differences and line search therefore use the admitted
zero generalized derivative at this equality cap.

The structural portion of `A4-HIGH-002` is also corrected. Every numerical and
domain failure has a complete schema-valid diagnostic DTO, null candidate,
attempted transaction identity, ordered evidence where a solve began, and
exact rollback. The broader executed matrix continues to support the selected
shortwave/reciprocal-longwave network, open and covered neutral turbulent
paths, signed evaporation/condensation, bare-soil and forest-litter surfaces,
surface/soil enthalpy, Crank--Nicolson ground heat, liquid advection,
beginning-snapshot D/A/F, post-solve ingress, active caps, failure trajectories,
and semantic poison rejection. The earlier `A`, `A2`, and `A3` scientific
mechanism findings are corrected in the calculator and canonical contract.

## Material findings

### `A5-HIGH-001` -- Failure codes contradict their canonical error families

The schema-valid diagnostic records pair failure kinds with error codes that
mean different processes in the canonical contract. The generator maps:

```text
singular           -> LSEB-E-038 / singular_pivot
backtracking_limit -> LSEB-E-039 / backtracking_limit
iteration_limit    -> LSEB-E-040 / iteration_limit
```

at `reference_calculator.py:1842-1845`. But
`SC-LANDSURFACEENERGY-001.md:824-835` assigns:

```text
LSEB-E-034 = numerical convergence
LSEB-E-038 = current-ingress ordering / same-interval availability
LSEB-E-039 = condensation mass/energy credit
LSEB-E-040 = soil-thermal owner/state/enthalpy mismatch
```

The same semantic mismatch affects all eight domain failures: calm wind,
frozen soil, missing ingress enthalpy, multiple surface classes, nonneutral,
snow present, snow terminal, and thawing soil all use `LSEB-E-035`, although
the contract assigns `LSEB-E-030` to unsupported domain and `LSEB-E-035` to
component/control-volume closure.

`lse_v1_diagnostics_schema.json` enumerates error codes and failure kinds
independently and has no conditional pairing rule, so Draft 2020-12 validation
cannot detect these contradictions. This is not merely a label preference:
runtime error precedence and owner disposition would classify a nonlinear
failure as ingress, condensation, or soil-thermal failure, while true domain
rejections would be reported as closure failures. The exact correction is to
emit the canonical code for each failure family and bind allowed code/kind
pairs in contract-derived evidence; schema validity alone is insufficient.

This keeps the semantic typed-diagnostic portion of `A4-HIGH-002`,
`A3-HIGH-004`, `A2-HIGH-003`, and `A-CRITICAL-007` open.

## Finding reassessment

| Finding family | Final assessment |
|---|---|
| `A4-HIGH-001`, active-set portion of `A3-HIGH-003` / `A2-CRITICAL-001` | corrected by exact frozen branch-and-value probe |
| `A4-HIGH-002`, structural DTO/rollback requirement | corrected: all eleven records are schema-valid and rollback-complete |
| `A-CRITICAL-001..006`, `A-HIGH-008..011`, `A-MEDIUM-012`, `A2-CRITICAL-002`, `A2-HIGH-004..005`, `A3-CRITICAL-001..002` | corrected in canonical authority and executed evidence |
| semantic diagnostic portion of `A-CRITICAL-007`, `A2-HIGH-003`, `A3-HIGH-004`, `A4-HIGH-002` | open under `A5-HIGH-001` |

This finding is a material, accepted-scope release defect. It is not a reason
for a new authority package or model identity: the canonical equations already
select the correct behavior. Child 1 is not eligible for
`COMPLETE / snow-free land-surface-energy implementation authority released`
until the generator/schema evidence is corrected, invalidated digests/vectors/
tests are regenerated, and a fresh independent review passes the new exact
bytes.
