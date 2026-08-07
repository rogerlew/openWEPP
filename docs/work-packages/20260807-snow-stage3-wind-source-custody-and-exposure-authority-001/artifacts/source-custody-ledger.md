# Wind Source Custody Ledger

Status: `complete / provider custody recovered / authority gaps explicit`.

Evidence mode: `Static + Ran`.

General product authority: Earth Engine asset `IDAHO_EPSCOR/GRIDMET` describes
daily `vs` in `m s^-1`, nominal `10 m`, at `4638.3 m` pixel size. Abatzoglou
(2013), DOI `10.1002/joc.3413`, describes the native gridded product; NASA GSFC
NLDAS-2 documents hourly 10 m vector forcing upstream.

| Site/role | Retained CLI identity | Product/version/status, pixel/sampling, generator/config, timezone/fill/transform | CLI/runtime/consumer | Disposition |
| --- | --- | --- | --- | --- |
| Mica Creek canonical | `target/.../snotel_mica_creek_st_joe_id/p1.cli`, `e8470ae7...` | Byte-identical `/wc1/runs/li/listed-scar/wepp/runs/p1.cli`; retained watershed centroid `(-116.26483416760449, 47.14987731602243)`, GRIDMET flag, daily parquet/CLI equality | daily `w-vl` (`m/s`) -> raw `vwind` -> `vwind_m_s` -> Stage 3 | partial custody; deployed request/product path, server semantics, and exposure missing |
| Niwot canonical | `target/.../snotel_niwot_co/p2.cli`, `841d639...` | Byte-identical `/wc1/runs/de/deathless-wangle/wepp/runs/p2.cli`; retained centroid `(-105.5424440268039, 40.036382248555775)`; same direct value evidence | same | same remaining gaps |
| Paradise canonical | `target/.../snotel_paradise_wa/p2.cli`, `6e0c874e...` | Byte-identical `/wc1/runs/op/open-source-thirtieth/wepp/runs/p2.cli`; retained centroid `(-121.74839302639597, 46.7843679183757)`; same direct value evidence | same | same remaining gaps |
| Snowbird development, non-decisive | `target/.../snotel_snowbird_ut/p8.cli`, `c673145e...` | Canonical wind comes from byte-identical `/wc1/runs/ba/barred-pro/wepp/runs/p8.cli`, `10c1ede1...`; retained centroid `(-111.65847092309646, 40.56532186724135)`; derivative changes precipitation only | same | same remaining gaps; never canonical |

Canonical Snowbird reference is separately `tests/fixtures/.../p8.cli`,
`10c1ede1...`, and is not substituted for the retained development estimand.
Ran: all five full hashes match `artifacts/authority-freeze.json` and its
receipt. Provider/fixture pairs and parquet-to-CLI daily serialization also
match; see `provider-custody-recovery.md`. Values were used only to verify the
declared retained value equality, never to infer product identity or exposure.
Static: nearby pre-build source would request GRIDMET `vs` at each centroid,
share one run-level series, and serialize one decimal. Unknown deployed identity
means those request and transform semantics are reconstruction only.
