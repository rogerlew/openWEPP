#Script for importing MERRA2 data and predicting precip phase
#using 18 different precipitation phase methods

#If using this code, please cite Jennings et al. (2018) Nature Communications

#Keith Jennings
#keith.jennings@colorado.edu
#2018-01-22

#Import libraries
library(raster)
library(rgdal)
library(plyr)     #plyr should always be loaded before dplyr
library(dplyr)
library(reshape2)
library(tidyr)
library(foreach)
library(doMC)
library(minpack.lm)
library(ncdf4)

#Note: this script requires you have MERRA daily reanalysis data on your local machine in two folders
#called "met" and "precipitation"

#Each MERRA met file has 13 variables
#CLDPRS cloud_top_pressure
#CLDTMP cloud_top_temperature
#H850 height_at_850_hPa
#PS surface_pressure
#Q850 specific_humidity_at_850_hPa
#QV10M 10-meter_specific_humidity
#QV2M 2-meter_specific_humidity
#SLP sea_level_pressure
#T10M 10-meter_air_temperature
#T2M 2-meter_air_temperature
#T2MWET wet_bulb_temperature_at_2_m
#T850 air_temperature_at_850_hPa
#time_bnds

#Note: Each MERRA precip file has 3 variables
#PRECSNO snowfall
#PRECTOTCORR total_precipitation
#time_bnds

#Note: The MERRA met and precip files are daily averages
#Precip is given in kg m-2 s-1 (multiply by 86400 s to get mm per day)

#Note: MERRA met is only for northern hemisphere and precip is global
#Follow subset routine so the data match spatially

#Note: Running this code requires a sufficient amount of processing power
#It was originally run on a 2015 MacBook pro with a 2.8 GHz Intel Core i7 processor
#and 16 GB of memory

#Assign the number of cores for parallelization
registerDoMC(cores = 4)

#Set working directory
setwd("~/merra/")

#List all met and ppt files
metfiles <- list.files(path = "met/")
pptfiles <- list.files(path = "precipitation/")

#Import the T50 map and change extent to match MERRA data
#Requires the the simulated 50% rain-snow temperature threshold map available on Data Dryad
t50_map = raster("../temp50_2_raster_merged.tif")
e <- extent(-180.3125, 179.6875, 0.25, 90.25)
t50_map <- extend(t50_map, e)

#Create a land/not land raster because we are only interested precip over land
#Load land polygon from NaturalEarthData (http://www.naturalearthdata.com/)
land_shp <- readOGR(dsn = "../../../../mapping/world_land/ne_110m_land/",
                    layer = "ne_110m_land")
#Reclassify raster by land polygon
land_map <-rasterize(land_shp,   #Rasterize the land_shp vector file
                     t50_map)    #With spacing and extent from t50_map
land_map <- overlay(stack(land_map), fun = function(x) {
  ifelse(x >= 1,
         1,
         NA)})   #Convert all land areas to 1 and ocean to NA


##############################################################################################
#1 Import all relevant variables (ppt, air temp, surface press, specific humidity, wet bulb temp)
#And designate each precipitation event as rain or snow using multiple methods


#Assign precipitation depth threshold (i.e. ppt must be greater than this amount for day to count)
ppt_thresh = 1


#Make function to combine parallel loop outptut
#Must be done because by default foreach can only output one list entry
#Normally not a problem, but I am computing precip phase with multipl algorithms
comb <- function(x, ...) {
  lapply(seq_along(x),
         function(i) c(x[[i]], lapply(list(...), function(y) y[[i]])))
}


#Loop through the years in standard loop, then do data manipulation in parallel loop
#Computer does not have enough memory to run all years in one parallel loop
#This update made on 2017-11-01

#Make vector of years (used for looping through met and ppt data)
data_yrs <- substr(x = metfiles, start = 32, stop = 35)
data_yrs <- unique(data_yrs)

#Initialize frequency lists
freq_ann.ls <- list()
freq_ssn.ls <- list()  
freq_ann_NH.ls <- list()
freq_ssn_NH.ls <- list()

for(k in 1:length(data_yrs)){
  metfiles.tmp <- 
    subset.default(x = metfiles, 
                   subset = as.numeric(substr(x = metfiles, start = 32, stop = 35)) == data_yrs[k])
  pptfiles.tmp <- 
    subset.default(x = pptfiles, 
                   subset = as.numeric(substr(x = pptfiles, start = 33, stop = 36)) == data_yrs[k])
  
  #Initialize list for precipitation phase
  phase.master = list()
  
  ############Begin parallel loop###################
  #Run parallel loop to partition precipitation phase
  #Per day per MERRA cell
  phase.master <-
    foreach(i = 1:length(metfiles.tmp), .errorhandling = "pass",
            .combine='comb', .multicombine=TRUE,
            .init=list(list(), list(), list(), list(), list(), list(),
                       list(), list(), list(), list(), list(), list(),
                       list(), list(), list(), list(), list(), list(),
                       list(), list(), list(), list(), list())) %dopar% { 
                         
                         #######################################################################################
                         #Import data layers from MERRA files
                         #Temperature
                         temp_k = raster(paste("met", metfiles.tmp[i], sep = "/"), #import the netcdf file
                                         varname = "T2M") * 1 #and choose variable of interest (T2M = 2 M air temperature in Kelvin)
                         #The "* 1" makes a single-band raster
                         temp_c = temp_k - 273.15 #make a temperature raster in kelvin
                         
                         #Precipitation (kg m-2 s-1)
                         ppt = raster(paste("precipitation", pptfiles.tmp[i], sep = "/"), 
                                      varname = "PRECTOTCORR") * 1
                         ppt = crop(ppt, extent(-180.3125, 179.6875, 0.25, 90.25))
                         ppt = ppt* 86400 #multiply by 86400 s to get mm per day
                         
                         #Surface pressure (Pa)
                         press = raster(paste("met", metfiles.tmp[i], sep = "/"), 
                                        varname = "PS") * 1
                         
                         #Wet bulb temperature (K converted to °C)
                         temp_wet = (raster(paste("met", metfiles.tmp[i], sep = "/"), 
                                            varname = "T2MWET") * 1) - 273.15
                         
                         #Specific humidity (kg kg-1)
                         spec_hum = raster(paste("met", metfiles.tmp[i], sep = "/"), 
                                           varname = "QV2M") * 1
                         
                         #Calculate relative humidity (%)
                         #http://earthscience.stackexchange.com/questions/2360/how-do-i-convert-specific-humidity-to-relative-humidity
                         rh = 0.263 * press * spec_hum * 
                           (exp((17.67 * (temp_k - 273.15))/(temp_k - 29.65))^(-1))
                         #Set all values > 100% to 100%
                         rh = overlay(stack(rh), fun =   
                                        function(x) { 
                                          ifelse( x > 100, 100, x) 
                                        } )
                         
                         #Calculate dew point temperature (°C)
                         #http://earthscience.stackexchange.com/questions/2360/how-do-i-convert-specific-humidity-to-relative-humidity
                         temp_dew = 243.04 * (log(rh / 100) + ((17.625 * temp_c) / (243.04 + temp_c))) /
                           (17.625 - log(rh /100) - ((17.625 * temp_c) / (243.04 + temp_c)))
                         
                         #######################################################################################
                         #Precipitation phase algorithms
                         
                         #Type 1
                         #Uniform air temperature thresholds
                         #1.0°C = NH station avg., 1.5°C = optimized average
                         thresh_t_air <- seq(-1, 3.0, by = 0.5)
                         phase_t_air.ls <- list()
                         for(j in 1:length(thresh_t_air)){
                           phase_t_air.ls[[j]] <- 
                             overlay(stack(temp_c, ppt, land_map), fun =   
                                       function(x,y,q) { 
                                         ifelse(q == 1,
                                                ifelse(y < ppt_thresh, 
                                                       NA,                     #NA = no precipitation
                                                       ifelse((x <= thresh_t_air[j]), 
                                                              1,               #1 = snow
                                                              0)),             #0 = rain
                                                NA) } )         
                         }
                         
                         #Type 2
                         #Uniform dew point and wet bulb thresholds
                         #Range is smaller in literature than T_air
                         #See Marks et al. (2013)
                         thresh_t_dew_wet <- seq(0, 1.0, by = 0.5)
                         phase_t_dew.ls <- list()
                         for(j in 1:length(thresh_t_air)){
                           phase_t_dew.ls[[j]] <- 
                             overlay(stack(temp_dew, ppt, land_map), fun =   
                                       function(x,y,q) { 
                                         ifelse(q == 1,
                                                ifelse(y < ppt_thresh, 
                                                       NA,                     #NA = no precipitation
                                                       ifelse((x <= thresh_t_dew_wet[j]), 
                                                              1,               #1 = snow
                                                              0)),             #0 = rain
                                                NA) } )         
                         }
                         
                         phase_t_wet.ls <- list()
                         for(j in 1:length(thresh_t_air)){
                           phase_t_wet.ls[[j]] <- 
                             overlay(stack(temp_wet, ppt, land_map), fun =   
                                       function(x,y,q) { 
                                         ifelse(q == 1,
                                                ifelse(y < ppt_thresh, 
                                                       NA,                     #NA = no precipitation
                                                       ifelse((x <= thresh_t_dew_wet[j]), 
                                                              1,               #1 = snow
                                                              0)),             #0 = rain
                                                NA) } )         
                         }
                         
                         
                         #Type 3
                         #Spatially variable air temperature threshold
                         #From world map
                         phase_map <- 
                           overlay(stack(temp_c, ppt, t50_map, land_map), fun =   
                                     function(x,y,z, q) { 
                                       ifelse(q == 1,
                                              ifelse(y < ppt_thresh, 
                                                     NA,                     #NA = no precipitation
                                                     ifelse((x <= z), 
                                                            1,               #1 = snow
                                                            0) ),             #0 = rain
                                              NA) } ) 
                         
                         
                         #Type 4
                         #Bivariate and trivariate regressions
                         phase_binlog <- 
                           overlay(stack(temp_c, ppt, rh, land_map), fun =   
                                     function(x,y,z,q) { 
                                       ifelse(q == 1, 
                                              ifelse(y < ppt_thresh, 
                                                     NA,                     #NA = no precipitation
                                                     ifelse((1/(1 + exp(-10.04 + 1.41 * x + 0.09 * z)) > 0.5), 
                                                            1,               #1 = snow
                                                            0) ),             #0 = rain
                                              NA)} ) 
                         
                         #Partition precipitation using the trivariate logistic regression
                         phase_trilog <- 
                           overlay(stack(temp_c, ppt, rh, press, land_map), fun =   
                                     function(x,y,z,w,q) { 
                                       ifelse(q == 1,
                                              ifelse(y < ppt_thresh, 
                                                     NA,                     #NA = no precipitation
                                                     ifelse((1/(1 + exp(-12.80 + 1.41 * x + 0.09 * z + 0.03 * (w / 1000))) > 0.5), 
                                                            1,               #1 = snow
                                                            0) ),             #0 = rain
                                              NA) } )
                         
                         
                         
                         
                         #######################################################################################
                         #Output results from the precipitation phase algorithms
                         #The precip phase algorithms must be presented in a list
                         #In order to be output as multiple list entries in foreach
                         #Also included are the meteorological data
                         list(
                           #Air temperature thresholds
                           phase_t_air.ls[[1]],
                           phase_t_air.ls[[2]],
                           phase_t_air.ls[[3]],
                           phase_t_air.ls[[4]],
                           phase_t_air.ls[[5]],
                           phase_t_air.ls[[6]],
                           phase_t_air.ls[[7]],
                           phase_t_air.ls[[8]],
                           phase_t_air.ls[[9]],
                           #Dew point and wet bulb thresholds
                           phase_t_dew.ls[[1]],
                           phase_t_dew.ls[[2]],
                           phase_t_dew.ls[[3]],
                           phase_t_wet.ls[[1]],
                           phase_t_wet.ls[[2]],
                           phase_t_wet.ls[[3]],
                           #Threshold from map
                           phase_map,
                           #Bi- and trivariate regressions
                           phase_binlog,
                           phase_trilog,
                           #Meteorological data
                           temp_c,
                           temp_dew,
                           temp_wet,
                           rh,
                           press
                         )
                       } 
  ##########End parallel loop###############
  
  
  #Create dummy list for storing all phase and met dataframes
  phase.ls <- list()
  
  #Loop through each day and add the met and phase data
  phase.ls <- foreach(i = 1:length(metfiles.tmp), .errorhandling = "pass") %dopar% {
    #Loop through the output variables
    #And add each to their own list entry
    phase.tmp <- list()    
    for(j in 1:length(phase.master)){
      phase.tmp[[j]] <- as.data.frame(rasterToPoints(phase.master[[j]][[i]]))
    }
    tmp <- 
      list(phase.tmp[[1]], phase.tmp[[2]], phase.tmp[[3]], phase.tmp[[4]], phase.tmp[[5]], phase.tmp[[6]],
           phase.tmp[[7]], phase.tmp[[8]], phase.tmp[[9]], phase.tmp[[10]], phase.tmp[[11]], phase.tmp[[12]],
           phase.tmp[[13]], phase.tmp[[14]], phase.tmp[[15]], phase.tmp[[16]], phase.tmp[[17]], phase.tmp[[18]],
           phase.tmp[[19]], phase.tmp[[20]], phase.tmp[[21]], phase.tmp[[22]], phase.tmp[[23]]) %>% 
      Reduce(function(dtf1,dtf2) left_join(dtf1,dtf2,by=c("x", "y")), .)
    
    #Rename columns
    colnames(tmp)[3:25] <- c("phase01", "phase02", "phase03", "phase04", "phase05", "phase06",
                             "phase07", "phase08", "phase09", "phase10", "phase11", "phase12",
                             "phase13", "phase14", "phase15", "phase16", "phase17", "phase18",
                             "t_air", "t_dew", "t_wet", "rh", "press" )
    tmp$date <- substr(x = metfiles[[i]], start = 32, stop = 39)
    
    #Below code was used to reduce dataset to only temps within ±8°C
    #tmp <- filter(tmp, t_air >= -8 & t_air <= 8)
    
    #Melt into vertical dataframe
    tmp_melt <- melt(tmp, id.vars = c("x", "y", "date", "t_air", "t_dew", "t_wet", "rh", "press"),
                     variable.name = "phase_alg", value.name = "snow_phase")
    
    #Output data to list
    tmp_melt
  } #End parallel loop
  
  #Turn list into dataframe
  phase.df <- ldply(phase.ls, rbind)
  
  #Remove any NA phase values
  phase.df <- filter(phase.df, is.na(snow_phase) == F)
  
  #Create rain phase column (phase = 1 for snow, phase = 0 for rain)
  phase.df$rain_phase <- ifelse(phase.df$snow_phase == 0,
                                1,
                                0)
  
  #Create cell id
  phase.df$cellid <- as.factor(paste(phase.df$x, phase.df$y, sep = ","))
  
  #Create month and season columns
  phase.df$month <- substr(phase.df$date, start = 5, stop = 6)
  seasons <- data.frame(season = rep(c("DJF", "MAM", "JJA", "SON"), each = 3),
                        month = c("01", "02", "03", "04", "05", "06", 
                                  "07", "08", "09", "10", "11", "12"))
  phase.df <- left_join(phase.df, seasons,
                        by = "month")
  
  #Export the data frame
  #saveRDS(phase.df,file = paste0("../merra_phase_sim_by_year/merra_phase_sim_", data_yrs[k]), ".rds" )
  
  
  ##############################################################################################
  #2 Compute rain-snow frequencies and put into list
  
  ###########################
  #Compute the frequencies and met stats
  ###########
  #Per cell
  
  #Annual frequency
  freq_ann.ls[[k]] <- phase.df %>% 
    group_by(cellid, phase_alg) %>% 
    summarise(n_snow = sum(snow_phase),
              n_rain = sum(rain_phase),
              t_air_mean = mean(t_air),
              t_dew_mean = mean(t_dew),
              t_wet_mean = mean(t_wet),
              press_mean = mean(press),
              rh_mean = mean(rh))
  
  #Seasonal frequency 
  freq_ssn.ls[[k]] <- phase.df %>% 
    group_by(cellid, season, phase_alg) %>% 
    summarise(n_snow = sum(snow_phase),
              n_rain = sum(rain_phase),
              t_air_mean = mean(t_air),
              t_dew_mean = mean(t_dew),
              t_wet_mean = mean(t_wet),
              press_mean = mean(press),
              rh_mean = mean(rh))
  
  ###########
  #Over the northern hemisphere
  
  #Annual frequency
  freq_ann_NH.ls[[k]] <- phase.df %>% 
    group_by(phase_alg) %>% 
    summarise(n_snow = sum(snow_phase),
              n_rain = sum(rain_phase))

  #Seasonal frequency
  freq_ssn_NH.ls[[k]] <- phase.df %>% 
    group_by(season, phase_alg) %>% 
    summarise(n_snow = sum(snow_phase),
              n_rain = sum(rain_phase))
  
  
  #Remove temp phase files
  rm(phase.df)
  rm(phase.master)
  rm(phase.ls)
  
  #Run garbage collection to clear memory
  gc()
  
  #Print loop number for progress update
  print(k)
  
} #End main loop

#Bind all list elements into dataframes

freq_ann.df <- ldply(freq_ann.ls, rbind)
freq_ssn.df <- ldply(freq_ssn.ls, rbind)
freq_ann_NH.df <- ldply(freq_ann_NH.ls, rbind)
freq_ssn_NH.df <- ldply(freq_ssn_NH.ls, rbind)

