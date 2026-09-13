# B01 WB14 ingress regression context feasibility

Evidence class: Static.

## Source identity

* Test copy: `/workdir/openwepp-experiments/b01-wb14-cadence/baseline-red`, copied from
  `/workdir/openwepp-experiments/b01-wb14-source-reconciliation/reconstructed-available145`.
* `Cargo.toml`: `e5271cfe2bca5132e263d6227264ebfd19c03e0275a9ff08cb0419c3f61bb676`.
* `surface_liquid_ingress.rs`: `1826924b199e8a1ff7a42776ac99713b0b1bb01ab8207a1ea508cc1539096c29`.
* `surface_liquid_ingress_coordinator.rs`: `af0c281679e5b600e219eca487f3af93928f7eb2bdaaafdb1b47867115dec835`.

## Consumed capture identity

* Packet: `/workdir/openWEPP/docs/work-packages/20260912-b01-wb14-available-source-red-witness-001/artifacts/current-run-boundary-packet.json`,
  SHA-256 `d8f1fe07d73ac09a6eb002b1a49a714cad7a0af1bb58ffab7612282ea002cc76`.
* Decoded day-4/interval-22 beginning-state bytes SHA-256:
  `b2f53b6a34ff80dd91a7b42f04f1c43b6d0b5da407684881c86199d625dfbac4`.
* Decoded day-4/interval-22 ingress-input bytes SHA-256:
  `00b83e6bfcaf845c2a891245fc02ac60978e734d3b8445597462b80150dad408`.
* The input decodes to transaction 255, day 4, interval 22 and 60.0 seconds.

## Finding and correction

The first packet record contains the captured ingress input and beginning state;
the second caller-failure record contains the matching serialized
`DirectWb14ParentWorkingState` and `DirectWb14CoupledChildBindingV1`. The second
record's parent validates only against the exact direct configuration embedded in
the retained owner-seed checkpoint, not against the separate LSE/vegetation
`configuration.json`. The owner seed's direct configuration declares the same
`a25b600e898017500147cd87919e81b1e663f24bd6b6c60748cd78c22cef1c86`
identity, owner, run, topology, lane and records as the captured parent.

An earlier first-record-only inventory mistakenly reported missing parent/binding
custody. The complete two-record inventory corrected that inspection error before
any test execution. The test uses the retained parent/binding bytes and rebuilds
the retained configuration through its canonical constructor; it does not invoke
the inactive-prefix constructor or remint custody.
