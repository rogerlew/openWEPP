# Rejected Execution V1

Status: `REJECTED BEFORE RESULTS`.

Evidence class: `Ran`.

Exact reviewed execution commit:
`9447f9b28653476e12d96c02b2b245cb2079f00e`.

Command:

```text
.venv/bin/python docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/tools/run_operator_reconciliation.py --expected-head 9447f9b28653476e12d96c02b2b245cb2079f00e
```

The command exited `1`. The paired enabled-evaluation lanes reached an
inactive no-snow/no-precipitation day for which the production hourly-forcing
provider correctly returned no typed array. The runner had allowed the
evaluation selector to bypass the inactive partition return and then raised:

```text
CLIHILL-E-011 ... direct production active snow partition requires typed winter hourly forcing for day 73
```

Mica Creek reported the first surfaced failure at day 73; the same failure
class occurred in paired lanes at all four sites. The exact ignored namespace
`target/snow_stage3_operator_reconciliation/` is retained and must not be
overwritten or reused. No compact result, execution receipt, retained manifest,
site summary, decision class, or scientific metric was produced.

Retained custody hashes:

- execution binary: `78996d78139776c0ee01bb2567b3964b89c0086ed9da81c86fd0ba539b13fef3`;
- retained v1 protocol: `1cf1174eca2eac57a63c80c9e5f6a964bc8066387ba093dde1f977bb45955901`;
- first Mica paired stderr:
  `1b5d4ca770a1af1a2988505e51a039609526628acb4f0dca4c45cd8bc679daa7`.

Disposition: this is an evaluation-observability lifecycle defect. It is not a
carrier or operator result. The contract-first amendment requires an enabled
inactive day to emit an empty schema-v6 record with 24
`operator_not_selected` statuses while disabled/default schema v4 remains
exact. Any corrected execution uses the new v2 namespace only after renewed
result-blind admission.
