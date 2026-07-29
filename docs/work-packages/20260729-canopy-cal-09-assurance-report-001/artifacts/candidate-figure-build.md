# Candidate Figure Build

Status: `PASS — retained figure set; figure contract frozen`

Evidence class: `Ran`

## Build

From `/home/workdir/openWEPP`:

```console
PYTHONDONTWRITEBYTECODE=1 .venv/bin/python \
  docs/work-packages/20260729-canopy-cal-09-assurance-report-001/tools/build_candidate_figures.py
PASS candidate_figures=8
```

Environment: CPython 3.12.3, Matplotlib 3.10.8, NumPy 2.4.6, and Pandas
3.0.3 from the repository `.venv`. The strict synthesis procedure is
standard-library-only; the separate time-series figure builder requires these
declared plotting dependencies.

The command was run twice. SHA-256 inventories of all eight SVG outputs matched
byte for byte.

## Output Identities

| Candidate | SHA-256 |
| --- | --- |
| `f1-coefficient-response.svg` | `756c3cca40e8b5fc91d1f9c210a343ba0f1cb024b94b459bb493bf02393b019b` |
| `f2-forest-class-seasonality.svg` | `39695fe9b669822ebcfded9a0ca7d4e30c08edc10ab8f6de48a00d3692e64ce8` |
| `f3-litter-residue-frost.svg` | `cb4b004dded2e2b36d7df26051287168c69e0eb9e245f20b37844ddf2eecde89` |
| `f4-temperate-observed-modeled-timing.svg` | `4775f72472eed5a46f323e85bce08d0b90477e2d6a2d1f9d179a34767829b268` |
| `f5-source-decay-trajectories.svg` | `2fecbe9b96ce0481201430a48f96b39064410fe2d0650523df049b37be4e9f4d` |
| `f6-canopy-gradient-snow-response.svg` | `8010b6e636f721dbff4b2c91bf181426fe3003f6a3fcf5ce5c1a8e0e779f4380` |
| `f7-hemisphere-seasonality.svg` | `1b543153c51ad42b04ea5e35426a8107f609c1740914bde73178f0c7b74a0711` |
| `f8-beza-observed-modeled.svg` | `cb127d5c8e9e587a26cda8a25ab0fac9e32c5052965e7620aeb9a3056657ab4c` |

## Validation

- All eight SVGs parse with `xmllint --noout`.
- All nine rows in `figure-candidates/source-manifest.csv` resolve and match
  their recorded SHA-256 identities.
- Every SVG has a paired Markdown sidecar with candidate caption, reader
  context, data/method description, and limitations.
- Derived plotted rows are retained for F1, F4, F6, and F7.
- F9 is absent by operator decision.
- F10 is represented by table T4 rather than an explanatory figure.

## Boundary

These are reviewed report assets, not approved V2 publication figures. Every
manifest path is repository-relative, and the plotted source and derived
tables are declared public-safe research objects in the report descriptor.
The current V2 schema can render only its native linear-magnitude-bar figure;
the time-series SVGs therefore remain linked research objects with adjacent
Markdown captions and limitations. Publication admission remains blocked
until the report passes the canonical V2 identity transaction.
