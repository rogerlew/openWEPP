# Review Disposition

Status: complete.

## Review 1

Static self-review:

- Confirmed production direct call site no longer references
  `DirectPublicationDayInputBuilder`.
- Confirmed explicit shadow/cutover compatibility builders remain available
  outside production direct.
- Confirmed typed production builder hot-loop `build` body reads typed climate
  forcing and committed direct lane state, not runtime-surface maps.

## Findings

1. Finding: setup-time authority still uses seeded day-zero surfaces.
   Disposition: accepted as outside the hot-loop scope. It is documented as
   future static-authority migration, not hidden production-loop
   compatibility.
2. Finding: active material snow/frost authority is not fully migrated into
   this typed production path.
   Disposition: accepted fail-closed behavior. R7F only unblocked inert frost
   option bits; non-zero frost depth or frozen-water carry remains blocked.
3. Finding: `direct_publication/day_input_and_helpers.rs` exceeds line-count
   closure policy.
   Disposition: accepted inherited module debt for this package. R7F recorded
   line evidence and added source scans; mechanical split should be a separate
   package.
4. Finding: clippy `--all-targets` exposed an unrelated long CLI `run()`.
   Disposition: added a targeted `#[allow(clippy::too_many_lines)]` to the CLI
   entrypoint rather than refactoring argument parsing inside R7F.

Review outcome: no unresolved R7F closure findings.
