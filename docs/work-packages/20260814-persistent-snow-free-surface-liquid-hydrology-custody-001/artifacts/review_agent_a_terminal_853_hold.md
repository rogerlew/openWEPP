# Rust Correctness Review At `85358c9b2`

Evidence class: `Static`

Verdict: `HOLD`

The fresh exact-byte Rust correctness review inspected commit
`85358c9b24d2ad74f34a1efc12295f147e393e84` from a clean worktree. It found
three accepted high-severity defects:

1. Receiver E003/E010/E011 failures could attribute an LSE or soil-thermal
   operand to the hydrology beginning-state digest, and one arithmetic path
   dropped the implicated owner entirely. Structural sealing repeated the
   owner/hash drift for missing or duplicate rollback rows.
2. The winter-domain validator accepted malformed frost containers, including
   nonintegral or cardinality-mismatched fine-layer counts, duplicate or
   unordered indices, undeclared membership and noncontiguous fine-layer
   indices. Those malformed inputs could fall through as E004 instead of the
   required earlier E003.
3. The unified public-entry E002 preflight covered only the outer transaction
   identity. Tile, OFE, surface, source and WB14 identities were checked after
   E003 request/winter validation and after the finalization callback, allowing
   E003 to mask E002 and permitting callback execution on invalid ingress.

The review requested owner-specific rollback provenance resolved from one
canonical `(OwnerKind, owner_id)` lookup, complete structural winter
validation and a complete input-only ingress identity preflight before every
E003 check or callback. No broad or terminal gate was run by the reviewer.
