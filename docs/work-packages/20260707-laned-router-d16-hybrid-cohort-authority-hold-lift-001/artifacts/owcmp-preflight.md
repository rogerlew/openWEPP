# owcmp Preflight

Status: EXECUTED-HOLD-ROUTE-COEFFICIENT-AUTHORITY. Evidence mode: Ran.

## Environment Preflight

All selected manifests pass `owcmp env --manifest`.

```text
tools/owcmp/owcmp env --manifest tools/owcmp/suites/minnesota-corn-ksflag1.json
exit 0
owcmp env: PASS
manifest minnesota-corn-ksflag1: PASS

tools/owcmp/owcmp env --manifest tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json
exit 0
owcmp env: PASS
manifest n-idaho-single-ofe-ksflag0: PASS

tools/owcmp/owcmp env --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json
exit 0
owcmp env: PASS
manifest wa-cascades-mofe-ksflag0: PASS
```

## Executable Suite Preflight

All selected manifests fail closed under `manifest run` because they are
inventory declarations, not active plain-vs-hybrid comparison pairs.

```text
tools/owcmp/owcmp manifest run --manifest tools/owcmp/suites/minnesota-corn-ksflag1.json
exit 1
owcmp cohort inventory manifests are preflight declarations; use `owcmp env --manifest <path>` or `owcmp manifest show --manifest <path>`

tools/owcmp/owcmp manifest run --manifest tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json
exit 1
owcmp cohort inventory manifests are preflight declarations; use `owcmp env --manifest <path>` or `owcmp manifest show --manifest <path>`

tools/owcmp/owcmp manifest run --manifest tools/owcmp/suites/wa-cascades-mofe-ksflag0.json
exit 1
owcmp cohort inventory manifests are preflight declarations; use `owcmp env --manifest <path>` or `owcmp manifest show --manifest <path>`
```

## Result

The owcmp roots are discoverable and present, but they cannot produce the
promotion evidence this package needs until source-authorized active inputs and
an executable suite manifest exist.
