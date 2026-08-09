# Contract-Test Implementation Evidence

Status: `PASS / strict acquisition-schema assertions added`

Evidence mode: `Ran`

`tests/integration/vegetation_boundary_authority_contract.rs` now binds the
version-3 authority-level admission: immutable tuple/digest identity,
caller-supplied local bytes, network/mutable rejection, duplicate-key failure,
no hidden defaults, raw/resolved separation, typed initial-state identity,
non-averaged mixed strata, new invariants, and residual gap labels.

The focused suite passed `9/9` using a dedicated `/dev/shm` temporary directory.
The first post-edit run failed one new line-wrap-sensitive literal assertion;
the test assertion was corrected without weakening the contract obligation and
the identical focused command then passed. No fixture or asserted value was
added.
