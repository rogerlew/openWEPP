# Worker Handoff

Status: **COMPLETE**.

## Handoff

Static: No in-scope implementation handoff remains for `GAP-OFEROUTE-007` on
the opt-in Lane D shadow. The source-authorized friction operand path is:

- static coefficients from native management `routing_coefficients`;
- `I_h` from live `wb14_hourly_rainfall_m[h] / 3600 s`;
- `LAI` from post-growth direct day-frame plant state;
- `h_c` from typed-management `canhgt`, required positive when `LAI > 0`.

Out-of-scope follow-ons remain separate: D10 `GAP-OFEROUTE-005` shock numerics,
D12 melt-limb hourly source coverage, D13 ADR-0036 erosion hourly-shape switch,
D14 production activation/profiling, and D15 default-promotion policy. Any
future production/default consumer must prove it reads the same rev-21 operand
path before promotion.
