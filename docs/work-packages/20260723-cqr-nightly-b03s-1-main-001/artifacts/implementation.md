# Implementation

Static: behavior-preserving whole-guard extraction reduced the three target functions without changing public CLI or library surfaces.

- `validate_package_chain_command` delegates ordered `base`, `head`, and `package` parsing to `package_chain_command_inputs`. Validation, persistence, and response JSON remain in their original order.
- `plan_request` delegates the existing authorized-path/source/authority sequence to `planning_context` and authority string extraction to `package_authority_fields`. Stage parsing still occurs after package reconstruction and before authority-field conversion.
- `package_authority` delegates file loading and independent reconstruction to `read_package_authority` and `reconstruct_package_authority`. Intent-package parsing, committed-head rejection, reconstruction, and exact-authority comparison retain their original order.

Static: no clone or allocation was introduced on a numeric path; this module contains control/identity data rather than floating-point computation.
