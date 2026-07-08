# Final Disposition

Status: `EXECUTED-HOLD-MN-CORN-H4-SHAPE-NONCONVERGED`
Evidence mode: Ran.

## Outcome

This package executed the narrow rev-41 Tier-2 hold-lift and stopped at a
legitimate hold. The `mn_corn_h4` `dx0p625` reference ran successfully, but the
strict one-third routed-shape adequacy gate did not close:

- `dx1p25` vs `dx0p625` shape max L1:
  `0.02094494047849004`
- required threshold: `0.0166667`

No `SC-OFEROUTE-001` amendment or Rust production mesh-policy change landed.
The active production default remains fixed `10 cells/OFE`. Shadow mesh policy
is unchanged and out of scope because no active production promotion landed.

## Evidence Summary

- Release runner SHA:
  `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`.
- Execution HEAD:
  `25a9f52d2b6dba7d18188d2e0d0523c4f0d7f6a1`.
- Rungs run for `mn_corn_h4`: fixed10, `dx20`, `dx10`, `dx5`, `dx2p5`,
  `dx1p25`, `dx0p625`.
- All seven rungs passed active closure.
- Pass parquet hashes were identical across all seven rungs:
  `a364287f6fe348f609d25f341823781fdb6885607644eb531050ba1abbf5084f`.
- The max shape miss is a routed-hourly redistribution on
  `sim_day_index=792`, `lane_index=1`, not a discrete shape-class cliff.

## Gates

Completed:

- `git diff --check`
- Markdown/doc lint for touched docs
- exact release-runner provenance
- `mn_corn_h4` `dx0p625` opt-in active trace run
- strict one-third adequacy gate, failed as hold evidence
- focused runner mesh-selector tests
- focused orchestrator active mesh/closure tests
- `cargo fmt --check`

Not required because no contract/code production flip landed:

- contract/profile/BEI checks
- protected default/off byte identity
- DC01/no-double-feed proof
- routed-hydrograph-to-erosion consumer proof beyond retained trace evidence
- full clippy/workspace/deny gates
- source-level anti-evasion guards

## Reviews and Verification

Dual review and dual verification completed. The review and verification
findings were accepted and fixed:

- status/gate wording corrected;
- run-completion summary made unambiguous;
- pass parquet metrics regenerated with `.venv/bin/python` and `pyarrow`;
- closure artifacts added;
- raw run outputs confirmed ignored.

## Follow-On

Next package:
`20260708-laned-router-mn-corn-h4-routed-shape-attribution-001`.

The follow-on should attribute `mn_corn_h4` `sim_day_index=792`,
`lane_index=1` before any renewed `dx5` promotion attempt.
