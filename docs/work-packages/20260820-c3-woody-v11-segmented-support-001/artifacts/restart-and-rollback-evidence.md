# Restart And Rollback Evidence

Status: authority executable PASS; production implementation remains gated.

`restart_v3_reference.py` embeds the complete immutable V2 checkpoint and
persists custody only for its one accepted `[0,600000000000)` ns slab. Parent,
cursor, slab/event ordinal, receipt, and support fields join V2 exactly. V3
admits three prefix debit receipts, three transitions, and seven complete owner
candidates; terminal candidate bytes equal V2 staged owners and staged V11
state. The external suffix `[600000000000,1800000000000)` is frozen separately,
never serialized as accepted work, and is executed from the restored prefix.
Full seven-owner/resource/receipt/event/reduction/publication continuation
matches uninterrupted execution at SHA-256
`0b2ff7b0182c756d6d706016b164459d5d55e99e148bd776aca1c0d1d6341096`.
The uninterrupted branch starts independently from frozen parent-beginning
owner/state bytes and applies frozen prefix and suffix operations without
reading the V3 checkpoint. The restore branch separately validates V3 and
executes the suffix. Comparison includes final owner/state, resource/material,
slab/event, reduction, and publication bytes.

Thirteen poisons cover a structurally valid V3/V2 graft mismatch, nanosecond support
scaling, cursor forgery, future-prefix substitution, terminal-owner mismatch,
missing/extra candidate cardinality, unconsumed suffix, candidate forgery, and
missing debit linkage, restored-only prefix forgery, coordinated parent/segment
reframing, and a forbidden snow-to-hydrology flux mapping. The authority integration test pins the three V2
artifact hashes, V3 poison count, accepted-prefix cardinality, and full
continuation digest.
