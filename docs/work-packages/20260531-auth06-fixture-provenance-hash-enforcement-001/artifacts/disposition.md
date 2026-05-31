# AUTH06 Disposition

Status: completed  
Evidence mode: Static + Ran

## Scope
- Close fixture reproducibility gaps for active external-authority suites.

## Decision
- **GO**

## Exit-criteria adjudication

1. Schema/model/template require fixture hash + source provenance:
   - pass
2. Active Level-4 suites publish lock/provenance sidecars:
   - pass
3. Release gate enforces blocking fixture-integrity checks before lane runs:
   - pass
4. AUTH06 contract-derived tests (including tamper detection) pass:
   - pass
5. AUTH06 artifacts and gate evidence are published:
   - pass

## Rationale

- AUTH06 turns fixture reproducibility into a machine-enforced gate.
- Sidecar lock/provenance files are now first-class required artifacts for
  active suites.
- Release automation fails closed on missing/mismatched fixture integrity.

## Follow-on

1. Extend sidecar backfill to future Level-5/Level-6 suites at authoring time.
2. Consider signed lock manifests for higher-trust release attestations.
