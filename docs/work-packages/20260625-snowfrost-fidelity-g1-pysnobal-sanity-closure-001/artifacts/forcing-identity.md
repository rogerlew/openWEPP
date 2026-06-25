# Forcing Identity and Reuse Evidence

Evidence mode: Ran.

G1 changed the diagnostic exporter to add WAT-backed `openwepp_snow.csv`; it
did not change the PySnobal forcing columns. Before reusing G0 PySnobal output
for the selected G1 `site-sane` lane, the package compared `forcing.csv`
checksums for `tg_0p0c_zg0p10m` across all five sites.

Ran:

```text
54f62afe78e0d5ae1c2158d0ddbae6e51070568e53a98be28bea18c6affcf6ab  target/snowfrost_fidelity_g0/site1/tg_0p0c_zg0p10m/forcing.csv
54f62afe78e0d5ae1c2158d0ddbae6e51070568e53a98be28bea18c6affcf6ab  target/snowfrost_fidelity_g1/site1/tg_0p0c_zg0p10m/forcing.csv
b469cc30159f28708578beb087df715e5b1fbaabb07001f91212a24150a9a45c  target/snowfrost_fidelity_g0/site2/tg_0p0c_zg0p10m/forcing.csv
b469cc30159f28708578beb087df715e5b1fbaabb07001f91212a24150a9a45c  target/snowfrost_fidelity_g1/site2/tg_0p0c_zg0p10m/forcing.csv
3586aa0190799c961497d9b149bed62ea296b2d20fc6a5c29a3db9b6eaa4d027  target/snowfrost_fidelity_g0/site3/tg_0p0c_zg0p10m/forcing.csv
3586aa0190799c961497d9b149bed62ea296b2d20fc6a5c29a3db9b6eaa4d027  target/snowfrost_fidelity_g1/site3/tg_0p0c_zg0p10m/forcing.csv
b91caf627106f138362e051524e9bc9ebe1786e5a55d6071a6b1f34735be1dc7  target/snowfrost_fidelity_g0/site4/tg_0p0c_zg0p10m/forcing.csv
b91caf627106f138362e051524e9bc9ebe1786e5a55d6071a6b1f34735be1dc7  target/snowfrost_fidelity_g1/site4/tg_0p0c_zg0p10m/forcing.csv
c2ff70582f4d78edbb0029e8664fae43a3d072a88d1327b0a9ee346843a7c0ba  target/snowfrost_fidelity_g0/site5/tg_0p0c_zg0p10m/forcing.csv
c2ff70582f4d78edbb0029e8664fae43a3d072a88d1327b0a9ee346843a7c0ba  target/snowfrost_fidelity_g1/site5/tg_0p0c_zg0p10m/forcing.csv
```

Disposition: selected-lane PySnobal output reuse is valid for G1 because the
forcing inputs are byte-identical. The harness still revalidated the reused
outputs for finite/nonnegative SWE and depth plus the 700 kg/m3 bulk-density
ceiling before producing the G1 summary.
