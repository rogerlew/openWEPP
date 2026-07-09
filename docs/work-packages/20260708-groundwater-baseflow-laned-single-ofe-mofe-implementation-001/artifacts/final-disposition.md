# Final Disposition

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

The package implemented the Lane D direct-runtime groundwater linear reservoir
and proved generated groundwater baseflow (`gwbfv`) through the direct WAT
producer and real watershed WAT consumer.

It does not claim full groundwater/baseflow export closure because generated
groundwater-reservoir deep seepage (`gwdsv`) and the `bftharea`
watershed/channel threshold branch still lack real downstream consumer proof in
this package.

Completed:

- `gwcoeff.txt` parser handoff into runner sidecar resolution;
- disabled authority for missing `gwcoeff.txt`;
- direct run-level groundwater storage carry;
- single-OFE and MOFE recharge/baseflow/deep-seepage recurrence;
- active-router negative proof by construction;
- direct WAT nullable `Base`;
- `hillslope_wat.Base:mm` unit registry authority;
- watershed WAT `Base` consumer proof;
- full Rust gates listed in `artifacts/gate-results.md`.

Held:

- generated `gwdsv` real downstream consumer;
- `bftharea` watershed/channel threshold branch.
