# Restart owner lineage domains

Status: `binding remediation design / executable proof pending`

| Domain | Owners | Cursor / transaction | Required joins | Omission consequence |
|---|---|---|---|---|
| immutable configuration | GSI configuration, forcing static configuration, vegetation configuration reference, LSE configuration reference, surface-liquid configuration, phase plan and day-input references | no interval transaction | exact canonical content digest and supplied-context equality | restored owners may validate against different physics/configuration |
| daily GSI | GSI beginning state, accepted daily receipt, staged ending state | climate date and day index | owner/run/day, climate source, beginning/ending state and configuration digests | GSI history or daily result diverges |
| daily provider | beginning cursor, complete destination receipts, ending cursor and precipitation carry | next-day index | static configuration, ordered destinations, 48 ordered intervals, GSI receipt, outgoing carry union | forcing replay/skip or precipitation loss/duplication |
| interval scientific | V10, LSE-V2, direct hydrology including surface liquid, soil thermal, BGC | last accepted interval transaction | one common accepted interval transaction across all five staged owners | mid-day continuation or atomic abort diverges |
| scheduler | checkpoint phase, day index, next interval and accepted count | semantic fixed-width day/interval/count types | boundary modulo 48; in-progress interval `1..=47`; cursor positions | replay, skip, or wrong day resumption |

`DirectHydrologyRestartV1` belongs inside both the committed day-beginning and
staged scientific sets. GSI and provider cursor do not inherit the interval
scientific transaction lineage.

Every externally supplied immutable value must retain a persisted identity,
named reconstruction source, exact digest comparison, typed mismatch poison,
and omission consequence in generated field metadata.
