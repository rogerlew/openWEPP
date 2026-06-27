#Script for creating a spatially continuous rain-snow threshold map using MERRA-2 reanalysis data
#and a bivariate binary logistic regression model

#If using this code, please cite Jennings et al. (2018) Nature Communications

#Keith Jennings
#keith.jennings@colorado.edu
#2018-01-22

#Import libraries
library(raster)
library(rgdal)
library(dplyr)
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

#Assign the number of cores for parallelization
registerDoMC(cores = 4)

#Set working directory
setwd("~/merra/")

#List files for all water years
metfiles <- list.files(path = "met/")
pptfiles <- list.files(path = "precipitation/")


#Initialize raster brick for precipitation phase
phase.master = raster::brick(array(rep(NA, length(metfiles)),
                                   dim = c(180, 576, length(metfiles))),
                             xmn = -180.3125, xmx = 179.6875,
                             ymn = 0.25, ymx = 90.25)

#Assign precipitation depth threshold (i.e. ppt must be greater than this amount for day to count)
ppt_thresh = 1

#Run parallel loop to partition precipitation phase
#Per day per MERRA cell

phase.master <-
  foreach(i = 1:length(metfiles), .errorhandling = "pass") %dopar% { 
    #.errorhandling = "pass" lets code skip over bad station data
    
    #Import data layers from MERRA files
    #Temperature
    temp_k = raster(paste("met", metfiles[i], sep = "/"), #import the netcdf file
                    varname = "T2M") * 1 #and choose variable of interest (T2M = 2 M air temperature in Kelvin)
    #The "* 1" makes a single-band raster
    temp_c = temp_k - 273.15 #make a temperature raster in kelvin
    
    #Precipitation (kg m-2 s-1)
    ppt = raster(paste("precipitation", pptfiles[i], sep = "/"), 
                 varname = "PRECTOTCORR") * 1
    ppt = crop(ppt, extent(-180.3125, 179.6875, 0.25, 90.25))
    ppt = ppt* 86400 #multiply by 86400 s to get mm per day
    
    #Surface pressure (Pa)
    press = raster(paste("met", metfiles[i], sep = "/"), 
                   varname = "PS") * 1
    
    #Specific humidity (kg kg-1)
    spec_hum = raster(paste("met", metfiles[i], sep = "/"), 
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
    
    #Partition precipitation using the binary logistic regression
    phase <- overlay(stack(temp_c, ppt, rh), fun =   
                       function(x,y,z) { 
                         ifelse(y < ppt_thresh, 
							 NA, 
                             1/(1 + exp(-10.04 + 1.41 * x + 0.09 * z)))
                       } )
    #NA = no precipitation
    
    phase
  }

#Initialize raster brick for air temp
temp.master = raster::brick(array(rep(NA, length(metfiles)),
                                  dim = c(180, 576, length(metfiles))),
                            xmn = -180.3125, xmx = 179.6875,
                            ymn = 0.25, ymx = 90.25)

#Run parallel loop to get air temperature
#Per day per MERRA cell
#Needed for binning rain, snow values
temp.master <-
  foreach(i = 1:length(metfiles), .errorhandling = "pass") %dopar% { 
    #.errorhandling = "pass" lets code skip over bad station data
    
    #Import data layers from MERRA files
    #Temperature
    temp_k = raster(paste("met", metfiles[i], sep = "/"), #import the netcdf file
                    varname = "T2M") * 1 #and choose variable of interest (T2M = 2 M air temperature in Kelvin)
    #The "* 1" makes a single-band raster
    temp_c = temp_k - 273.15 #make a temperature raster in kelvin
    
    temp_c
  }

#Initialize raster brick for relative humidity
rh.master = raster::brick(array(rep(NA, length(metfiles)),
                                dim = c(180, 576, length(metfiles))),
                          xmn = -180.3125, xmx = 179.6875,
                          ymn = 0.25, ymx = 90.25)

#Run parallel loop to get RH
#Per day per MERRA cell
#Needed for binning rain, snow values
rh.master <-
  foreach(i = 1:length(metfiles), .errorhandling = "pass") %dopar% { 
    #.errorhandling = "pass" lets code skip over bad station data
    
    #Import data layers from MERRA files
    #Temperature
    temp_k = raster(paste("met", metfiles[i], sep = "/"), #import the netcdf file
                    varname = "T2M") * 1 #and choose variable of interest (T2M = 2 M air temperature in Kelvin)
    
    #Surface pressure (Pa)
    press = raster(paste("met", metfiles[i], sep = "/"), 
                   varname = "PS") * 1
    
    #Specific humidity (kg kg-1)
    spec_hum = raster(paste("met", metfiles[i], sep = "/"), 
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
    rh
  }



#Convert raster to points as data frame
#This gives all entries in data frame format per day
#Then go to next do and convert raster to points
#rbind this to previous day

phase.ls <- list()

phase.ls <- foreach(i = 1:length(metfiles), .errorhandling = "pass") %dopar% {
  temp.tmp <- as.data.frame(rasterToPoints(temp.master[[i]]))
  phase.tmp <- as.data.frame(rasterToPoints(phase.master[[i]]))
  rh.tmp <- as.data.frame(rasterToPoints(rh.master[[i]]))
  tmp <- left_join(temp.tmp, phase.tmp, by = c("x", "y"))
  tmp <- left_join(tmp, rh.tmp, by = c("x", "y"))
  #Rename columns
  colnames(tmp)[c(3,4,5)] <- c("temp", "snow_phase", "rh")
  tmp$date <- substr(x = metfiles[[i]], start = 32, stop = 39)
  tmp <- filter(tmp, temp >= -8 & temp <= 8 &
                  is.na(snow_phase) == F)
  tmp
}

#Load plyr to bind all list elements
detach("package:dplyr", unload=TRUE)
suppressWarnings(library(plyr))
phase.df <- ldply(phase.ls, rbind)

#Remove plyr and reload dplyr
detach("package:plyr", unload=TRUE)
suppressWarnings(library(dplyr))

#Create rain phase column (phase = 1 for snow, phase = 0 for rain)
phase.df$rain_phase <- ifelse(is.na(phase.df$snow_phase) == F,
                              1 - phase.df$snow_phase,
                              NA)

#Create cell id
phase.df$cellid <- as.factor(paste(phase.df$x, phase.df$y, sep = ","))

#Create temperature bins
temp_bins <- seq(-8, 8, by =1)
phase.df$temp_bin <- cut(phase.df$temp, breaks = temp_bins, include.lowest = F, right = T)


#Compute snow frequencies by cell and temp_bin
freq.df <- phase.df %>% 
  group_by(cellid, temp_bin) %>% 
  summarise(snow_freq = sum(snow_phase) / 
              (sum(snow_phase) + sum(rain_phase)),
            n_obs = length(snow_phase))

#Convert the cut factors to numeric temperatures (the midpoint of each bin)
temp_cuts_to_number <- data.frame("temp_bin" = levels(phase.df$temp_bin),
                                  "temp" = seq(-7.5, 7.5, by = 1))

#Join the frequency data and numeric temperature bin midpoints
freq.df <- left_join(freq.df, temp_cuts_to_number,
                     by = "temp_bin")

#Remove NA values
freq.df <- filter(freq.df, is.na(temp_bin) == F)

#Identify unique cells
cellids <- unique(freq.df$cellid)

#Loop to get 50% threshold per cell

#Create data frame for 50% temperature threshold per cell
temp50_sim <- data.frame("cellid" = numeric(),
                       "temp50" = numeric())

#First for probabilistic frequencies based on Taylor's regression
#snow_freq2
for (i in 1: length(cellids)){
  #Put data for one cell into temporary data frame
  tmpcell <- subset(freq.df, cellid == cellids[i])
  
  #Identify the cell
  temp50_sim[i, "cellid"] = as.character(unlist(tmpcell[1, "cellid"]))
  
  #Run if there are at least 14 entries
  #And at least 200 observations
  #Fewer entries means not enough temperature bins observed
  if (length(tmpcell$temp) >= 14 & sum(tmpcell$n_obs) > 200) {
    tryCatch(
      {freq_fit <- nlsLM(snow_freq ~ #predicts snow frequency
                           a * (tanh( b* (temp - c)) - d), #as a function of air temperature and 4 fitting parameters
                         #tanh is the hyperbolic tangent (gives curve shape)
                         data = tmpcell, 
                         start = list(a = -45, b = 0.7, c = 1.2, d = 1)) #the starting values are from Dai (2008)
      #Calculate the 50% threshold temperature from the hyperbolic tangent equation
      #Note: atanh function is not used, but rather its underlying equation is
      #atanh has difficulties with larger numbers
      #See: http://www.stat.umn.edu/macanova/htmlhelp/node30.htm
      temp50_sim[i, "temp50"] = (.5*log((1+as.numeric(coef(freq_fit)[1]))/
                                        (1-as.numeric(coef(freq_fit)[1]))) + 
                                 as.numeric(coef(freq_fit)[4]) )/
        as.numeric(coef(freq_fit)[2]) +
        as.numeric(coef(freq_fit)[3])})
  } else { #if not enough entries, enter temperature as NA
    temp50_sim[i, "temp50"] = NA
  }
}

suppressWarnings(library(tidyr))

#Split cellid into lat and lon components
temp50_sim <- temp50_sim %>% 
  separate(cellid, sep = ",", into = c("lon", "lat"))




