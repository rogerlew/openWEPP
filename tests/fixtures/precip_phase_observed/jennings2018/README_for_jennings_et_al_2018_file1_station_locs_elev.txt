README

Within this folder are several files regarding precipitation phase partitioning (rain vs. snow) in the Northern Hemisphere. They are as follows:

1) jennings_et_al_2018_file1_station_locs_elev.csv: Station locations and elevations for observations in (2)

2) jennings_et_al_2018_file2_ppt_phase_met_observations.csv: Observations of precipitation phase and meteorological quantities from the stations in (1)

3) jennings_et_al_2018_file3_temp50_observed_by_station.csv: Calculated 50% rain-snow air temperature thresholds from stations with data presented in (2)

4) jennings_et_al_2018_file4_temp50_raster.tif: Map of gridded, modeled 50% rain-snow air temperature thresholds fit with a hyperbolic tangent

5) jennings_et_al_2018_file5_temp50_linregr_raster.tif: Map of gridded, modeled 50% rain-snow air temperature thresholds fit with linear regression in grid cells without enough rain and/or snow events to be modeled using the hyperbolic tangent in (4)

6) jennings_et_al_2018_file6_precipphase_station_observations_code.R: R code for computing the 50% rain-snow air temperature threshold using the station data in (2)

7) jennings_et_al_2018_file7_precipphase_phasemethods_code.R: R code for evaluating the performance of different precipitation phase methods using the station data in (2)

8) jennings_et_al_2018_file8_precipphase_merra_threshold_simulation_code.R: R code for creating the simulated 50% rain-snow air temperature threshold maps in (4) and (5)

9) jennings_et_al_2018_file9_precipphase_merra_snowfall_frequency_sensitivity_code.R: R code for analyzing the sensitivity of simulated snowfall frequency to 18 different precipitation phase methods

For full details of these data and methods please see Jennings et al. (2018) Nature Communications or contact Keith Jennings at the email address below.

2018-01-30
Keith Jennings
keith.jennings@colorado.edu