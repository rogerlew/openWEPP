# Hold Legitimacy Audit

Status: EXECUTED-HOLD-CASE4-HYBRID-LADDER

## Hold Condition

The parent LANED-T3 hybrid selector cannot be ratified because the Case-4
hybrid oracle ladder fails the current `SC-OFEROUTE-001` Case-4 peak tolerance.

## Evidence Proving The Blocker

Ran:

```
cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick
```

With the retained ratification vector active, the run failed:

```
ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle

hybrid peak error at ladder step 0: 0.2278616770287263
(ref 0.008334954309852328); ladder
[0.2278616770287263, 0.1546442906262445, 0.1020328027401588]
```

Contract tolerance:
- `SC-OFEROUTE-001` rev 24/26 Case-4 peak tolerance: `<= 0.05` relative error
  at every rung of the ladder.

Observed:
- 120 cells / 0.25 s sample: `22.8%`
- 240 cells / 0.125 s sample: `15.5%`
- 480 cells / 0.0625 s sample: `10.2%`

After the vector was quarantined as ignored to keep normal CI green, the held
ratification gate was re-run explicitly:

```
cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only
```

Result: FAIL, 1/1, `150.896 s`; see
`artifacts/case4-hybrid-ignored-ratification.log`.

The sequence improves under refinement but fails every rung of the named
tolerance. This is not a tooling failure or missing run; it is a real
ratification failure.

## Why This Is Outside Safe Closure In This Package

The package was authorized to execute the two open ratification gates and land a
deterministic implicit solve-cost lever. It was not authorized to rewrite the
hybrid method's fidelity policy after the comparator failed.

Possible in-envelope routes considered:
- **Ratify looser tolerance:** rejected. The current Case-4 acceptance tolerance
  is already contract-ratified from D10B and cannot be loosened merely to accept
  a method that fails it.
- **Promote on H2637 timing/closure only:** rejected. H2637 active hybrid timing
  is useful solve-cost evidence (`36.61 s` user; rev-31 counters live), but it
  cannot replace the required Case-4 oracle ladder.
- **Keep the failing vector non-ignored and close gates:** rejected. Normal CI
  would fail permanently. The vector is retained and explicitly ignored with a
  reason so it can be run as a ratification gate while the package closes held.
- **Partial selector promotion:** rejected. The package and contract require no
  promotion when a required ratification gate fails.

## Work Completed Despite Hold

- `SC-OFEROUTE-001` rev 31 added cost-only authority for deterministic
  branch-local warm seeding and implicit solve-cost counters.
- The implementation landed profile counters for implicit equilibrium map
  evaluations and branch evaluations.
- The active H2637 timing/profile evidence passed with the exact release binary:
  `36.61 s` user / `0:36.65` wall; `274681460` implicit equilibrium map
  evaluations and `37241376` branch evaluations recorded.
- Tier-1 and Tier-2 follow-on packages were scaffolded.

## First Actionable Follow-On

Execute `docs/work-packages/20260707-laned-router-tier1-local-numerics-001/`
first. It targets the dominant remaining cost source shared by explicit and
implicit paths (friction/equilibrium/celerity evaluation) without changing mesh
policy.

Do not promote `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` until a later package either:
- changes the hybrid method/implicit phase under contract-first authority and
  passes the Case-4 hybrid oracle ladder, or
- records a new authority-backed ratification criterion that supersedes the
  current Case-4 peak/rise/timing requirements.
