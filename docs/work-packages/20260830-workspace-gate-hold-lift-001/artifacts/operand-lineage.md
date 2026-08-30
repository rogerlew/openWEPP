# Accepted publication operand lineage

Status: `PRE-IMPLEMENTATION`

Evidence mode: `Static`

| Published/checked quantity | Unit/basis | Authoritative source | Required normalization and closure | Known wrong formula/alias rejected |
|---|---|---|---|---|
| accepted upstream runon | `m` on destination OFE ground | sealed accepted `ingress_receipts()` whose kind is `UpstreamRunon`, including support, source/destination identity, disposition splits, and destination-basis amount | aggregate exactly once from accepted receipt dispositions; reconcile sent/received per support, adjacent transfer, and hillslope cancellation | open-ingress/LSE forcing parcels; upstream public `Q`; total ingress; local rain |
| accepted local liquid | `m` on destination OFE ground | sealed accepted non-runon ingress receipt/parcel operands, retaining distinct raw-rain and vegetation-release kinds | aggregate each accepted destination source once; `local_liquid = accepted_ingress - accepted_runon` only after independent receipt/parcel reconstruction | total ingress mislabeled local; public runoff; canopy storage |
| public `UpStrmQ` | output schema units from destination-basis accepted runon | accepted runon reconstruction above | schema conversion after physical reconstruction; real public row must match independent destination-basis sum | zero/default carry; upstream `Q`; producer-side area basis |
| requested WAT5 timing/intensity | WAT5 schema units on accepted support | accepted Stage-3 ingress/replay timing and raw WB14 producer parameters | assemble only when requested; preserve every accepted segment and positive non-rain additional supply; install opt-in and producer payload before generation | public daily row reconstruction; historical daily builder; synthetic uniform timing |
| WAT5 transaction set | bytes plus manifest identity | pending cloned publication frame after accepted-source generation | publish staged siblings before manifest and atomically roll back on source/writer/close failure | partial sibling replacement; manifest-first publication; absent-payload fallback |

Anti-tautology fixtures must give different numeric results for each listed
wrong formula. Acceptance requires independent output reconstruction plus real
per-OFE, adjacent-transfer, hillslope-cancellation, WAT5 magnitude, and
transaction atomicity evidence.
