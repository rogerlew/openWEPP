# CQR30 Kernel Profile Compliance

Static: kernel-affecting work used the science-contract local playbook and
read the EROD13 authorities in `SC-SED-001`, `SC-HYDRAULICS-001`,
`SC-RUNOFFPART-001`, `SC-WATBAL-001`, and `SC-SYSTEM-001`.

Static: no public API, parser compatibility, alias, unit, runtime publication,
or writeback symbol was changed.

Static: typed EROD13 guard family remains:

- `HKERNEL-EROD13-CORE-E-001`
- `HKERNEL-EROD13-CORE-E-002`
- `HKERNEL-EROD13-CORE-E-003`

Static: the only lint suppression left in the target file is a narrow
`#[allow(clippy::similar_names)]` on `erod13_process_inputs` for contract
variable names `effdrr` and `effdrn`. The previous target-level
`clippy::too_many_lines` suppression was removed.

Ran: `cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings`

Result: passed.

Status: kernel profile compliance satisfied for this behavior-preserving
decomposition.
