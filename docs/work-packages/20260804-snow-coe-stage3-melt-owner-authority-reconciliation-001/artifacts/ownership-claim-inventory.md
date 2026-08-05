# Ownership Claim Inventory

Status: complete

Evidence mode: Static

| Claim surface | Prior claim | Final classification | Disposition |
|---|---|---|---|
| `SC-SNOWFREEZE-001#INV-SNOWFREEZE-080` | CoE owns melt; Stage 3 routes already-generated liquid. | `superseded-target` and `current-runtime` | `INV-SNOWFREEZE-093` supersedes only prospective melt ownership. Existing runtime description and other guards remain binding during the hold. |
| `INV-SNOWFREEZE-085` | Positive surface-energy excess is not converted; CoE remains authoritative. | `superseded-target` and `current-runtime` | Target prohibition is replaced by Stage 3 bounded phase conversion; present reported excess and CoE generation remain truthful implementation state. |
| `INV-SNOWFREEZE-086` | Active/lower energy satisfies cold content but does not melt; CoE owns existence/melt. | `superseded-target` and `current-runtime` | Thermal-volume, conduction, timestep, and no-limiter clauses remain; only future melt ownership changes. |
| `SC-SNOWENERGY-001` versions 3-6 | CoE owns snow mass and positive excess is rejected. | `superseded-target` and `implementation-hold` | Version 7 admits positive-excess conversion only after complete energy and atomic cutover gates. |
| `INV-SNOWENERGY-026` | At `m_s <= 1 kg m^-2`, preserve CoE/persistent state and suspend Stage 3. | `current-runtime` and `implementation-hold` | Current behavior remains. Target residual-snow phase disposition is unresolved and blocks cutover; no proxy is authorized. |
| `INV-SNOWFREEZE-091` | One authoritative production result with linked solid/liquid ledgers. | `retained` | Becomes a core cutover guard; Stage 3 must own the exact debit/credit and handoff after cutover. |
| `INV-SNOWFREEZE-052/055/056` and CoE modernization addenda | CoE candidates/default constraints govern prospective melt work. | `historical-compatibility` and `superseded-target` | Candidate-specific guards remain for the current runtime; `INV-SNOWFREEZE-093` supersedes any lasting CoE target ownership. |
| Stage 3, Stage 3-Decouple, EB-03, EB-03A, and promotion addenda | Preserve CoE while introducing diagnostic thermal/liquid capability. | `current-runtime` and `superseded-target` | Their no-partial-change, closure, schema, and rollback guards remain. Their prospective CoE-owner phrases cannot survive the atomic 093 cutover. |
| `INV-SNOWFREEZE-092` | Wet compaction consumes current gross positive generated melt while preserving the selected generator. | `retained-chronology` | Current CoE source remains until cutover; afterward the same exact pre-retention operand must come from Stage 3 without changing anti-alias rules. |
| Current CoE Rust source | Post-2007 CoE generates runtime melt. | `historical-compatibility` | Byte-identical in 21N; permitted only until an atomic Stage 3 implementation/cutover package closes the hold. |
| Current Stage 3 Rust source | Applies partial surface energy to cold content and reports unused positive energy. | `implementation-hold` | Not target-conforming because complete fluxes and melt conversion are absent. |

`INV-SNOWFREEZE-093` supplies the global rule: every earlier invariant,
addendum, boundary row, and package constraint is superseded only where it
assigns lasting target melt ownership to CoE. Historical/current-runtime
descriptions and unrelated guards remain binding. No active clause authorizes
simultaneous CoE and Stage 3 melt generation.
