# Rust and Ownership Review

Evidence class: `Static` plus reviewer-run delta diff hygiene.

Verdict: `PASS` on exact commit
`97d789bc7a6378a2ab85f7f42c3e055944500547`.

The public restored constructor cannot mint the opaque prepared capability
without exact GSI/configuration, cursor, ordered destination, 48-step WB14 and
scalar, receipt, and carry joins. Empty, reordered, wrong-static, and
stale-ending poisons preserve complete live-owner bytes. No material Rust,
serialization, capability, or ownership finding remains.
