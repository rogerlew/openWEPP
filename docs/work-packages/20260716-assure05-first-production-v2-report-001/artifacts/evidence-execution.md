# ASSURE-05 Evidence Execution

Status: COMPLETE

Evidence class: Ran

## Focused Confirmation

Command:

```bash
cargo nextest run --workspace --profile quick \
  -E 'test(/gwbaseflow|r6a_direct_hbp_writer_serializes_groundwater_payload_operands/)'
```

- exit: `0`
- Nextest run ID: `60f43595-861c-471b-bda7-1ca6860bf6ad`
- outcome: 7 run, 7 passed, 0 failed, 1,988 skipped
- Nextest time: 0.021 s; wrapper time: 1.162 s
- retained log: `/tmp/openwepp-assure05-heavy-focused.log`

The exact test names are retained in
`assurance/v2/reports/linear-groundwater-reservoir-recurrence/evidence/assure05-production-evidence.json`.

## Rejected H2637 Acquisition

The first H2637 run, Nextest ID
`61137899-246a-44f3-9a19-9fc503b57037`, passed its test assertion but is not
accepted as report evidence. Its scratch suffix was `_412811`. The actual debug
test sidecar named source commit `ec396c458a5015c504011a75814ff13e274544a1`,
not frozen commit `01ed70550a4e371e99afe35c4bdd4d9b667e812c`. The regular
release sidecar also recorded `unknown` source and a binary hash that differed
from the actual executable. Test success did not cure the provenance failure.

## Accepted Clean-Build H2637 Acquisition

The runner and test were rebuilt under isolated target root
`/tmp/openwepp-assure05-clean-target`.

```bash
CARGO_TARGET_DIR=/tmp/openwepp-assure05-clean-target \
  cargo build --release -p openwepp-runner --bins
CARGO_TARGET_DIR=/tmp/openwepp-assure05-clean-target \
  cargo nextest run --test laned_shadow_h2637 --run-ignored ignored-only \
  -E 'test(=h2637_native_active_owner_routes_and_closes)'
```

- release build: exit `0`, 105.395 s
- Nextest run ID: `01415f5a-1a16-48c2-a845-04a33a602f53`
- outcome: 1 passed, 0 failed, 9 skipped, 1 slow
- test execution: 445.192 s; command including clean debug build: 483.006 s
- off/default/on scratch roots:
  `/tmp/laned_shadow_h2637_active_off_473038`,
  `/tmp/laned_shadow_h2637_active_default_473038`, and
  `/tmp/laned_shadow_h2637_active_on_473038`
- default and explicit-active HBP and pass-Parquet bytes: identical

| Object | Bytes | SHA-256 |
| --- | ---: | --- |
| release `openwepp-cli-hill` | 10,732,704 | `7ed022035a38cc9e74b544e0e9e3033f24beded009ec48f6a7c384608382468b` |
| debug H2637 test executable | 174,739,496 | `32285cf8fae111dbddb19606afe6fbf8ac2d14eef552171ecdbe0d302fa5993e` |
| debug executable sidecar | — | `b0ceb401587f0106052059bafd01f5705b93e2357a2841cd6dfa8768751fada6` |
| explicit-active `manifest.json` | 9,470 | `756e324e5b4f055ea45c33b0d5f679ab2fc9f4b958e853dc0b70f17aeb592208` |
| explicit-active `H2637.hbp` | 5,742 | `378a8c1d80a22c9452fb256cf9a95eab09035f3a6cd387c6d626ab26c426c453` |
| explicit-active `H2637.pass.parquet` | 26,920 | `915f3b99c2ff20e3e0632b4e90a6ceb1cb8e7fee58f0d3e29b41de10c540f550` |

The sidecar's source commit is the frozen commit and its recorded executable
hash matches the actual debug test binary. The produced manifest also names the
frozen commit. The independent report procedure verified the HBP and Parquet
hashes against the manifest before reconstructing either ledger.

## Freeze Confirmation

After acquisition, Git HEAD remained `01ed7055`; the SC-GWBASEFLOW digest,
H2637 fixture stream, and all paths in `realization-freeze.md` remained
unchanged. No rejected run contributes a claim-bearing result.
