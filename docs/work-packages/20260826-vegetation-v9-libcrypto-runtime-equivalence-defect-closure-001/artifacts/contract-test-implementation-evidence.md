# Contract-test implementation evidence

Status: complete.

Static: the V9 integration test now requires the canonical v29 binding IDs and
authority text, invokes the new active verifier without an overlay, retains the
four protected-byte before/after checks, and adds a four-case poison population
for wrong SHA-256 capability, mapped-provider identity mismatch, second runtime
mismatch, changed protected bytes, and changed output. Each of the five poisons
uses the complete verifier admission path and requires its intended rejection.
