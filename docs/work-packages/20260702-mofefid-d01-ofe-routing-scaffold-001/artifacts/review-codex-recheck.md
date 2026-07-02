# Codex Re-check - MOFEFID-D01

Outcome: **accepted after ratification edit**.

## Evidence Classes

Static:

- Re-read `review-codex.md`, `review-disposition.md`, `validation-cases.json`,
  `source-manifest.md`, `package.md`, and ADR-0033 after commit `2ed801a9`.
- Inspected `d36d052a..2ed801a9` and verified the D3 Rust code was unchanged.
- Verified the remaining ADR scope now authorizes only representation,
  opt-in activation, and retention of shadow-first D3 kernels.

Ran:

- Re-extracted the local ignored `3.1_Validation_Input.docx` and confirmed
  Case 3 is `6.1 m x 1.8 m`; Cases 1/2/4 matched the checked operands.
- `sha256sum` for the local ignored `3.1_Validation_Input.docx` and
  `Figure_4.xlsx` matched `source-manifest.md`.
- `python3 -m json.tool` on `validation-cases.json`.

## Disposition

- `D01-CX-001`: **closed**. Case 3 now uses `[6.1, 1.8]`, adds the missing
  soil and `k_o`, and records the paper-body `3.6 m` discrepancy explicitly.
- `D01-CX-002`: **closed**. `source-manifest.md` records the ignored local
  source paths and hashes without vendoring copyrighted supplemental files.
- `D01-CX-003`: **closed after Codex ratification edit**. Commit-local edits
  changed ADR-0033 from Proposed to Accepted, added it to the ADR index, and
  removed the remaining active package sentence saying ratification authorized
  solver stages. D4/D5 remain gated on `SC-OFEROUTE-001` being authored and
  ratified first.

No production code or tests were modified during the re-check/ratification
edit. ADR-0033 is accepted only for the narrowed representation + opt-in
activation scope.
