# Function Length After

Static: working-tree `read_batch_into` is at lines `482-495`, inclusive.

Static: after refactor function span is `14` lines and no
`#[allow(clippy::too_many_lines)]` appears in `watershed_wat.rs`.

Static: target file after line count is `1177`, below the 2000-line WARN
threshold.

Ran:

```text
nl -ba crates/openwepp-runner/src/watershed_wat.rs | sed -n '476,505p'
rg -n "allow\(clippy::too_many_lines\)|fn read_batch_into" crates/openwepp-runner/src/watershed_wat.rs
wc -l crates/openwepp-runner/src/watershed_wat.rs
```

Disposition: function-length target closed.
