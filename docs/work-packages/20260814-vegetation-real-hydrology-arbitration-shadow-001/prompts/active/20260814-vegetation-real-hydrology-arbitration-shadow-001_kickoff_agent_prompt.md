# Execute V7 Real-Hydrology Arbitration Shadow

Scope: local default-off openWEPP implementation only. Do not change production
selectors, defaults, outputs or state, and do not push.

Execute this package after Child 1 releases authority. Use actual production
hydrology state/candidate logic, preserve exact D/A/F identity, isolate legacy
ET inside the clone, and prove production byte invariance and rollback.

Subagent authorization: this prompt explicitly authorizes and requires the
reviewer, comparator and verifier roles named in `package.md`.
