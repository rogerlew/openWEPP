# Validation

Evidence class: `Ran`

Focused repair loop:

- `cargo fmt --check` — PASS.
- `cargo clippy --test testgate_ci_executor_contract -- -D warnings` — PASS in
  2.39 seconds after a build-directory lock wait.
- `cargo nextest run --test testgate_ci_executor_contract` — PASS, 2/2 tests,
  0 skipped, 0.030 seconds.
- `git diff --check` — PASS.

No broad command was run manually. The exact mechanical terminal plan remains
the authority for workspace-level obligations.

Mechanical terminal plan:

- risk: `BOUNDED_COMPONENT`;
- nodes: 10; inventory: 1,513;
- PASS: authority admission, format, package Clippy, docs lint, groundwater and
  snow-phase hard invariants, placeholder scan, workspace Clippy, and doctest;
- FAIL: affected CRAP, due only to Unix socket `SUN_LEN` in
  `assurance_dossier_build_contract`; and
- receipts:
  `601158fb9c26446fec5af28cb8663a55a7de8223197cf9eea428dbd2eb2d6e37`
  and
  `49ecc4a97cfa2e67925140067b9999b8d808a4aeda442b1ef3899644408f0c27`,
  both `LOCAL_UNTRUSTED` / `FAIL`.

The short-root retry progressed from two socket failures to one, proving outer
path length contributes but does not eliminate the fixture's own oversized
label. No receipt is relabeled or stitched into PASS.
