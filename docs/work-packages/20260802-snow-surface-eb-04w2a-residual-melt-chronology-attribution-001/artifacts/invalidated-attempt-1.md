# Invalidated Attempt 1

Status: `INVALID / PRE-SIMULATION INPUT FAILURE`

Evidence mode: **Ran + Static**.

The first frozen execution attempted all eight declared cells, and all eight
returned before forcing export or CoE simulation. The package-local command
passed each retained direct-production `*-B.run` to snowbench. Snowbench uses
the supplied file stem to regenerate a harness runfile, so it searched the
scaled fixture for nonexistent `*-B.sol`, `*-B.man`, `*-B.slp`, and `*-B.cli`
files. The scaled fixtures instead contain their canonical `p1.run`, `p2.run`,
or `p8.run` and matching sidecars.

No `coe_melt_snow.csv` or `coe_melt_summary.json` was produced. The failed
outputs remain under
`target/snow_surface_eb04w2a_melt_chronology_attribution/invalidated-attempt-1/`.
The superseded freeze is retained as `invalidated-attempt-1-freeze.json` with
SHA-256 `c33d89aa9c219a9c3ed21f0793d72eef5cab7a7748b2afd4f626c30de37acaeb`.

The correction uses the one canonical `.run` already present and hash-bound in
each scaled fixture. This is a result-blind harness-command repair; the frozen
models, cells, operators, windows, thresholds, and claim boundaries are
unchanged. A corrected freeze must predate every valid replay.
