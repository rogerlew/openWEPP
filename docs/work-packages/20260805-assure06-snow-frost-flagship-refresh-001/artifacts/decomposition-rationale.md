# Amendment Module Decomposition Rationale

Evidence class: Static

`crates/openwepp-assurance/src/v2/amendment.rs` is 2,887 lines at review time,
which triggers the repository's 2,000-line warning but remains below the
3,000-line closure threshold. ASSURE-06 keeps the bounded change in this module
because it extends the existing manifest-adoption and lifecycle-entry paths and
reuses their confinement, reset, and successor-transaction helpers. Splitting
the module inside a scientific-report refresh would mix a broad mechanical
refactor with the reviewed behavior correction.

Follow-on intent: before the next material amendment-workflow expansion,
scaffold a dedicated mechanical package to separate report-source adoption,
lifecycle requests, and principal/role amendment orchestration into focused
modules without changing public APIs, identity algorithms, or transaction
semantics. The package must preserve current focused contracts and run the full
assurance amendment/publication suite before closure.
