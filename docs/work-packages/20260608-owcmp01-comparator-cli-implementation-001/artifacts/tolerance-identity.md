# Tolerance Identity Evidence

Status: complete
Evidence mode: Ran

Command:

```bash
sha256sum \
  tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json \
  tools/owcmp/configs/pl14s_wat_tolerances.json \
  tools/legacy_comparison_suite/requirements.in \
  tools/owcmp/requirements.in \
  tools/legacy_comparison_suite/requirements.lock.txt \
  tools/owcmp/requirements.lock.txt
```

Observed hashes:

```text
dfabae701af645b5e44fbb0c233daa906610e9dd13e687484132453e58bcfb57  tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json
dfabae701af645b5e44fbb0c233daa906610e9dd13e687484132453e58bcfb57  tools/owcmp/configs/pl14s_wat_tolerances.json
3c400db7e61e577eefbf08defd52c16f9ac9bb7e2d858d272ce0696c5a7399cb  tools/legacy_comparison_suite/requirements.in
3c400db7e61e577eefbf08defd52c16f9ac9bb7e2d858d272ce0696c5a7399cb  tools/owcmp/requirements.in
6a7438fdd001710d8abf375f52ce1b56e9339186473f29f36b9a85815145f4c0  tools/legacy_comparison_suite/requirements.lock.txt
6a7438fdd001710d8abf375f52ce1b56e9339186473f29f36b9a85815145f4c0  tools/owcmp/requirements.lock.txt
```

Byte-identity checks:

```text
tolerance_config_cmp=0
requirements_in_cmp=0
requirements_lock_cmp=0
```

Conclusion: the PL14S WAT tolerance profile and Python dependency files copied
into `tools/owcmp` are byte-identical to the active legacy-suite inputs.
