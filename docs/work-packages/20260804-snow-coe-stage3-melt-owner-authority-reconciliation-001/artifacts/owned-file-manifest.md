# Owned File Manifest

Status: scaffolded

Evidence mode: Static

Tracked ownership is exactly the package directory, both named snow contracts,
their lifecycle index, the authority-owning static contract test, the three
catalog/roadmap files, and the existing integration tests containing the stale
exact `contract_version: 125` marker. Those additional tests permit only the
mechanical `125 -> 126` replacement discovered by the first heavy profiles.
The ignored target namespace is package-local. All production Rust, fixtures,
references, observations, and every other test assertion are read-only.
