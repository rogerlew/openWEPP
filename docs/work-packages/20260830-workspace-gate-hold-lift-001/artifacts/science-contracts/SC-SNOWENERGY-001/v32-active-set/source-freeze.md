# SC-SNOWENERGY-001 v32 source freeze

Evidence mode: `Static + Ran`

## Authority identities

- Owner-authorized package amendment:
  `c1cfd6e4bb5d4f28f85fae538b70c75747f207a6`
  (`Authorize snow vapor active-set solver`).
- Contract-test write-set admission:
  `8ec04440de3dc593406153ed8e4f08c6093c552f`
  (`Bind snow v32 contract test`).
- Contract-first authoring worktree HEAD observed before the v32 pre-red:
  `d7f03ae847c339a9bc2edaed162c2ce7bf23ef09`.
- Existing v31 authority is retained for same-disposition endpoints. Version 32
  supersedes only the opposite-sign vapor-disposition refusal described by
  `WGHL-FULL-001D-V32`.

## Unchanged production identities at contract intake

| Path | Git object ID | SHA-256 |
|---|---|---|
| `crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs` | `03fe26391ee3116787fdd9a9f8d8a7d8228aaf33` | `7e6dbe0653ee4727e64a93c2448f60ea8f87b675f68a0f88b9e3b9a40c3be7f1` |
| `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs` | `8bc3e9d94f2f2acd38b0e4c8f1150e2091e7ffef` | `3418232f68b0f472ed488ad4db676f27b1e5f2ed7f91d5986eddbcfa5f715349` |
| `crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs` | `553e406937ecf982d7f2ccc4883fbc0576ec8a76` | `4b87082cf67631b4ce4b66020ed1adb80cb3d6b3e8485613c363a441ccd0a595` |

These identities freeze the unchanged v31 implementation against which the
contract-derived expected-red was authored. They are evidence identities, not
permission to overwrite concurrent production work.

## Captured canonical support operands

Source: direct owner amendment and `WGHL-FULL-001D-V32` failure inventory for
the canonical DFF-WS2 `1_860_000_000_000..1_920_000_000_000 ns` exact covered
support.

| Image | `V` (`kg m^-2`) | `D` (`kg m^-2`) | `S` (`kg m^-2`) | `Q_v` (`J m^-2`) |
|---|---:|---:|---:|---:|
| current unpublished | `+2.12159691239571346e-4` | `+2.12159691239571346e-4` | `+0` | `+649.057936925197964` |
| raw authentic | `-4.61661230425127085e-3` | `+0` | `+4.61661230425127085e-3` | `-13081.6326253264015` |

The strict-convex vapor root is
`alpha_v=-V_current/(V_authentic-V_current)=0.04393657257739406` in
binary64. Direct binary64 evaluation gives exact positive-zero signed vapor.
Affine interpolation of the endpoint latent components instead gives the
captured inadmissible `+45.77845449909091 J m^-2`, proving that linked latent
energy must be set from the active disposition rather than interpolated across
the zero interface.

## Protected boundaries

Version 32 changes no tolerance, iteration cap, adaptive proposal/acceptance
policy, 60-second minimum floor, constitutive equation, event chronology,
topology, custody, receipt, rollback, public schema, persistence, diagnostic,
or publication surface. Interface and branch-entry images are synthetic and
unpublished; raw authentic history and later fresh-authentic-only finalization
remain mandatory.
