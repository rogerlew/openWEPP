# Kernel Profile Compliance Checklist

Evidence: Static
Date: 2026-06-10

| Item | Result | Notes |
|---|---|---|
| Kernel/runtime production code edited | no | SCSTRUCT05 edited contract/docs and one contract-derived test only. |
| New physics/equation authority introduced | no | No new `INV-*`/`OBL-*` rows were promoted. |
| Binding obligations removed or weakened | no | Three historical rows moved to sidecar with `INV-SYSTEM-027` conservation. |
| Typed guards changed | no | No runtime guard code or guard table rows changed. |
| Comparator tiering changed | no | No comparator re-tiering or acceptance-threshold change. |
| Remaining unresolved authority explicitly gated | yes | 11 narrower HOLD rows listed in `followon-queue.md`. |
