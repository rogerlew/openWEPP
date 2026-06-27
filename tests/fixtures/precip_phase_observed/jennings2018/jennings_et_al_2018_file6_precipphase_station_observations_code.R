#Script for analyzing station observations of precipitation phase
#Requires the file2_ppt_phase_observations.csv dataset

#If using this code, please cite Jennings et al. (2018) Nature Communications


#Keith Jennings
#keith.jennings@colorado.edu
#2018-01-22

#############Load required packages##################
#Note: These must already be installed on your system
library(dplyr)
library(tidyr)
library(minpack.lm)
library(reshape2)

############ Load Data ############
station_obs = read.csv("jennings_et_al_2018_file2_ppt_phase_met_observations.csv")

#Columns in station_obs:
#Station_ID			= numeric station identifier
#Date				= date (YYYY-MM-DD)
#Hour				= hour (H)
#Air_Temp			= air temperature (°C)
#Dewpoint			= dewpoint temperature (°C)
#RH					= relative humidity (%) calculated from Air_Temp and Dewpoint
#gridded_data_pres	= surface pressure from MERRA-2 reanalysis
#Prec_Type			= WMO precipitation code (see Dai (2001))
#Snow_Phase			= binary integer where 1 = snow and 0 = not snow
#Rain_Phase			= binary integer where 1 = rain and 0 = not rain

#Note: The following codes were use to split rain and snow
#Snow only codes: 22,70,71,72,73,74,75,76,77,78,85,86
#Rain only codes: 21,25,29,51,52,53,54,55,58,59,60,61,62,63,64,65,80,81,82,91

###############################################################################
###############################################################################
###############################################################################
#Calculate observed 50% rain-snow air temperature threshold per station

#Bin data by air temperature, relative humidity and surface pressure
#Generate the RH, Ps, and T bins
temp_seq = -8:8 #temperature bins
rh_seq=c(40,50,60,70,80,90,100) #RH bins 
p_seq = c(60,70,80,90,105) #pressure bins 

#Add bins to the dataset
station_obs$temp_bin <- cut(station_obs$Air_Temp, breaks = temp_seq, include.lowest = F, right = T)
station_obs$rh_bin <- cut(station_obs$RH, breaks = rh_seq, include.lowest = F, right = T)
station_obs$p_bin <- cut(station_obs$gridded_data_pres, breaks = p_seq, include.lowest = F, right = T)

#Create vector of unique stations
stations <- unique(filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & is.na(temp_bin) == F)$Station_ID)

#Calculate snow frequency for each stations
freq_station <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & 
         is.na(temp_bin) == F) %>% 
  group_by(Station_ID, temp_bin) %>% 
  summarise(freq = sum(Snow_Phase) / 
              (sum(Snow_Phase) + sum(Rain_Phase)),
            n_obs = length(Snow_Phase))

#Create numeric vector for temperature bins
temp_cuts_to_number <- data.frame("temp_bin" = levels(station_obs$temp_bin),
			                                  "temp" = seq(-7.5, 7.5, by = 1))

#Join frequency data to numeric temperature values
freq_station <- left_join(freq_station, temp_cuts_to_number, by = "temp_bin")


#Create blank data frame for 50% temperature threshold per cell
temp50 <- data.frame("Station_ID" = numeric(),
                       "temp50" = numeric())

#Calculate threshold by fitting hyperbolic tangent to each station's frequency data
#Note: Even with filtering, not every station has enough data to fit curve

for (i in 1:length(stations)){
  #Put data for one cell into temporary data frame
  tmpcell <- subset(freq_station, Station_ID == stations[i])
  
  #Identify the cell
  temp50[i, "Station_ID"] = as.character(unlist(tmpcell[1, "Station_ID"]))
  
  #Run if there are at least 14 entries
  #And at least 200 observations
  #Fewer entries means not enough temperature bins observed
  if (length(tmpcell$temp) >= 14 & sum(tmpcell$n_obs) > 200) {
    tryCatch(
      {freq_fit <- nlsLM(freq ~ #predicts snow frequency
                           a * (tanh( b* (temp - c)) - d), #as a function of air temperature and 4 fitting parameters
                         #tanh is the hyperbolic tangent (gives curve shape)
                         data = tmpcell, 
                         start = list(a = -45, b = 0.7, c = 1.2, d = 1)) #the starting values are from Dai (2008)
      #Calculate the 50% threshold temperature from the hyperbolic tangent equation
      #Note: atanh function is not used, but rather it's underlying equation is
      #atanh has difficulties with larger numbers
      #See: http://www.stat.umn.edu/macanova/htmlhelp/node30.htm
      temp50_2[i, "temp50"] = (0.5 * log((1 + (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4]))) / 
                                           (1 - (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4])))))/
        as.numeric(coef(freq_fit)[2]) + 
        as.numeric(coef(freq_fit)[3])
      })
  } else { #if not enough entries, enter temperature as NA
    temp50[i, "temp50"] = NA
  }
}



#################################################################################
#################################################################################
#################################################################################
#Snow frequency curves by RH and Ps

#################################################################################
#Calculate frequency by RH bin
freq_rh <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & is.na(temp_bin) == F) %>% 
  group_by(rh_bin, temp_bin) %>% 
  summarise(freq_by_rh = sum(Snow_Phase) / 
              (sum(Snow_Phase) + sum(Rain_Phase)),
            n_obs = length(Snow_Phase))

#Calculate SD and standard error per RH bin
freq_rh_sd <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & is.na(temp_bin) == F) %>% 
  group_by(rh_bin, temp_bin) %>% 
  summarise(sd_by_rh = sd(Snow_Phase),
            n_obs = length(Snow_Phase),
            se_by_rh = sd_by_rh/sqrt(n_obs))

#Combine with temp cuts
freq_rh <- left_join(freq_rh, temp_cuts_to_number, by = "temp_bin")

#Calculate 50% threshold by RH bin
freq_rh_bins <- unique(freq_rh$rh_bin)
temp50_byRH = data.frame(rh_bin = character(),
                         temp50 = numeric())

for(i in seq_along(freq_rh_bins)){
  #First fit hyperbolic tangent to data
  freq_fit <- nlsLM(freq_by_rh ~ #predicts snow frequency
                      a * (tanh( b* (temp - c)) - d), #as a function of air temperature and 4 fitting parameters
                    #tanh is the hyperbolic tangent (gives curve shape)
                    data = filter(freq_rh, rh_bin == freq_rh_bins[i]), 
                    start = list(a = -45, b = 0.7, c = 1.2, d = 1)) #the starting values are from Dai (2008)
  
  #Calculate the 50% threshold temperature from the hyperbolic tangent equation
  #Note: atanh function is not used, but rather it's underlying equation is
  #atanh has difficulties with larger numbers
  #See: http://www.stat.umn.edu/macanova/htmlhelp/node30.htm
  temp50_byRH[i, "temp50"] = (0.5 * log((1 + (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4]))) / 
                        (1 - (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4])))))/
    as.numeric(coef(freq_fit)[2]) + 
    as.numeric(coef(freq_fit)[3])
}
temp50_byRH$rh_bin <- freq_rh_bins
temp50_byRH$rh_bin_mid <- seq(45, 95, by = 10)


#################################################################################
#Calculate frequency by pressure bin
freq_p <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & is.na(temp_bin) == F) %>% 
  group_by(p_bin, temp_bin) %>% 
  summarise(freq_by_p = sum(Snow_Phase) / 
              (sum(Snow_Phase) + sum(Rain_Phase)),
            n_obs = length(Snow_Phase))

#Calculate standard deviation and standard error by pressure bin
freq_p_sd <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & is.na(temp_bin) == F) %>% 
  group_by(p_bin, temp_bin) %>% 
  summarise(sd_by_p = sd(Snow_Phase),
            n_obs = length(Snow_Phase),
            se_by_p = sd_by_p/sqrt(n_obs))

#Combine with temp cuts
freq_p <- left_join(freq_p, temp_cuts_to_number, by = "temp_bin")

#Calculate 50% threshold by Ps bin
freq_p_bins <- unique(freq_p$p_bin)
temp50_byP = data.frame(p_bin = character(),
                         temp50 = numeric())

for(i in seq_along(freq_p_bins)){
  #First fit hyperbolic tangent to data
  freq_fit <- nlsLM(freq_by_p ~ #predicts snow frequency
                      a * (tanh( b* (temp - c)) - d), #as a function of air temperature and 4 fitting parameters
                    #tanh is the hyperbolic tangent (gives curve shape)
                    data = filter(freq_p, p_bin == freq_p_bins[i]), 
                    start = list(a = -45, b = 0.7, c = 1.2, d = 1)) #the starting values are from Dai (2008)
  
  #Calculate the 50% threshold temperature from the hyperbolic tangent equation
  #Note: atanh function is not used, but rather it's underlying equation is
  #atanh has difficulties with larger numbers
  #See: http://www.stat.umn.edu/macanova/htmlhelp/node30.htm
  temp50_byP[i, "temp50"] = (0.5 * log((1 + (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4]))) / 
                                          (1 - (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4])))))/
    as.numeric(coef(freq_fit)[2]) + 
    as.numeric(coef(freq_fit)[3])
}
temp50_byP$p_bin <- freq_p_bins
temp50_byP$p_bin_mid <- c(65,75,85,97.5)


#################################################################################
#################################################################################
#################################################################################
#Snow frequency curves by Ts, Tw, and Td

#Calculate wet bulb temperature (dew point reported in initial dataset)
#Equation from Stull (2011)
station_obs$t_wet <- with(station_obs,
                        Air_Temp * atan(0.151977 *( (RH + 8.313659) ^ 0.5)) +
                          atan(Air_Temp + RH) -
                          atan(RH - 1.676331) +
                          ((0.00391838 * (RH^ 1.5)) * atan(0.023101 * RH)) -
                          4.86035)

#Calculate snowfall frequencies for all temperature types
freq_t_air <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & 
                       is.na(temp_bin) == F ) %>%
  group_by(temp_bin) %>% 
  summarise(freq_by_t_air = sum(Snow_Phase) / 
              (sum(Snow_Phase) + sum(Rain_Phase)),
            n_obs = length(Snow_Phase))
freq_t_wet <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & 
                       is.na(t_wet_bin) == F ) %>%
  group_by(t_wet_bin) %>% 
  summarise(freq_by_t_wet = sum(Snow_Phase) / 
              (sum(Snow_Phase) + sum(Rain_Phase)),
            n_obs = length(Snow_Phase))
freq_t_dew <- filter(station_obs, is.na(rh_bin) == F & is.na(p_bin) == F & 
                       is.na(t_dew_bin) == F ) %>%
  group_by(t_dew_bin) %>% 
  summarise(freq_by_t_dew = sum(Snow_Phase) / 
              (sum(Snow_Phase) + sum(Rain_Phase)),
            n_obs = length(Snow_Phase))

#Join all frequencies into one dataset
freq_t_all <- left_join(freq_t_air, freq_t_wet,
                        by = c("temp_bin" = "t_wet_bin"))
freq_t_all <- left_join(freq_t_all, freq_t_dew,
                        by = c("temp_bin" = "t_dew_bin"))
freq_t_all <- left_join(freq_t_all, temp_cuts_to_number, 
                        by= "temp_bin")

#Melt the data
freq_t_all_melt <- melt(dplyr::select(freq_t_all, 
                                      temp, freq_by_t_air, freq_by_t_wet, freq_by_t_dew),
                        id.vars = "temp", variable.name = "type", value.name = "freq")

#Calculate 50%, 10%, and 90% thresholds by temp type
temp_types <- unique(freq_t_all_melt$type)
temp50_t_all = data.frame(type = character(),
                          temp50 = numeric(),
                          temp90 = numeric(),
                          temp10 = numeric())

for(i in seq_along(temp_types)){
  #First fit hyperbolic tangent to data
  freq_fit <- nlsLM(freq ~ #predicts snow frequency
                      a * (tanh( b* (temp - c)) - d), #as a function of air temperature and 4 fitting parameters
                    #tanh is the hyperbolic tangent (gives curve shape)
                    data = filter(freq_t_all_melt, type == temp_types[i]), 
                    start = list(a = -45, b = 0.7, c = 1.2, d = 1)) #the starting values are from Dai (2008)
  
  #Calculate the 50% threshold temperature from the hyperbolic tangent equation
  #Note: atanh function is not used, but rather it's underlying equation is
  #atanh has difficulties with larger numbers
  #See: http://www.stat.umn.edu/macanova/htmlhelp/node30.htm
  temp50_t_all[i, "temp50"] = (0.5 * log((1 + (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4]))) / 
                                           (1 - (0.5/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4])))))/
    as.numeric(coef(freq_fit)[2]) + 
    as.numeric(coef(freq_fit)[3])
  temp50_t_all[i, "temp90"] = (0.5 * log((1 + (0.9/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4]))) / 
                                           (1 - (0.9/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4])))))/
    as.numeric(coef(freq_fit)[2]) + 
    as.numeric(coef(freq_fit)[3])
  temp50_t_all[i, "temp10"] = (0.5 * log((1 + (0.1/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4]))) / 
                                           (1 - (0.1/as.numeric(coef(freq_fit)[1]) + as.numeric(coef(freq_fit)[4])))))/
    as.numeric(coef(freq_fit)[2]) + 
    as.numeric(coef(freq_fit)[3])
}
temp50_t_all$type <- temp_types

