# Literature Synthesis

Status: complete

## Governing Conclusions

1. Dun et al. (2010) is the WEPP-specific implementation bridge, but it does
   not by itself authorize blindly enabling pinned-baseline `Qwet`. The paper
   describes v2010.1 frost improvements that include water migration and a
   moderate freezing-front potential, while the pinned baseline source disables
   the migration term with `frzftp = 0.0`. That conflict must be treated as a
   contract question.

2. Watanabe and Flury (2008), Azmatch et al. (2012), Ming et al. (2020), and
   Cheng et al. (2023) form a candidate family for frozen hydraulic
   conductivity. They support future `K_frozen(theta_liq, T, soil_params)`
   exploration, not an immediate scalar multiplier.

3. Kurylyk and Watanabe (2013) should be the theory review read before
   changing the frost solver. It frames the chain from soil water retention or
   SFCC to unfrozen water and hydraulic conductivity, and it records unsettled
   assumptions that matter for openWEPP.

4. Dall'Amico et al. (2011) and Kurylyk et al. (2014) should inform numerical
   gates. Future production work should preserve energy/latent-heat consistency
   and should consider analytical thaw benchmarks before field calibration.

5. Devoie et al. (2022) is a parameter/data source, not a runtime algorithm. It
   is useful for priors, texture checks, and uncertainty around SFCC choices.

6. Amankwah et al. (2021) is important but not first-order for ordinary upland
   WEPP erosion cases unless salinity, reclaimed/irrigated land, roadside salts,
   or arid soils are in scope.

## Recommended Physics Ladder

Future `GAP-SNOWFREEZE-002` work should proceed in this order:

1. Keep the current observation harness as the acceptance surface. Do not use
   compatibility frost output as the target.
2. Validate snow depth/density and residue insulation before attributing frost
   depth residuals to soil freezing physics.
3. Validate the no-migration heat-flow column (`Qwet = 0`) against the observed
   sites and simple analytical benchmark cases.
4. Add an experimental SFCC/unfrozen-water diagnostic path without changing
   production hydrology.
5. Compare candidate frozen hydraulic-conductivity models behind a research
   switch: Dun-style, Watanabe/Flury capillary bundle, SFCC-derived
   Azmatch/Ming, and Cheng-style impedance where ice segregation/open saturated
   conditions apply.
6. Only promote `Qwet` when residuals require water migration and the selected
   frozen-K/impedance model satisfies mass, energy, and observation gates.

## Open Questions for Follow-Up

- Does the current official WEPP branch still carry the `frzftp = 0.0`
  behavior, or did later releases revise the migration term?
- Can the request-only Dun Pullman/Morris observation series be acquired for
  direct WEPP-lineage validation?
- Which pilot observation sites have snow-depth quality sufficient to
  attribute frost residuals to soil physics rather than snow insulation?
- What minimal analytical benchmark subset from Kurylyk et al. (2014) should be
  encoded as openWEPP tests before field-data tuning?
- Does openWEPP need a salinity-aware SFCC mode, or should salinity remain an
  explicitly out-of-scope future extension?
