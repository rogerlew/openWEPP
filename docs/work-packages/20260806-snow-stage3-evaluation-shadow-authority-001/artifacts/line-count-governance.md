# Line-Count Governance

Evidence class: `Static + Ran`

The increment changes no production `.rs` file. Its only substantive Rust file
is the new 130-line static integration test; all other `.rs` diffs are one-line
v126-to-v127 assertion updates, except one file with two such updates.

Ran: `runoff_reconciliation.rs` remains `3,177` lines, exactly inherited from
the predecessor. It is dispositioned—not waived—as the first mandatory action
of `SNOW-STAGE3-SHADOW-SOLVER-EXTRACTION-AND-OBSERVABILITY`. This authority-only
package adds no line to it and authorizes no feature work there before the
mechanical extraction.
