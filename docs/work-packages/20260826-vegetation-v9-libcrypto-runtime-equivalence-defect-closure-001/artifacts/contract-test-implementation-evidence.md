# Contract-test implementation evidence

Status: complete.

Static: the V9 integration test now requires the canonical v29 binding IDs and
authority text, invokes the new active verifier without an overlay, retains the
four protected-byte before/after checks, and adds a five-case poison population
for wrong SHA-256 capability, mapped-provider identity mismatch, second runtime
mismatch, changed protected bytes, and changed output. Each of the five poisons
uses the complete verifier admission path and requires its intended rejection.

The first full-workspace candidate exposed one stale v28 vegetation registry
note in `snow_stage3_terminal_receiver_authority_contract`; that lifecycle-only
assertion now binds the canonical v29 note. All terminal-receiver contract and
behavior assertions remain unchanged.
