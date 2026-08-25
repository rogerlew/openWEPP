# Terminal diagnostic correlation V4 review disposition

Disposition: **HOLD / NO IMPLEMENTATION INTENT / NO SOURCE AUTHORITY**

Both independent reviewers verified every frozen hash in
`terminal-diagnostic-correlation-v4-candidate-manifest.md`, made no frozen or
source edits, did not communicate with one another, and returned `HOLD`. The
two-GO gate failed.

## Accepted progress

The actual `framed_sha256` primitive is now correctly stated. Findings
`TDCV3-WIRE-007`, `TDCV3-OWN-008` and `TDCV3-OWN-009` are accepted. V4 retains
the accepted provider/coupling/selection/terminal ownership split, exact live
roles and pair positions, separate rejected-pair and pre-provider floor
decisions, cardinality, raw binary64 error convention, accepted-event receipt
exclusion and external LSE owner-file discovery. The package-local tool really
uses `syn`, records source blob/declaration fingerprints, and its generated
census reproduces byte-for-byte under the guard.

## Accepted blocking findings

1. The generator's replay class is a file-wide substring heuristic. A type is
   classified as native replay merely because another field/function in the
   same file mentions replay bytes. This falsely classifies
   `CoveredCarrierPhaseResultV1` as whole-record native replay.
2. The hard-coded target list is not a recursive type graph. Required nested
   identifiers, boxed typed-error payloads, envelope variants, candidate owner
   types, string-backed IDs and collection element/key types are absent. A
   guard can reproduce an incomplete census without detecting the omission.
3. Owner stage, native validator/digest and required private access are also
   name/text heuristics rather than AST-resolved functions and visibility/
   module paths. They are not sufficient authority for replay extraction.
4. `provider-owned projection of CoveredCarrierPhaseResultV1` is not an exact
   census type or a complete tagged field schema. The live result embeds a
   transition and ending candidates whose carrier-versus-later custody must be
   split by an explicit generated projection, not prose trimming.
5. Class-2 and class-3 adapters still lack recursively enumerated unique wires.
   Referring to a missing nested census entry cannot define its fields,
   variants, string/ID representation or native preimage.
6. Several new-record details remain non-unique, including exact key/member
   byte schemas, finiteness flag tags/cardinality, optional float encoding,
   `last_*` lexical discovery, final result bytes and boxed error adapters.
7. Because nested owners are missing, the exact owner-module/privacy/file
   boundary is incomplete despite the newly named external LSE modules.

These are authority defects, not source implementation choices. A successor
generator must recursively resolve the complete transitive type graph, fail on
unresolved types, associate digest/replay functions with the exact type rather
than its file, emit explicit generated carrier projections, and generate every
wire tag/order from a checked schema model. The guard must reject both missing
and extra reachable nodes.

## Retained hold

No exact-file implementation intent, correlation seam, receipt capture or
matrix is authorized. The physical result remains exactly
`Stage3(TerminalNumerics(BelowCarrierDomain))`; production Rust is unchanged
and `43cc9bbea2fbf5fe6ab6596cee4162de75cef999` remains the last qualified
physical implementation. Final v21 review, temporal operator, Batch V2,
event, receiver, restart, runner, Child 3 and cutover remain prohibited.
