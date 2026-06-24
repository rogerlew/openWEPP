# Worker Handoff

Status: queued.

This package is the successor to R7H opt-in closure for frost-depth fidelity.
It must acquire/normalize historic frost-depth observations and build the
comparison harness before any frost-model remediation.

Do not:

- resume direct-vs-compatibility frost bit-parity;
- default-activate direct runtime;
- tune frost physics inside the harness package;
- call observation disagreement an openWEPP defect without
  `INV-SNOWFREEZE-047` and ADR-0017 criteria.

First execution step:

1. Read `package.md` and `artifacts/required-reading.md`.
2. Fill `artifacts/dataset-inventory.md` with current source access,
   licensing, and storage policy.
3. Decide the normalized corpus/checksum format before writing fetch code.
