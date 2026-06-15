# Function Length Before

Static: `HEAD:crates/openwepp-runner/src/watershed_wat.rs` had
`read_batch_into` at lines `380-524`, inclusive.

Static: baseline function body span was `145` lines, with
`#[allow(clippy::too_many_lines)]` at line `379`.

Static: target file baseline line count was `908`.

Ran:

```text
git show HEAD:crates/openwepp-runner/src/watershed_wat.rs | nl -ba | sed -n '372,535p'
git show HEAD:crates/openwepp-runner/src/watershed_wat.rs | wc -l
```

Disposition: baseline suppression and function length confirmed.
