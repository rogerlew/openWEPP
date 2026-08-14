# Execute V7 Real-Hydrology Arbitration Shadow

Scope: local default-off openWEPP implementation only. Do not change production
selectors, defaults, outputs or state, and do not push.

Child 1 released `OPENWEPP_C3_WOODY_V8` and
`OPENWEPP_SNOW_FREE_LSE_V1` authority at commit
`3f1cf8ee32855a501d7d5b07ac3459d8a3fc8cc3`. Use actual production
hydrology state/candidate logic for the completed V7 water phase, preserve
exact OFE/requester/layer D/A/F
identity, isolate legacy ET inside the clone, and prove production byte
invariance and rollback. This child does not implement ground evaporation or
the LSE runtime. Add a V8-precursor root/OFE owner envelope without claiming the V8
constitutive state migration.

Subagent authorization: this prompt explicitly authorizes and requires the
reviewer, comparator and verifier roles named in `package.md`.
