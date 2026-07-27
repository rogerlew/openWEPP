# Scaffold Review Findings

Status: `CORRECTED / RE-REVIEW REQUIRED`

Evidence class: `Static`

Both independent reviews of `1f93921d` returned `HOLD`.

| ID | Finding | Disposition |
|---|---|---|
| SC-01 | Capability consumption owner and restart semantics were ambiguous | Pre-LIGHT is mutation-free; audit owns the sole rename/consumption and immutable proof; post-consumption restart requires new dispatch |
| SC-02 | Red/green observations were prose-only | Acceptance matrix now names tests, counters, retained trees/ledgers, exact mutation counts, and proof-verification immutability |
| SC-03 | Catalog and roadmap omitted the successor | Both now identify the invalid predecessor and prospective successor before production edits |
| SC-04 | Initial defect/write set omitted later review findings | Package now freezes lifecycle ordering, recovery races, attestation freshness, CSV headers, error taxonomy, required Rust/CAL/schema paths, and exact tests |

The invalid predecessor package bytes remain unchanged as retained evidence.
The catalog and roadmap remove its execution authority and identify this
successor as the sole active adapter correction.
