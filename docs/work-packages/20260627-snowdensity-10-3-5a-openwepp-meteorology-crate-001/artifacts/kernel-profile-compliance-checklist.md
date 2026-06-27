# Kernel Profile Compliance Checklist

Status: complete
Evidence mode: Static + Ran

| Item | Status | Evidence |
|---|---|---|
| Contract-first sequencing followed | PASS | Static: `SC-SNOWFREEZE-001` v91 and contract guard were added before crate implementation. |
| Typed domain errors used | PASS | Static: public functions return `Result<_, MeteorologyError>`; typed variants cover boundary failures, absolute-zero, non-positive values, invalid options, and non-convergence. |
| No silent defaults/clamps | PASS | Static: values are rejected through checked constructors; precipitation fractions are bounded by typed unit-interval wrappers; non-convergence returns an error. |
| No broad production `Box<dyn Error>` swallowing | PASS | Static: new production-free crate uses typed `MeteorologyError`; no `Box<dyn Error>` added. |
| No production `.unwrap()` / `.expect()` | PASS | Static: `.expect()` appears only in `#[cfg(test)]` test code. |
| Unit/quantity boundaries documented | PASS | Static: crate exposes typed wrappers and docs for `kPa`, `kg m^-3`, `J kg^-1`, `m^2 s^-1`, `W m^-1 K^-1`, Celsius, and fractions. |
| Candidate/default isolation preserved | PASS | Ran: no-production-wiring scan passed; root workspace member only, no production dependency edge. |
