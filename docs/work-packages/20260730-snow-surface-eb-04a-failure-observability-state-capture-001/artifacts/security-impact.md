# Security Impact

Static:

- No secrets, network access, authentication, authorization, unsafe Rust,
  fixture mutation, or new external dependency.
- Diagnostic subprocess arguments and environment keys are explicit.
- Captured state contains simulation values only.

Disposition: `NO SECURITY IMPACT`.
