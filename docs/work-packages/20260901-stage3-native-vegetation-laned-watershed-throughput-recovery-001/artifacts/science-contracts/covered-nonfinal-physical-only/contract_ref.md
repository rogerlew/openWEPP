# Covered nonfinal physical-only cross-contract cycle

Evidence mode: `Static + Ran`

Base commit under review:
`a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`.

The contract revision is an uncommitted authorized worktree increment. Its
scoped diff SHA-256 at review dispatch was
`152e25dd8293d11c26696194e2e163f6bb6a40656aadef9c347ee00ad0d9a19b`.
After accepting and applying every review finding, the expanded scoped
contract/test diff SHA-256 submitted for verification is
`5e1c303754aa7c4ef0ccab43560bebec867d779fad5113f6d10258a581ff440a`.
The first dual verification rejected incomplete behavioral assertions. After
accepting and applying those verification findings, the current repeat-
verification full-file manifest SHA-256 is
`ea628948b21522359c52a788a7e065f4f1d2428f8c2ff59fe1642bd282ee4b44`.
It is the SHA-256 of the ordered `sha256sum` rows for every canonical path
listed below, so it binds both tracked and untracked files. This replaces a
tracked-diff-only digest that omitted the untracked integration test.

Canonical paths under review:

- `docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`;
- `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`;
- `docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md`;
- `docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md`;
- `docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md`;
- `docs/specifications/science-contracts/index.md`;
- `tests/integration/stage3_native_vegetation_laned_throughput_recovery.rs`;
- `crates/openwepp-hillslope-orchestrator/src/snow_stage3_v11_adaptive_production_tests.rs`.

The authority preserves every charged canonical covered-map role and the
eight-map limit while separating private nonfinal physical endpoints from the
one independently charged final-complete owner envelope. Production code was
not changed for this amendment before its contract-derived expected red.

Ran: all five strict Binding Exposure Index checks, the scoped science-contract
unit-governance lint, and scoped `git diff --check` passed. Evidence:
`artifacts/logs/covered_nonfinal_contract_static_gates_20260903T0142Z.log`.

Ran: the exact contract-derived test selected one test and failed only because
`CanonicalCoveredIterationMapV1` is absent. Evidence:
`artifacts/logs/covered_nonfinal_physical_only_expected_red_20260903T013512Z.log`.

Ran after review amendments: the executable crate-level behavioral population
failed compilation only on the deliberately absent role-audit, failure-
injection, parity, and poison APIs. Evidence:
`artifacts/logs/covered_nonfinal_behavioral_expected_red_20260903T015519Z.log`.

Ran after review amendments: the supplemental anti-evasion source test passed
all contract/version/obligation checks and failed at the first absent private
production type. Evidence:
`artifacts/logs/covered_nonfinal_source_antievasion_expected_red_20260903T015716Z.log`.

Ran after first-verification amendments: all five strict BEI checks, scoped SC
unit lint, and `git diff --check` passed in
`artifacts/logs/covered_nonfinal_contract_reverification_static_gates_20260903T0205Z.log`.
The strengthened behavioral population remains expected red only on the absent
implementation APIs/types in
`artifacts/logs/covered_nonfinal_behavioral_expected_red_reverification_20260903T0211Z.log`.
The strengthened source guard remains expected red at the absent private
iteration-map type in
`artifacts/logs/covered_nonfinal_source_antievasion_expected_red_reverification_20260903T0207Z.log`.
