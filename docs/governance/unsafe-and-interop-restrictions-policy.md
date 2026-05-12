# Unsafe Code and Interop Restrictions Policy

- **Status:** Active
- **Date:** 2026-05-12

## Purpose

Define hard restrictions for `unsafe` Rust and foreign-language interoperability
in openWEPP.

## Scope

Applies to all Rust crates and modules in openWEPP, including FFI boundaries to
legacy WEPP/Fortran-era artifacts and C-compatible adapters.

## Unsafe code restrictions

1. Safe Rust is the default. Crates should use `#![deny(unsafe_code)]` unless
   there is a documented boundary exception.
2. `unsafe` is allowed only in narrowly scoped boundary modules (for example
   FFI adapters or low-level memory/ABI glue) and not in high-level scientific
   orchestration code.
3. Every `unsafe` block must be minimal and include a `// SAFETY:` comment that
   states the exact invariants being relied on.
4. Every public `unsafe fn` must document a `# Safety` contract that callers
   must satisfy.
5. `unsafe` convenience usage is prohibited. If a safe standard-library or
   crate API can implement the behavior, use it.
6. `std::mem::transmute`, `static mut`, and unsafe attributes affecting symbol
   linkage/export (`no_mangle`, `export_name`, `link_section`) are prohibited in
   production paths unless there is a documented boundary necessity and review
   rationale.

## Interop restrictions

1. All `extern` blocks must be explicit-ABI and declared as `unsafe extern`.
2. For stable cross-language boundaries, use `extern "C"` or
   platform-specific `extern "system"` only.
3. `extern "Rust"` is not permitted for durable cross-language contracts
   because Rust ABI stability is not guaranteed.
4. Interop data types must use explicit layout contracts (`#[repr(C)]` or
   `#[repr(transparent)]` where applicable).
5. Do not expose Rust-specific container/string/reference types directly over
   FFI boundaries (`String`, `Vec<T>`, `&str`, slices, trait objects, `&CStr`).
   Use pointer/length or C-compatible forms and wrap with safe Rust APIs.
6. Unwinding across non-unwind ABIs is prohibited. FFI boundary functions must
   prevent Rust panics from crossing into foreign runtimes (for example with
   boundary wrappers and typed error returns).

## Review and evidence requirements

1. Any new `unsafe` region requires explicit code-review attention on soundness
   invariants.
2. Boundary-signature changes require contract documentation updates in the same
   change set.
3. Unsafe/interop changes must include targeted tests for invariants and error
   behavior.

## Sources

- Rust Reference: External blocks  
  https://doc.rust-lang.org/reference/items/external-blocks.html
- Rust Reference: Unsafety  
  https://doc.rust-lang.org/reference/unsafety.html
- Rust Reference: Type layout  
  https://doc.rust-lang.org/reference/type-layout.html
- Rust Reference: Panic and FFI unwinding  
  https://doc.rust-lang.org/reference/panic.html
- Rustonomicon: FFI  
  https://doc.rust-lang.org/nomicon/ffi.html
- Rustonomicon: Safe and unsafe meaning (`forbid(unsafe_code)`)  
  https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html
