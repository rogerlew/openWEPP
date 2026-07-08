# Gate Results

Status: `executed-complete`

Evidence mode: `Ran:` local gates and `Static:` source review.

| Gate | Status | Evidence |
|------|--------|----------|
| Release CLI build | PASS | `cargo build --release -p openwepp-runner --bins`. |
| Current-main producer proof | PASS | `target/release/openwepp-cli-hill` on `tests/fixtures/erosion_multi_ofe_p102`: pass rows `3652`, `sum(tdet)=41531.85795763501`, `sum(tdep)=29195.4647928195`, all `sedcon_*` sums nonzero. |
| Full W7R fixture serial run | PASS | `openwepp-cli-watershed` on `tests/fixtures/watershed/p102-sediment-active/runs`, `--jobs 1`, `wall=0:00.78`. |
| Full W7R fixture parallel run | PASS | Same fixture, `--jobs 4`, `wall=0:00.74`. |
| Serial/parallel public parquet identity | PASS | All 14 required outputs have `schema_delta=0`, `row_delta=0`. |
| Nonzero public sediment | PASS | `totalwatsed3`: `tdet=584.2332653870001`, `tdep=282.14618621700004`, `sed_del=0.08391307754719238`. |
| Independent sediment reconstruction | PASS | Focused test parses generated HBP latest event and proves `totalwatsed3.tdet/tdep` match HBP payload; hourly sediment sum equals `tdet - tdep`; `sed_del` matches EBE routed sediment yield and is not a detachment-minus-deposition alias. |
| Source guard | PASS | `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity` plus existing typed-frame public path guards. |
| Fixture checksum manifest | PASS | `(cd tests/fixtures/watershed/p102-sediment-active && sha256sum -c input-manifest.sha256)`. |
| Focused W7R test | PASS | `cargo test -p openwepp-runner --test watershed_cli_behavior_contract wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity -- --nocapture`: `1 passed`. |
| `cargo fmt --check` | PASS | Ran after rustfmt. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Finished `dev` profile with no warnings. |
| `cargo nextest run --workspace --profile full` | PASS | `1438` tests run: `1438` passed, `3` skipped; elapsed `591.493s`. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| Scoped docs lint | PASS | `markdown-doc lint --path ...`: `42 files validated, 0 errors, 0 warnings`. |
| `git diff --check` | PASS | Clean. |

Complete closure is claimed with all required W7R gates green.
