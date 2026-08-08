# Prompt Lifecycle

Status: `queued`

The executable kickoff prompt begins in `active/`. At terminal disposition,
move it byte-for-byte to `archived/` and update both directory indexes.
