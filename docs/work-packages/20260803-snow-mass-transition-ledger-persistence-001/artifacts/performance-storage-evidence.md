# Performance And Storage Evidence

Status: `PASS`

Evidence mode: `Ran + Static`

The frozen fixture manifest is
`16571c3b8c0d5fe692ae7ff1552cae78485368d19305275421a4ef2dd448c36e`.
The scaffold release binary is `11232920 B`, SHA
`464c87e16f24997753627d83399979b1f4bcc232629196c1d9847a7f9d0bb407`.
The exact remediated candidate is `11249152 B`, SHA
`4e0ebd96da7daa74c6a2c22dce200c87208997df9ac424a0e0b31de83b51da47`.
It was built from scaffold HEAD `3490ca153106` plus the terminal dirty package
diff. The tracked manifest records the exact changed-path set, while the
independent exact-current rebuild reproduces the candidate binary content
identity.

After one warm-up per binary, trace-disabled samples were:

| Metric | Scaffold samples | Candidate samples | Median ratio | Result |
|---|---|---|---:|---|
| wall seconds | `4.122740, 4.169536, 4.102785, 4.086175, 4.049310, 4.291565, 4.128319` | `3.951235, 3.976927, 3.882335, 3.924966, 4.084754, 4.004156, 4.041653` | `0.964632` | PASS; 3.54% lower |
| peak RSS KiB | `35360, 35356, 35360, 34808, 35356, 35168, 35360` | `36004, 36008, 35628, 35832, 36016, 36008, 36012` | `1.018441` | PASS; 1.84% higher, below 5% cap |

The scaffold `DirectSnowLiquidPartition` was `15816 B`; the candidate is
`656 B` (95.9% smaller). The immutable ledger/outcome bundle is `112 B`, and
`Option<Box<DirectSnowVerboseDiagnostics>>` is an `8 B` handle.

Allocation/copy inventory:

- `DirectSnowLiquidPartition` stores one inline `112 B` bundle;
- `DirectSnowCouplingInputs` owns one always-on `Box<...>` allocation per
  constructed day input to retain the constructor layout ceiling;
- production state and downstream operands each store one inline `Copy`
  bundle and never carry the verbose payload;
- the already-optional live-frame shadow is one boxed record per executed R4G
  span, with its bundle inline in that record, retaining the day-frame ceiling;
- trace-disabled execution stores `None` and constructs neither hourly verbose
  arrays nor the verbose box; selected trace execution creates one verbose box
  for the selected day/lane result.

The exact paired RSS measurement includes these allocations. The measured
1.84% RSS increase and 3.54% runtime decrease both pass the frozen 5% bounds;
no performance-improvement claim beyond this fixture is made.

The enabled trace is exactly `659499507 B` in both builds (ratio `1.0`) and
has identical SHA `84a64c1b4031584842c4d20023acac92fcffbea946fcce04953fb5a0a339fb5f`.
Raw receipts and the comparison are under
`target/snow_mass_transition_ledger_persistence/reports/`.
