# Contract Implementation Evidence

Status: complete

Evidence mode: static

Static:

- `SC-EVAP-001` was amended to version `11` with `INV-EVAP-016` for final
  post-WB19 root-uptake `Ep = ΣUi`, PL scheduler activation preservation,
  active root-depth (`Rd`/`rtd`) lineage, and neutral prior `Ws` seeding before
  same-day ET/root uptake.
- `SC-WATBAL-001` was amended to version `77` with `INV-WATBAL-038` requiring
  WB13 `Ep` publication to consume final post-WB19 root-uptake flux and with
  WB15 near-zero `I`/liquid roundoff canonicalization before writeback.
- `SC-PLANT-001` was amended to version `16` with `INV-PLANT-025` requiring
  established-perennial initial live-canopy assimilation from initial `cancov`
  into `vdmt`, `lai`, `rtd`, `rtmass`, and compatible `sumgdd` lineage.
- `docs/specifications/science-contracts/index.md` was updated with the
  HPHYS0250 active authority note for `SC-EVAP-001#INV-EVAP-016`,
  `SC-WATBAL-001#INV-WATBAL-038`, and `SC-PLANT-001#INV-PLANT-025`.

Contract-first posture:

- Canonical contract amendments were authored before production code edits.
- The package remains `HOLD`; contract authority now identifies the next
  continuation focus instead of declaring parity closure.
