# Accepted publication operand lineage

Status: `IMPLEMENTED — WAT5 ENDPOINT BLOCKED UPSTREAM`

Evidence mode: `Static + Ran`

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

Static: committed publication now reconstructs destination-basis
`UpstreamRunon` only from sealed accepted ingress receipts. Routed-source IDs
must match independently on sent and received sides, and source-area sent
volume must close destination-area received volume before publication. The
same accepted depth enters the same-support liquid-runon operand once; the
beginning transfer normalization is unchanged and is not double-counted.
Public `UpStrmQ` and `SubRIn` read the sealed downstream runon-carry operands,
not normalization or public runoff columns.

Ran: the synthetic anti-alias fixture closes 0.400000000000 m3 sent from a
100 m2 source against 0.400000000000 m3 received on a 200 m2 destination,
while distinguishing 0.002 m runon from 0.003 m local liquid and 0.005 m total
ingress. Terminal focused run `6ad7b421-ab55-4e9e-b74a-1652f1127301`
passed 4/4. The real accepted child retains nonzero routed receipt custody and
independently closes source/destination volume within `1.0e-12` m3 in that
same run. The canonical CLI consumer publishes
nonzero destination-basis `UpStrmQ` values 144.71027400837232 mm and
483.03341075609865 mm without deriving either expectation from public `Q`;
run `0ee69cbf-70b8-4e01-8981-362f72b22858`, 1/1 passed in 61.958 s.

Static: requested-only WAT5 construction installs the opt-in and producer
payload before generation, preserving accepted raw-rain segment timing,
accepted WB14 operands, and hourly accepted snow/runon supply. It does not
invent five-minute timing for positive hourly-only supply. The accepted
hyetograph shape/magnitude test and the producer-overwrite/absence guards are
anti-tautological. Atomic output publication remains confined to the cloned
pending frame.

Ran: the WAT5 shape/magnitude unit path passed in terminal focused run
`6ad7b421-ab55-4e9e-b74a-1652f1127301`. The real transaction-order target
remains unreachable on the normal canonical path: run
`75b21f23-db97-476a-bfc2-6f750bee8e22` failed after 105.944 s at
1800..2700 s with `SURFACELIQUID-E-003 IndependentClosure: soil-thermal ending
enthalpy arithmetic`, before the requested day-two WAT5 source guard. This is
recorded as an upstream package blocker, not as WAT5 endpoint closure.
