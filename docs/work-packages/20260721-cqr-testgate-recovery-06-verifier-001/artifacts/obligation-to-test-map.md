# Obligation-to-Test Map

Static: verifier obligations are production/error-priority obligations rather
than process-physics vector families.

| Obligation | Binding test |
| --- | --- |
| Valid READY-admitted HEAVY receipt returns the downstream verdict unchanged | `ready_audit_verification_preserves_order_and_exact_verdict` |
| Receipt/plan identity rejects before transition checks | `ready_audit_verification_preserves_order_and_exact_verdict` |
| Live context mismatch preserves its exact error | `ready_audit_verification_preserves_order_and_exact_verdict` |
| Missing HEAVY work preserves audit-admission rejection | `ready_audit_verification_preserves_order_and_exact_verdict` |
| Valid admission continues into complete receipt verification | `ready_audit_verification_preserves_order_and_exact_verdict` |
| Retry, prerequisite, audit, artifact, and binding guard edges remain fail-closed | `local_verifier_guards_cover_retry_prerequisite_audit_and_binding_edges` |
| Receipt/envelope public identity and trust fields are exact | `verdict_accessors_preserve_identity_and_trust_fields` |

Ran: all mapped tests pass in the authoritative 138-test changed-head
measurement.
