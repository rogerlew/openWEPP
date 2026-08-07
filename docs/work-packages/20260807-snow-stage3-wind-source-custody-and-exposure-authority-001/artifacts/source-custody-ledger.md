# Wind Source Custody Ledger

Status: `complete / authority gaps explicit`.

Evidence mode: `Static + Ran`.

General authority only: Earth Engine asset `IDAHO_EPSCOR/GRIDMET` describes
daily `vs` in `m s^-1`, nominal `10 m`, at `4638.3 m` pixel size. Abatzoglou
(2013), DOI `10.1002/joc.3413`, describes the native gridded product; NASA GSFC
NLDAS-2 documents hourly 10 m vector forcing upstream. None identifies the
retained generator request or exposure.

| Site/role | Retained CLI identity | Product/version/status, pixel/sampling, generator/config, timezone/fill/transform | CLI/runtime/consumer | Disposition |
| --- | --- | --- | --- | --- |
| Mica Creek canonical | `target/.../snotel_mica_creek_st_joe_id/p1.cli`, `e8470ae7...` | `AUTHORITY_MISSING`; manifest says “GRIDMET wind” only | daily `w-vl` (`m/s`) -> raw `vwind` -> `vwind_m_s` -> Stage 3 | custody incomplete |
| Niwot canonical | `target/.../snotel_niwot_co/p2.cli`, `841d6390...` | `AUTHORITY_MISSING`; same limitation | same | custody incomplete |
| Paradise canonical | `target/.../snotel_paradise_wa/p2.cli`, `6e0c874e...` | `AUTHORITY_MISSING`; same limitation | same | custody incomplete |
| Snowbird development, non-decisive | `target/.../snotel_snowbird_ut/p8.cli`, `c673145e...` | Original wind is unchanged by the precipitation-only derivative, but original GRIDMET request remains `AUTHORITY_MISSING` | same | custody incomplete; never canonical |

Canonical Snowbird reference is separately `tests/fixtures/.../p8.cli`,
`10c1ede1...`, and is not substituted for the retained development estimand.
Ran: all five full hashes match `artifacts/authority-freeze.json` and its
receipt. No wind values were used to fill missing custody.
