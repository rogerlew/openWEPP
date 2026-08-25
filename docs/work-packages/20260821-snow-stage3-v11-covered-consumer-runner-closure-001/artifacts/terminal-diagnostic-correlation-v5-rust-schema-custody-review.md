# Terminal diagnostic correlation V5 Rust/schema/custody/privacy review

Recommendation: **HOLD / NO GO-to-evidence**

Evidence class: `Static`. Independent read-only review; no files changed and no
communication with the numerical reviewer. All 12 candidate-manifest hashes
matched exactly.

Refresh revalidation: all 12 whitespace-normalized frozen hashes also matched;
the HOLD and every substantive finding remain unchanged.

## Findings

1. **Critical:** the claimed recursive closure is only over ten hand-authored
   IDs. Dozens of nonprimitive types have null nested schemas, and the listed
   terminal-ledger source declaration is not connected to a record wire.
2. **Critical:** field Rust types, fully qualified types, owner modules/stages
   and privacy claims are not resolved. They are trusted schema strings. The
   carrier fully qualified type is wrong because `carrier_phase.rs` is included
   directly into `v11_covered`, not a `v11_covered::carrier_phase` module.
3. **Critical:** the carrier projection checks only top-level names/types.
   Nested projections are unchecked placeholders without recursive schemas.
4. **Major:** the owner plan repeats trusted pseudo-module labels and does not
   establish real module paths, field visibility, helper placement or exact
   complete owner files.
5. **Major:** the generated wire echoes open schema placeholders, leaving
   optional floats, keys, supports, snapshots, receipts/parcels, provider arena
   and selection records nonunique.
6. **Major:** zero native wires is an acceptable posture, but its report is
   fixed prose rather than verified function/API analysis.

The local review shell lacked `cargo`, so this reviewer did not rerun the guard;
the primary execution recorded in the manifest did run it successfully. Static
manifest and source checks were completed read-only.
