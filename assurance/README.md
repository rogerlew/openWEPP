# Scientific Assurance Transition Source

Status: v1 retired; zero public reports

The v1 status-first dossier publisher and SNOTEL candidate were removed by
ASSURE-03 after exact source, generated, review, and compiler provenance was
bound to Git commit `3352388465f8b288aed4636e8f9752ca6c1cceb9` and the package
content manifest. The retirement concerns publication architecture, not the
quality or extent of openWEPP snow/frost science.

This directory now contains only the neutral transition catalog, its static
public template, and a dormant export with zero documents. The retained CLI is
deliberately fail-closed: it rejects any nonempty legacy dossier catalog. V2
manuscript sources and tooling belong to ASSURE-04 and are not present here.

From the repository root:

```bash
cargo run --quiet -p openwepp-assurance -- validate --all
cargo run --quiet -p openwepp-assurance -- plan --all
cargo run --quiet -p openwepp-assurance -- build --all
cargo run --quiet -p openwepp-assurance -- check --all
```

An explicit release operation may add `--snapshot <id> --snapshot-root <path>`
only after the release transition preflight passes. Ordinary CI never invokes
snapshot mode. Historical recovery is audit-only and does not authorize v1
publication, export, snapshotting, or vendoring.
