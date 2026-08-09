# Contract-Test Implementation Evidence

Evidence class: `Ran`

The focused test was changed before canonical contract edits to require version
4 registry metadata, caller-configuration/state invariants, independent
native-forest component fluxes, the canopy-loss poison rule, root-layer
demonstrations, Penman-Monteith nuance, and the new gap label.

Initial run:

```text
TMPDIR=/home/workdir/openWEPP/target/task-tmp \
  cargo nextest run --test vegetation_boundary_authority_contract --profile quick
10 tests run: 7 passed, 3 failed
```

The first attempt without `TMPDIR` did not compile because root `/tmp` had no
free space. The rerun reached the intended assertions and failed on the absent
version-4 contract text and registry date.

Post-amendment run:

```text
10 tests run: 10 passed, 0 skipped
```

No production implementation is tested or claimed.
