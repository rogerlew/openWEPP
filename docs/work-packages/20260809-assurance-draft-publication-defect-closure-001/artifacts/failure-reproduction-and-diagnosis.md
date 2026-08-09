# Failure Reproduction And Diagnosis

Evidence class: `Ran`

## Invalid Scratch Topology

Command:

```bash
TMPDIR=/home/workdir/openWEPP/target/task-tmp \
  cargo nextest run --test assurance_v2_publication_contract \
  draft_subject_root_is_stable_but_cannot_publish --profile quick
```

Result: `FAIL`, exit `100`, one test run. The strengthened assertion exposed the
actual typed error:

```text
DRAFT publication returned the wrong invalid-state error:
staging and repository roots must be unrelated
```

The temporary staging directory was a descendant of the repository because
Rust's `std::env::temp_dir()` honors `TMPDIR`. `validate_roots` deliberately
runs before report-context loading and correctly rejected this topology. This
is a confinement success, not a production publication defect.

## Corrected External Scratch Topology

Command:

```bash
TMPDIR=/home/workdir/openwepp-task-tmp \
  cargo nextest run --test assurance_v2_publication_contract \
  draft_subject_root_is_stable_but_cannot_publish --profile quick
```

Result: `PASS`, exit `0`, one test run in 2.547 seconds. The production entry
point reached `validate_publishable`, returned the explicit typed DRAFT
rejection, and created no catalog, snapshot, or receipt.

## Disposition

No production Rust correction is authorized or warranted. The closure is to:

1. use an external high-capacity `TMPDIR` for assurance/full-workspace gates;
2. retain the root-confinement invariant unchanged; and
3. retain the regression diagnostic enhancement that prints the unexpected
   error and verifies the snapshot/receipt root remains empty.
