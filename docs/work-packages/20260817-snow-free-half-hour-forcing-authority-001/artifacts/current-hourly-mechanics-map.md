# Current Hourly Mechanics Map

Static:

- daily `radly` remains Langleys/day and converts once to `radmj`;
- `simimpl28_radcur` computes horizontal hourly extraterrestrial energy weights;
- current `hr_tmp` exports slope-adjusted `estrad * radcur/rpoth`, which is not
  the horizontal global parent required by Weiss--Norman;
- the authoritative horizontal parent is always `radmj * radcur/rpoth`; the
  legacy near-isothermal `radmj/24` behavior is not this provider;
- `simimpl28_hrtmp` supplies hourly air temperature;
- `sunmap.cloud_fraction` supplies one daily effective cloud fraction;
- phase/RH/hydrometeor temperature come from the existing selected SIMIMPL28
  Harder--Pomeroy path;
- breakpoint `timem/intsty` vectors remain the finer precipitation support.

Production winter-trigger behavior and diagnostic complete-row behavior remain
distinct and must retain byte parity after common-kernel extraction.
