# CQR30 Behavior Equivalence

Static: refactor preserves the EROD13 public entry point
`Wb11HydrologyKernel::run_erod13_wave1_core` and only extracts private helpers.

Static: preserved formulas and expression order:

- `tau_f = taufe * (fs / ft)`
- `eta = (cntlen * kr * kradjf * shrsol) / tcend`
- `taucn = (tcadjf * shcrit) / shrsol`
- `theta = ((cntlen * detinr) / tcend) * (effdrr / effdrn)`
- `phi = (beta * veleff) / pkro`
- `tc = tcadjf * tc_k * tau_f.powf(tc_m)`
- detachment branch before deposition branch
- deposition branch positive-`q` recheck remains local to deposition

Static: final writeback order remains `Dc`, `Tc`, `Df`, `eta`, `taucn`,
`theta`, `phi`.

Ran: `cargo test --test erod13_wave1_core_kernel_contract`

Result: `7` passed, `0` failed.

Ran: full after-LCOV workspace test pass completed successfully while writing
`lcov_after.info`; this included the EROD13 and EROD14 integration suites.

Status: no behavior-equivalence drift found.
