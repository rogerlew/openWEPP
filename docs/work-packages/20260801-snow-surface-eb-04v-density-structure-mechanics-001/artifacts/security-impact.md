# Security Impact

Status: `PASS / no material security impact`.

Evidence mode: `[Static] + [Ran]`.

The package adds no network access, secret handling, authentication,
authorization, deployment, unsafe Rust, dependency, or external write. The
cohort runner removes inherited `OPENWEPP_*` state and installs only the seven
frozen selectors before each subprocess. Runfiles, outputs, traces, and
provenance remain under explicit package-owned `target/` paths.

No external-authority suite posture or cohort binding changed, so the
authority-suite anti-evasion guards were not selected. Root `Cargo.toml`
changed only to register the package integration target; `cargo deny check`
passed advisories, bans, licenses, and sources with one pre-existing nonfailing
unused `MIT-0` allowance warning.
