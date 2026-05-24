# CLI01 Sidecar Discovery Behavior Evidence

Status: complete
Evidence mode: Static + Ran

## Static
- Sidecar discovery is blind run-directory enumeration in
  `execute_hillslope_run` (`crates/openwepp-runner/src/lib.rs`).
- Required sidecar contracts are explicit and typed:
  - `frost -> frost.txt`
  - `snow -> snow.txt`
  - `wepp_ui -> wepp_ui.txt`
  - `pmetpara -> pmetpara.txt`
- Policy behavior is delegated through `openwepp-legacy-bridge` adapter:
  - strict unknown sidecar: hard-fail
  - compat unknown sidecar: warning + continue
  - missing required sidecar: hard-fail in both strict and compat

## Ran
Commands executed with `target/debug/openwepp-cli-hill` against CLI01 fixtures:

```text
STRICT unknown-sidecar case
exit=1
CLIHILL-E-008 sidecar adapter failure: LSB-E-009 (LSB-E-009 strict policy disallows unknown sidecar mystery_sidecar.txt)

COMPAT unknown-sidecar case
exit=0
sidecar-warning: LSB-W-002 ignored unknown sidecar mystery_sidecar.txt at /tmp/cli01_compat_unknown_GyDuLb/mystery_sidecar.txt
```

```text
Missing required sidecar (removed frost.txt)
policy=strict exit=1
CLIHILL-E-008 sidecar adapter failure: LSB-E-007 (LSB-E-007 missing required sidecar frost (frost.txt))

policy=compat exit=1
CLIHILL-E-008 sidecar adapter failure: LSB-E-007 (LSB-E-007 missing required sidecar frost (frost.txt))
```

Evidence summary:
- strict unknown-sidecar behavior matches typed hard-fail requirement.
- compat unknown-sidecar behavior matches warning requirement.
- missing required sidecars remain typed hard failures in both policies.
