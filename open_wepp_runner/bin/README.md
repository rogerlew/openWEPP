# open_wepp_runner/bin

Optional local binary drop-in directory for the Python `open_wepp_runner`
package.

Expected names:

- `open_wepp_runner` (launcher binary)
- `openwepp-cli-hill` (hillslope driver binary)
- release-tag binaries such as `openwepp_YYMMDD...` and `openwepp_YYMMDD..._hill`

If binaries are not present here, resolution falls back to `PATH` or explicit
environment-variable overrides.
