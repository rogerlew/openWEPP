# Line-count governance

Static: no production Rust changes. One authorized administrative document-
structure test changes only its expected existing inventory count, 22 to 27.
Ran: wc -l tests/integration/advisory_linter_authority_contract.rs = 357;
no 2000/3000 threshold or exemption. Scope is the explicit user exception for
administrative path/structure tests; all atomic authority bindings unchanged.
Root + common always-read governance = 16,150 bytes (15.77 KiB).
