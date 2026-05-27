# REFACTOR005 Modularization Plan Report

Status: complete
Evidence mode: static
Date: 2026-05-27

## Static
Mechanical extraction plan for `03_kernel_support.rs`:
1. Split at the second `impl Wb11HydrologyKernel` boundary.
2. Move lines 1..(second impl - 1) to
   `03_kernel_support_00_support_helpers.rs`.
3. Move lines (second impl)..EOF to
   `03_kernel_support_01_kernel_phases.rs`.
4. Replace `03_kernel_support.rs` with two `include!` statements preserving
   source order.
5. Run required gate suite to prove no semantic drift.

This keeps symbol visibility and compile ordering identical while reducing the
single-file maintenance burden.
