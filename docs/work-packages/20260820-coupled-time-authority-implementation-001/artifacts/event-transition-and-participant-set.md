# Event Transition And Participant Set.md

Status: authority candidate

Evidence mode: Static

The complete parent owner set is immutable. Each segment declares active
participants. Inactive owners carry byte-identically unless a zero-duration
event transition explicitly debits/credits custody and closes its ledger.

Reference chronology: `[0,5)` A+B active/C unchanged; at tick 5 event B-to-C
changes custody without advancing time and creates segment 1; `[5,10)` A+C
active/B retained terminally. Parent commit installs A+B+C once.
