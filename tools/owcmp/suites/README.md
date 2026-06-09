# owcmp Suite Manifests

This directory holds small declarative suite manifests for common comparator and
validation cohorts. They are meant to reduce prompt/context load: agents can name
a manifest path and run `tools/owcmp/owcmp env --manifest <path>` instead of
carrying long path inventories in chat.

Current manifest schema: `owcmp-suite-manifest-v1`.

Useful commands:

```bash
tools/owcmp/owcmp manifest list
tools/owcmp/owcmp manifest show --manifest tools/owcmp/suites/<suite>.json
tools/owcmp/owcmp env --manifest tools/owcmp/suites/<suite>.json
```

`cohort-inventory` manifests are preflight declarations. They intentionally do
not run a comparator by themselves because they identify run roots and expected
surfaces, not a complete baseline-vs-candidate comparison pair.
