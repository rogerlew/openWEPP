# Characterization

Static: dual eligibility review is PASS and the exact baseline is complete.

Characterization will use the declared split test module and the existing
normalized valid plan/receipt fixture. It must bind the public READY-audit path
in exact order: identity, live execution context, HEAVY admission, then full
receipt verification. It will also directly bind the remaining eligible floor
gaps without changing receipt schemas, errors, or production behavior.
