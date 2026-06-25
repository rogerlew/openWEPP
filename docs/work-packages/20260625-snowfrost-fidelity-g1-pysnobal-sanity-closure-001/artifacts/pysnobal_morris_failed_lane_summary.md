# PySnobal Site Summary

- PySnobal Python: `/tmp/pysnobal-g0-venv/bin/python`
- PySnobal path: `/home/workdir/pysnobal`
- Route policy: `all-lanes`
- Route recommendation: `HOLD-PYSNOBAL-SANITY-FAILURE`
- Failed lane count: `1`

- Site filters: `['site4_ggd498_morris_mn']`
- Lane filters: `['tg_neg0p5c_zg0p10m']`
- Window: `None` to `None`

| Site | Lane | Status | Max SWE kg/m2 | Max depth m | Paired obs | Mean abs obs residual m | OpenWEPP paired | Mean abs Py-OpenWEPP depth m | Reason |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| site4_ggd498_morris_mn | tg_neg0p5c_zg0p10m | FAIL |  |  | 0 |  | 0 |  | PySnobal failed with exit code 1: <string>:7: DeprecationWarning: 
Pyarrow will become a required dependency of pandas in the next major release of pandas (pandas 3.0),
(to allow more performant data types, such as the Arrow string type, and better interoperability with other libraries)
but was not found to be installed on your system.
If this would cause problems for you,
please provide us feedback at https://github.com/pandas-dev/pandas/issues/54466
        
[pysnobal/c_snobal/libsnobal/sati.c:17] ERROR: Input temperature (tk): -153.450833 is less than zero |
