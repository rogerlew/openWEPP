# Assurance drift census

Status: `COMPLETE — TYPED SOURCE ADOPTION PASSED`.

`Static:` Initial generation was `90313e7b476cb5366605a1a708c29b5c2eeb68ecac36f90b00b9160b882c4fd8`.
The planner selected the DRAFT report
`snow-and-frozen-soil-process-evaluation`; both changed contracts are declared
`local_content` dependencies of that report. The affected source paths were:

```text
docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md
docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md
```

`Ran:` the first direct `adopt-report-source --check` in the live tree
refused with `generated identity member changed` for the other stale contract.
This exposed a sequencing limitation in the operation, not a source-class
refusal. A disposable source-complete workspace was used to execute the
admitted typed operation sequentially, with no generated hash hand-edit:

1. `adopt-report-source` SnowFreeze `--check` then `--apply`, generation
   `90313e...` → `7b4e36...`, receipt
   `2bc79a057f41ae28fcf68f30989ed194e6b378504d8e01720a3a54cccfcd5271.json`.
2. `adopt-report-source` SnowEnergy `--check` then `--apply`, generation
   `7b4e36...` → `41b142902d22e139ea732288ed40a504931a1fb54ab27c891d56891910229dd3`,
   receipt
   `74ede41f6e091c0825f7ec7cfd8d207bde466d85d71d4196fb20d9cdb67f8533.json`.

`Ran:`

```text
nix develop --command cargo run --quiet -p openwepp-assurance -- \
  verify-generation --base-ref 15763d7f6d5d4125333d9b7583424c714f5f5ea4
```

Result: `PASS`, current generation `41b142902d22e139ea732288ed40a504931a1fb54ab27c891d56891910229dd3`,
`83` verified transitions. Generated identity, affected review locks, report
manifest, and both typed transaction receipts are retained in the repository.
No assurance source or generated hash was manually edited.
