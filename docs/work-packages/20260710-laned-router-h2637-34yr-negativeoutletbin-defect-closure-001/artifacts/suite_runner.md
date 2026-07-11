# Selected-Cohort Active DX5 Suite

Status: `PASS`

Evidence mode: `Ran`

## Scope and Method

The package-authorized suite runner executed only `active_default_dx5` for the
three required real cohort members: `mn_corn_h4`, `n_idaho_forest_h1`, and
`wa_cascades_forest_h1`. The run imported
`docs/work-packages/20260708-laned-router-dx5-production-mesh-policy-ratification-001/artifacts/run_default_dx5_evidence.py`
(SHA-256
`d23b43fd7153a0d9ab84037dfc94fec63361c3d5129fed83c6cae89b743faa30`)
and called `run_member_mode(member, "active_default_dx5")` once per member.

The method copied each selected-cohort materialization into the unique scratch
root
`/tmp/openwepp_laned_nob_001_suite_runner_20260711T075722Z_a822036f`,
normalized the copied text inputs, redirected runfile outputs into each copied
run directory, and did not write to or overwrite the historical dx5 package's
summaries or run tree. The source materialization was
`docs/work-packages/20260707-laned-router-d16-rowcrop-canhgt-active-runtime-publication-001/artifacts/selected-cohort-materialization.json`.

The runner sanitized the documented Lane D control variables, then set:

```text
OPENWEPP_LANED_ACTIVE=1
OPENWEPP_LANED_ACTIVE_TRACE=1
OPENWEPP_LANED_SHADOW_PROFILE=1
```

The ambient shell contained no `OPENWEPP_LANED_*` variables. In particular,
`OPENWEPP_LANED_ACTIVE_MESH_TARGET_DX_M` remained unset: activation and trace
were explicit, while dx5 came from the production-default manifest policy.

## Release-Binary Provenance

The exact target had already been rebuilt before delegation with
`cargo build --release -p openwepp-runner --bin openwepp-cli-hill`; this bounded
suite runner did not rebuild it. It verified the delegated binary before any
cohort execution.

| Field | Observed |
|---|---|
| Path | `target/release/openwepp-cli-hill` |
| Expected SHA-256 | `a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d` |
| Observed SHA-256 | `a822036fd327c2f54d877ab51dc6c2e9aae13accff2ad4a61c154cbd730a131d` |
| Size | `10687800` bytes |
| mtime (UTC) | `2026-07-11T07:34:56.607738+00:00` |
| Git HEAD at execution | `9fa0a294a0b8cd2db2abdefa38e15a2d7da0d73f` |
| Branch | `main` |

The captured execution-time worktree state was:

```text
 M crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs
 M docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md
 M docs/specifications/science-contracts/index.md
 M docs/work-packages/README.md
?? docs/work-packages/20260710-laned-router-h2637-34yr-negativeoutletbin-defect-closure-001/
```

## Exact Run Commands and Results

All commands ran from `/home/workdir/openWEPP` with the environment described
above. `/usr/bin/time -v` wrote combined process output to the corresponding
scratch `time.log`.

```text
/usr/bin/time -v target/release/openwepp-cli-hill --run-dir /tmp/openwepp_laned_nob_001_suite_runner_20260711T075722Z_a822036f/runs/mn_corn_h4/active_default_dx5/run_dir --run-file p4.plain.run.toml --output-dir /tmp/openwepp_laned_nob_001_suite_runner_20260711T075722Z_a822036f/runs/mn_corn_h4/active_default_dx5/run_dir/output

/usr/bin/time -v target/release/openwepp-cli-hill --run-dir /tmp/openwepp_laned_nob_001_suite_runner_20260711T075722Z_a822036f/runs/n_idaho_forest_h1/active_default_dx5/run_dir --run-file p1.plain.run.toml --output-dir /tmp/openwepp_laned_nob_001_suite_runner_20260711T075722Z_a822036f/runs/n_idaho_forest_h1/active_default_dx5/run_dir/output

/usr/bin/time -v target/release/openwepp-cli-hill --run-dir /tmp/openwepp_laned_nob_001_suite_runner_20260711T075722Z_a822036f/runs/wa_cascades_forest_h1/active_default_dx5/run_dir --run-file p1.plain.run.toml --output-dir /tmp/openwepp_laned_nob_001_suite_runner_20260711T075722Z_a822036f/runs/wa_cascades_forest_h1/active_default_dx5/run_dir/output
```

| Member | Exit | Status | Wall | User | System | Manifest SHA-256 |
|---|---:|---|---:|---:|---:|---|
| `mn_corn_h4` | `0` | `PASS` | `0:00.74` | `0.71 s` | `0.02 s` | `0c265944fda9ebf23354aa7ca8dc36357d18ace5e0f28f229fab434d6ff73957` |
| `n_idaho_forest_h1` | `0` | `PASS` | `0:15.65` | `15.63 s` | `0.02 s` | `9f28ea8bf9e613889f3e167aee140e706d11dc1a09f7d118a110a2d78bde5b85` |
| `wa_cascades_forest_h1` | `0` | `PASS` | `0:49.70` | `49.64 s` | `0.05 s` | `ec9f9ec35c01e9846d81795236a5f8f32469bcb4118077c9e24acd5cdba66dfb` |

All three output manifests existed and reported the active runtime.

## Production Mesh Policy

The mesh-policy assertion requires `mode=target_dx`, `target_dx_m=5.0`,
`min_cells=10`, `max_cells=4096`, and `max_dt_s=300.0` for every active run.

| Member | Mode | Target dx (m) | Min cells | Max cells | Max dt (s) | Assertion |
|---|---|---:|---:|---:|---:|---|
| `mn_corn_h4` | `target_dx` | `5.0` | `10` | `4096` | `300.0` | `PASS` |
| `n_idaho_forest_h1` | `target_dx` | `5.0` | `10` | `4096` | `300.0` | `PASS` |
| `wa_cascades_forest_h1` | `target_dx` | `5.0` | `10` | `4096` | `300.0` | `PASS` |

## Active-Manifest Closure and Clamp

The reused runner's closure assertion requires clamp/source `<= 1e-12`,
maximum supply reconstruction relative residual `<= 1e-12`, and each maximum
day cascade, seam, and identity relative residual `<= 1e-10`.

| Member | Days seen / routed | Max supply reconstruction rel | Max cascade rel | Max seam rel | Max identity rel | Total clamp (m3) | Clamp / source | Assertion |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| `mn_corn_h4` | `2557 / 209` | `4.438822286981392e-16` | `1.0388136044916924e-14` | `2.8758034552001814e-15` | `8.752483860221787e-14` | `0.0` | `0.0` | `PASS` |
| `n_idaho_forest_h1` | `1461 / 185` | `8.378371644554163e-16` | `8.787388422926861e-15` | `6.911656981460711e-15` | `5.259801400133539e-14` | `0.0` | `0.0` | `PASS` |
| `wa_cascades_forest_h1` | `2192 / 1381` | `5.714626530090617e-16` | `8.808971439230974e-14` | `1.1481975666335607e-13` | `9.798782411746036e-14` | `1.1927923918694446e-12` | `1.3783122416763528e-18` | `PASS` |

The WA clamp is nonzero in absolute floating-point dust but is
`1.3783122416763528e-18` of its `865400.7095073956 m3` source total, well
inside the runner's `1e-12` materiality bound. The other two runs report zero
clamp.

## Final Disposition

`PASS`: all three required real active cohort members exited `0`; every
manifest reported the production dx5 mesh policy; and every manifest closure
and clamp assertion passed the selected-suite thresholds. No historical dx5
evidence artifact or run directory was modified.
