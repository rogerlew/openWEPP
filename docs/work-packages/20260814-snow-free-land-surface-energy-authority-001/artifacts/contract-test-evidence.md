# Contract Test Evidence

Status: `terminal hashes confirmed / focused and heavy gates PASS`.

All earlier focused runs remain historical evidence and were invalidated by
subsequent accepted review findings. The new frozen candidate provides:

- LSE definition `e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f`;
- V8 definition `622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b`;
- generator `1156fa88a6d7e4dd98f6dd70fe5b891f69e0b6825694179ac4d687a38907c859`;
- exact joint core `c9555b2dd02a5d6f11d71eb923fb60bc882e9638ec20eb79accc96cec9018be5`;
- vectors `7b6a303ae434ca6ad59c7082ebf486300214427d6abe20c36bfaa9b8cbdab91c`;
- coupled schema `02dfa522b7d070df9a7d3e904d4f538a7f734eb6c8315fcbf033b7628b28e07f`;
- diagnostics schema `41fb7909d073b4fdf4e59c9fa7da26b9a965ad916688b7867a56525d1bf1460c`;
- six strict schema instances validated before serialization;
- 22 exact mandatory scenarios, including coupled ground-albedo lower-boundary
  feedback and a frozen active-cap centered-perturbation probe;
- 76 semantically validated poisons and 11 complete-identity failure vectors;
- five physically constructed owner candidate bodies and independently
  reconstructed typed receipts;
- exact first soil-node reconstruction at `292.28354996106884 K` from its
  beginning temperature, ground-heat receipt, infiltration enthalpy, tile
  fraction, and areal heat capacity;
- 19 shared-source finalized uses and six ending-store source ledgers;
- a complete positive-condensation owner transaction;
- a routed 120-to-200 square-metre OFE crossing that preserves 72 kilograms
  and `5,952,940.00850654` joules extensively;
- exact five-owner-plus-transaction-envelope rollback.

Historical Ran: two controlled post-third-review Python regenerations were
byte-identical. The then-frozen bytes passed:

- `land_surface_energy_balance_authority_contract`: 7/7;
- `vegetation_boundary_authority_contract`: 26/26;
- package Markdown lint: 36 files, zero errors and zero warnings;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS.

Those historical results preceded the failed release reviews and did not
confirm the terminal hashes. Normal Rust tests consume committed vectors and
do not invoke Python. Ran on the terminal hashes: independent regeneration was
byte-identical at `7b6a303a...`; both terminal reviewers returned `PASS / GO`;
the LSE authority target passed 7/7; the final focused authority population,
strict workspace Clippy, 2,674-test full workspace run, doctest invocation,
dependency policy, Markdown, formatting, and diff-hygiene gates passed. Exact
commands, failed attempts, and retry history remain in `gate-results.md`.
