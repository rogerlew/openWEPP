# Disposition

Status: executed-hold

Evidence mode: Ran

Ran:

Final disposition: `executed-hold`

WBVAL01 executed the Rocky Mountain single-OFE validation package as far as the
current runner allows.

Results:

- Discovered `22` single-OFE hillslopes (`p1..p22`) and `pw0` as a `9`-OFE
  observed-only surface.
- Built release `openwepp-cli-hill` at source commit
  `30794db1ce5031aa9a8639a246bd61ce440ee801`.
- Generated TOML wrappers because direct legacy `.run` execution fails closed
  with `CLIHILL-E-010`.
- Ran all `22` single-OFE hillslope wrappers.
- `12/22` emitted complete WAT ledgers.
- `12/12` emitted ledgers are `conservation-break` for full years `2..6`
  against the `1.0 mm/year` tolerance.
- `10/22` failed closed before WAT publication:
  - `6` `CLIM-RUNTIME-E-017`
  - `4` `HKERNEL-WB11-PERC-E-003`
- Year `1` full-calendar closure is not classified because WAT output lacks a
  pre-day-1 initial storage row.

Closure criteria not met:

- The package cannot claim a complete ledger for every single-OFE hillslope
  because `10` single-OFE hillslopes emitted no WAT rows.
- The package cannot claim every-year full-calendar closure because year `1`
  lacks initial storage.

This is not a production failure and no code changes were made. It is an honest
characterization hold: the emitted surfaces already name concrete frost targets,
and the blocked surfaces name required domain-unblock work.
