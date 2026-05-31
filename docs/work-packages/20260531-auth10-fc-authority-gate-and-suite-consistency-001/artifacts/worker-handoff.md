# AUTH10 Worker Handoff

Status: completed  
Evidence mode: Static

## Immediate next actions
1. Keep new FC direct-theta suite in Level-4 required lane when adjusting
   tolerances/cases; do not demote without explicit governance decision.
2. When adding new cohort fixtures, always update both:
   - `fixtures.sha256`
   - `fixtures.provenance.yaml`
   and keep suite-spec fixture hashes synchronized.
3. Preserve Level-3 WB19 suite as legacy/sanity evidence only
   (`periodic`/`investigation`) unless a separate constitutive authority
   package explicitly supersedes it.
