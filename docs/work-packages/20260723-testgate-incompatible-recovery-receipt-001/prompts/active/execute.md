# Execute Incompatible Recovery Receipt Hardening

Treat only `GATE-RESUME-RECEIPT-INVALID` as a retained rejection of a prior
receipt; continue the current admitted attempt without importing it. Preserve
fail-closed handling for all other recovery-integrity errors. Do not dispatch
TESTGATE in this correction.
