# Line-Count Governance

Status: PASS.

The only executable package artifact is the independent Python calculator.
Production Rust line-count limits are unaffected.

- calculator: 792 lines, authority-oracle evidence rather than production Rust;
- vegetation authority integration test: 2,399 lines total, with V7 assertions
  factored into bounded helpers after a retained `too_many_lines (161/100)`
  retry failure; warnings-denied Clippy PASS;
- adjacent implementation test: 721 lines; only two mechanical `clone_from`
  corrections are owned by this package.
