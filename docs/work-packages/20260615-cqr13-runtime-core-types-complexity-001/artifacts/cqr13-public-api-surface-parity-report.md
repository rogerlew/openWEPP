# CQR13 Public API Surface Parity Report

Status: complete.

Static: planned production edits are private helper extraction in a runtime
input core type module. No public API change is authorized.

Static: live metrics proved no production refactor was needed. Public API
surface parity is preserved because CQR13 made no production Rust edits.

Static: the target file still exposes `pub enum HillslopeRuntimeInputError` and
`pub const fn code(&self) -> &'static str` as before the package.
