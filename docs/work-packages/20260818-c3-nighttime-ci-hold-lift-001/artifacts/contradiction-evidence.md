# Exact consumer contradiction evidence

Status: `CONFIRMED / blocks forcing-adapter completion`

Ran on 2026-08-18:

```text
nix develop --command cargo test -p openwepp-hillslope-orchestrator \
  v9_real_consumer_shadow::tests::sealed_repository_receipts_project_into_real_child4_forcing_types \
  --lib -- --exact --nocapture
```

The test constructed sealed receipts from an actual parsed repository climate
day, projected them into real LSE/V9 forcing types, and entered the real Child-4
interval kernel. Interval 0 returned:

```text
Physical(Physical(LandSurface(ConstitutiveDomain("ci_bracket"))))
```

Static trace: the midnight provider receipt has exact zero shortwave. In
`occupancy_solver/constitutive.rs`, `solve_class_inner` retains positive dark
respiration, obtains nonpositive net assimilation, and calls the Ci root solve
over `[gamma, Ca]`. The respiration-driven diffusion solution is not bracketed
in that interval. No forcing tolerance or fixture substitution is authorized.

