# Real Consumer Proof

Status: `PASS for WAT5 publication; erosion cutover not applicable`

Evidence mode: `Ran`

The real `openwepp-cli-hill` streaming callback consumed
`DirectDayFrame.wat5_subhourly_generation`, wrote
`H61.wat-subhourly.parquet`, and registered its checksum in the run manifest.
PyArrow independently read 24 rows, reconstructed both authoritative hours,
and observed all candidate erosion columns null. See
`five-minute-water-closure.md`.

No Wave-1 real-consumer proof is claimed because erosion was not adopted.
