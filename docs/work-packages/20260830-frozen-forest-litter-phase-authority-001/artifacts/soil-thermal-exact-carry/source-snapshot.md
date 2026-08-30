# Soil-thermal exact-carry source snapshot

Evidence mode: `Static`

Captured before the version-15 contract or contract-test edits and before any
production implementation.

| Surface | Captured identity |
|---|---|
| repository source | commit `b68d7756d2ca41bf2b5778ade6f779e68795da22` |
| `SC-SURFACELIQUID-001.md` v14 | SHA-256 `bbb165f03c2f3588b32d4e97b41757612a73ef2641c4b4c8ae4d07f4a66df7e8` |
| `SC-LANDSURFACEENERGY-001.md` v14 | SHA-256 `857b49f06fdb675cd91fe2776727388aea72d19fdb999e2e4cd6e248f0e836d1` |
| science-contract index | SHA-256 `343ef3b9d161046a9ed294efee84a0d2202fbf32fdf56ddc79d194f9d0228b12` |
| surface-liquid authority test | SHA-256 `a73614342a7a2ea0159a002d55bd6e1470dfef15e5cffc76d2d9593faa8efd72` |
| LSE authority test | SHA-256 `520cc94a1878494cf06f21c6aa9c739b0ee362aa787015182df62f3841e21832` |

The observed canonical WAT5 failure has beginning soil high term
`H_hi=-34315.42154113602 J m^-2` and accepted infiltration energy credit
`Q_inf=-8.0670339832330148e-19 J m^-2`, only `1.10875e-7` ULP of the high
term. The nonzero accepted credit is exactly conserved but cannot change the
binary64 high-term bits; the existing independent closure therefore refuses
with `SURFACELIQUID-E-003` rather than silently discarding energy.

This snapshot supplies reproducibility identity only. It is not authority for
a tolerance, forced ULP, `nextafter`, canonical-zero change, producer residual,
or process-physics change.
