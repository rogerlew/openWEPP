# Independent Review A

Status: `COMPLETE`

Evidence class: `Static: complete-file and raw-ledger review; Ran: independent
checksum, extractor rebuild, hold validator, documentation, and diff checks`

Verdict: `HOLD`

The reviewer independently passed both checksum manifests, the exact extractor
rebuild and comparison (1,251 rows; SHA-256 `890a0f...1b61`), all manifest
identities, the hold validator, Markdown lint, diff hygiene, and write-set
inspection. The raw timing audit confirmed 932 Hubbard calibration rows over
1989–2024, 319 Harvard holdout rows over 1991–2023, and no 1992 fall holdout.

Scientific assessment:

- finite input-validity domains do not establish finite evidence-supported GSI
  search bounds;
- the upward/downward 0.5 crossing rules, equality convention, missing-crossing
  invalidation, and equal-year interval RMSE match admitted authority;
- the corrected partition-sum and mature-LAI roles match the CAL-03 ledger and
  do not separately identify the requested operands;
- Harvard remains sealed, with empty candidate, ensemble, and holdout ledgers;
- scientific `BLOCKED` gates are neither passed, waived, nor deferred and
  correctly force `EXECUTED / HOLD`.

One `MEDIUM` finding was accepted and corrected before verification:
`command-log.csv` contained descriptive or cwd-invalid replay commands.
CMD-004..007, CMD-009, and CMD-012 now contain exact repository-relative
commands, and the two validators are retained as package-local scripts.

No high or critical scientific finding remains.
