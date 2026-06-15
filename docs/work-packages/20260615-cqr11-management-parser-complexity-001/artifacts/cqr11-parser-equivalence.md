# CQR11 Parser Equivalence

Status: complete.

Static: public parser entry points remain `parse_management_from_path` and
`parse_management_from_str`; their signatures were not changed.

Static: public structs, enums, and error variants in `management.rs` were not
renamed, removed, or reshaped.

Static: `parse_yearly_perennial` still reads header fields in the same order:
`jdharv`, `jdplt`, `jdstop`, `rw`, `mgtopt`.

Static: yearly perennial branch behavior is preserved:

- `mgtopt == 1` reads `ncut` and one `cutday` record per cut.
- `mgtopt == 2` reads `ncycle` and one four-token `graze_cycle` record plus
  `gday` and `gend` per cycle.
- `mgtopt == 3` emits empty cut and grazing vectors.
- legacy datver rejects `mgtopt` outside `1..3` with `MAN-E-004`.
- 2016-plus datver accepts the domain `1..7` but still rejects `4..=7` with
  the existing parser-support `MAN-E-004` message.

Ran: focused parser tests covered accepted and rejected perennial branch
behavior before and after production refactor:

```console
cargo test --test infile_management_parser_contract perennial -- --nocapture
```

Result: exit `0`, `9` passed in both focused runs.

Ran: full workspace tests passed after the refactor:

```console
cargo test --workspace
```

Result: exit `0`.
