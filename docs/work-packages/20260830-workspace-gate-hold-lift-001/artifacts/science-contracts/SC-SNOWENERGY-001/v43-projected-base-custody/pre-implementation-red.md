# V43 projected-base custody pre-implementation red

Status: `EXPECTED_RED`

Evidence mode: `Ran`

Retained r113 (`/tmp/wghl_001d_v42_64m_r113.log`, SHA-256
`9c1d35d0f34991bec6386cbef9b6ca1295f6ca2e281e9774ba4b6bade5df3188`)
localized the first charged `1860..1920 s` refusal to
`carrier-final-envelope`: the V38 numerical-coordinate sibling was passed to
`DirectSoilThermalUnpublishedContinuationResultV2::try_from_base_unpublished_trial`.
Identity and support passed. The first unequal sealed field was predecessor
custody: reconstructed `AcceptedReceiptChain` versus retained
`NumericalCoordinateProjection`.

Ran:

```text
nix develop -c cargo nextest run \
  --test snow_terminal_enthalpy_event_numerics_contract \
  -E 'test(/v43_/)'
```

Nextest run `b5f36647-e2bb-48c1-9002-2fafc9233fd9` executed two tests: the
version-43 canonical contract binding passed and the source/behavior binding
failed only because the typed projected fixed-point posture and its five
required behaviors were not yet implemented. This is the expected
contract-first red. No production source changed before this run.
