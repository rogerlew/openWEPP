# Population Execution Evidence

Status: `PASS`

Evidence class: `Ran`

The package-required comparator runner built
`target/debug/openwepp-cli-hill` and recorded binary SHA-256
`0242c39fa26e9cbbd9461a36a4d6843b8adf0600fb72c215c349a454cbf66a50`.
It then executed the frozen matrix once with four workers.

- build: PASS in 1.245 s;
- anti-alias self-check: PASS in 0.083 s;
- population: PASS in 612.944 s;
- inventory: 12 lanes, 48 cells, 48 PASS, 0 FAIL;
- daily rows consumed: 761,212;
- former EB-04 failures: 24/24 complete;
- every WAT/trace pair exists, hashes, and has equal row count;
- every trace chronology is sequential from day zero.

The direct runner manifests prospectively bind the command, source, binary,
runfile, WAT, completion class, and day count. They do not hash the opt-in
research trace. `retained-output-evidence.md` therefore records the separate
forensic trace seal and its execution-window mtime/ctime proof without calling
it prospective attempt evidence.

Commands and complete console output are retained in `cmd1_cargo_build.log`,
`cmd2_self_check.log`, and `cmd3_execute.log`. Machine-readable results are in
`qualification-results.json` and `cell-qualification.csv`; transient runfiles,
WAT, traces, stdout, and stderr remain under
`target/snow_surface_eb04e_qualification/`.
