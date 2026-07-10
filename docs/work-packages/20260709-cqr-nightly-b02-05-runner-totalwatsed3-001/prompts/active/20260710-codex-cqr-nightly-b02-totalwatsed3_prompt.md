# CQR Nightly Batch 02 Target 05 Kickoff

Scope: local behavior-preserving CQR work inside `/home/workdir/openWEPP`; no
external connectivity. Execution mode: package-end-to-end.

Read root/crate/work-package/science-contract guidance, `SC-SYSTEM-001`, CQR
ExecPlan/guides, ADR-0021, this package, target CLI, and
`totalwatsed3_cli_contract.rs`. Required-reading map:
`artifacts/required-reading-map.md`; budget `~80 KiB`, `OK`.

Close CRAP only through private structural extraction. Preserve CLI grammar,
required/optional file discovery, source-relative paths, errors, output row
identity, units, and fail-closed behavior. Use delegated runners for heavy
metrics and workspace gates; bounded review/verification delegation is
authorized. Autonomy: execute through disposition unless hard-blocked.
