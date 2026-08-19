# Production exclusion baseline

At intake the V10/V9 real-consumer module is explicitly default-off and exposes no production selector, publication, or output API. Production direct publication and runtime selection remain outside the restart module. This package will guard unchanged selectors/defaults/publication sources and compare production state/output bytes with the shadow absent versus present.
