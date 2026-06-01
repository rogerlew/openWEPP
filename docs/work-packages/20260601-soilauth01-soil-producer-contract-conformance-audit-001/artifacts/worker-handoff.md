# SOILAUTH01 Worker Handoff

Status: complete  
Evidence mode: Static

## Scope
Handoff target: `SOILAUTH02` remediation package.

## Immediate Next Actions
1. Resolve `SA01-M001` (policy-first ordering) with explicit canonical decision
   and aligned producer/parser tests.
2. Resolve `SA01-M002` by enforcing explicit `avke` emission in canonical
   producer paths for `7778/9002/9003/9005`.
3. Resolve `SA01-M003` by ratifying restrictive-row authority
   (profile-footer vs per-OFE semantics) and aligning contract + parser + fixtures.
4. Resolve `SA01-M004` by handling canonical double-quote fallback tokenization
   or prohibiting it with typed producer failure.
5. Recompute and record fixture provenance hashes for modified authoritative
   fixtures.

## Blocking Risks
- Contract ambiguity if canonical ordering/cardinality decisions are not made
  before implementation edits.
