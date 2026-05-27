# WSHEDIMPL14 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
- Added comparator lane test
  `wshedimpl14_baseline_authoritative_cli_lane_replays_baseline_ebe_signature`
  in `crates/openwepp-runner/tests/watershed_cli_behavior_contract.rs`.
- Comparator lane reads baseline authority data from
  `.../wepp_dcc52a6/ebe_pw0.txt`, seeds watershed HBP fixture from baseline
  peak/runoff signature, runs `openwepp-cli-watershed`, and validates:
  - key continuity (`month`, `day_of_month`, `simulation_year`),
  - topology dispatch continuity (`sim_day_index`),
  - branch/publication continuity (`chan.out` peak equals `ebe` peak),
  - baseline-signature runoff/peak parity at emitted parquet boundary.
- Added runner test dev dependency on `parquet` for direct parquet row decode.

## Ran
- not run
