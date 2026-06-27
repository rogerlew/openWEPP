#Script for comparing precipitation phase methods to station observations
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
#Optimize the binary logistic regression models where:
#Univariate: phase is f(air temperature)
#Bivariate: phase is f(air temperature, relative humidity)
#Trivariate: phase is f(air temperature, relative humidity, and surface pressure)

#This is the model develpment script 
#We run 250 test simulations and obtain the model coefficients 
#The optimized coefficients are the mean from the 250 simulations

#Because the data are selected randomly, different iterations of this script will
#produce different model coefficients

#Filter data to -3°C to +5°C temperature range and rename pressure column
station_obs_filtered = filter(station_obs, Air_Temp > -3 & Air_Temp < 5) 
station_obs_filtered$Pressure <- station_obs_filtered$gridded_data_pres

#create empty data frames 
coef_T = data.frame() #Univariate model coefficient data frame 
coef_T_RH = data.frame() #Bivariate model coefficient data frame
coef_T_RH_P = data.frame() #Trivariate model coefficient data frame
validation_removal = c() #data frame to store the index values for observations used in the training dataset. These observations will then be removed from the validation dataset

#training data frames from which random samples of data will be used in each simulation 
train_T = data_frame(Rain_Phase = station_obs_filtered$Rain_Phase, Air_Temp = station_obs_filtered$Air_Temp)
train_T_RH = data_frame(Rain_Phase = station_obs_filtered$Rain_Phase, Air_Temp = station_obs_filtered$Air_Temp, RH = station_obs_filtered$RH)
train_T_RH_P = data_frame(Rain_Phase = station_obs_filtered$Rain_Phase, Air_Temp = station_obs_filtered$Air_Temp, RH = station_obs_filtered$RH, Pressure = station_obs_filtered$Pressure)

#Run 250 simulations to obtain 250 sets of coefficients for each model #############
for (i in 1:250){
  index_train = sample.int(nrow(station_obs_filtered),size = 5000,replace = FALSE,prob = NULL) #index values for random sample of 5000 data points 
  train_data_T = train_T[index_train,] #select the sample from the training dataset 
  train_data_T_RH = train_T_RH[index_train,]
  train_data_T_RH_P = train_T_RH_P[index_train,]
  
  validation_removal = c(validation_removal,index_train) #add the selected training index to the validation removal data frame 
  
  #create the three different models using the selected training data 
  model_T = glm(Rain_Phase ~.,family = binomial(link='logit'),data=train_data_T) 
  model_T_RH = glm(Rain_Phase ~.,family = binomial(link='logit'),data=train_data_T_RH) 
  model_T_RH_P = glm(Rain_Phase ~.,family = binomial(link='logit'),data=train_data_T_RH_P) 
  
  #add the Ts model coefficients to the Ts model coefficients data frame 
  new.row.T = cbind(model_T$coefficients[1],model_T$coefficients[2])
  coef_T = rbind(coef_T,new.row.T)
  
  #add the Ts-RH model coefficients to the Ts-RH data frame 
  new.row.T.RH = cbind(model_T_RH$coefficients[1],model_T_RH$coefficients[2],model_T_RH$coefficients[3])
  coef_T_RH = rbind(coef_T_RH,new.row.T.RH)
  
  #Add the Ts-RH-Ps model coefficients to the Ts-RH-Ps coefficients data frame
  new.row.T.RH.P = cbind(model_T_RH_P$coefficients[1],model_T_RH_P$coefficients[2],model_T_RH_P$coefficients[3],model_T_RH_P$coefficients[4])
  coef_T_RH_P = rbind(coef_T_RH_P,new.row.T.RH.P)
  
  #reset the following variables 
  rm(train_data_T_RH,train_data_T_RH_P,new.row.T,new.row.T.RH,new.row.T.RH.P,index_train) 
} #end of for loop 

#name the columns of the coefficient data frames 
colnames(coef_T) <- c("(Intercept)", "Air_Temp")
colnames(coef_T_RH) <- c("(Intercept)", "Air_Temp","RH")
colnames(coef_T_RH_P) <- c("(Intercept)", "Air_Temp","RH","Pressure")

#for each model, the coefficients are the means of the coefficients from the 250 test simulations: 
model_T$coefficients <- colMeans(coef_T)
model_T_RH$coefficients <- colMeans(coef_T_RH)
model_T_RH_P$coefficients <- colMeans(coef_T_RH_P)


###############################################################################
###############################################################################
###############################################################################
#Validate the binary regression models on observed data

#Remove the unique training data from the vaidation dataset 
validation_removal = unique(validation_removal) 
test_data = station_obs_filtered[-validation_removal,] #validation dataset 


###############################################################################
#Calculate success rates by relative humidity bin
rhbins <- data.frame(rhbin_max = seq(42, 100, by = 2),
                     rhbin_min = seq(40, 98, by = 2))
success_rates_RH <- list()
for(i in seq_along(rhbins$rhbin_max)){
  tmp = filter(test_data, RH >= rhbins[i, "rhbin_min"] & RH < rhbins[i, "rhbin_max"])
  success_rates_RH[[i]] = tmp %>% 
    tbl_df %>% 
    mutate(type_T = ifelse(1/(1 + exp(-1.54 + 1.24 * Air_Temp)) <=  0.5,
                           1, 0)) %>%
    mutate(type_T_RH = ifelse(1/(1 + exp(-10.04 + 1.41 * Air_Temp + 0.09 * RH)) <= 0.5,
                           1, 0)) %>%
    mutate(type_T_RH_P = ifelse(1/(1 + exp(-12.8 + 1.41 * Air_Temp + 0.09 * RH + 0.03 * gridded_data_pres)) <= 0.5,
                                      1, 0)) %>%
    summarise(
      Success_T=1-mean(type_T != Rain_Phase),
      Success_T_RH=1-mean(type_T_RH != Rain_Phase),
      Success_T_RH_P=1-mean(type_T_RH_P != Rain_Phase),
      Total = n()
    )
}

#Load plyr and convert list to data frame
library(plyr)
success_rates_RH_d <- ldply(success_rates_RH, .fun = rbind)
detach("package:plyr", unload=TRUE)

success_rates_RH_d$rhbin <- 1:30
success_rates_RH_d$rhbin_mid <- seq(41, 99, by = 2)

#Melt the data frame
success_rates_RH_melt <- melt(success_rates_RH_d,
                              measure.vars = c("Success_T", "Success_T_RH", "Success_T_RH_P"), 
                              variable.name = "model", value.name = "success_rate" )



###############################################################################
#Calculate success rates by temperature bin
tbins <- data.frame(tbin_max = seq(-2.75, 5, by = 0.25),
                    tbin_min = seq(-3, 4.75, by = 0.25))
success_rates_T <- list()
for(i in seq_along(tbins$tbin_max)){
  tmp = filter(test_data, Air_Temp >= tbins[i, "tbin_min"] & Air_Temp < tbins[i, "tbin_max"])
  success_rates_T[[i]] = tmp %>% 
    tbl_df %>% 
    mutate(type_T = ifelse(1/(1 + exp(-1.54 + 1.24 * Air_Temp)) <=  0.5,
                           1, 0)) %>%
    mutate(type_T_RH = ifelse(1/(1 + exp(-10.04 + 1.41 * Air_Temp + 0.09 * RH)) <= 0.5,
                              1, 0)) %>%
    mutate(type_T_RH_P = ifelse(1/(1 + exp(-12.8 + 1.41 * Air_Temp + 0.09 * RH + 0.03 * gridded_data_pres)) <= 0.5,
                                1, 0)) %>%
    summarise(
      Success_T=1-mean(type_T != Rain_Phase),
      Success_T_RH=1-mean(type_T_RH != Rain_Phase),
      Success_T_RH_P=1-mean(type_T_RH_P != Rain_Phase),
      Total = n()
    )
}

#Load plyr and convert list to data frame
library(plyr)
success_rates_T_d <- ldply(success_rates_T, .fun = rbind)
detach("package:plyr", unload=TRUE)

success_rates_T_d$tbin <- 1:32
success_rates_T_d$tbin_mid <- seq(-2.875, 4.875, by = 0.25)

#Melt the data frame
success_rates_T_melt <- melt(success_rates_T_d, 
                             measure.vars = c("Success_T", "Success_T_RH", "Success_T_RH_P"), 
                             variable.name = "model", value.name = "success_rate" )


###############################################################################
###############################################################################
###############################################################################
#Calculate the success rates for the threshold-based precipitation phase methods

#Thresholds:
thresh_t_air <- seq(-1, 3.0, by = 0.5)
thresh_t_dew_wet <- seq(0, 1.0, by = 0.5)

#Calculate wet bulb temperature (dew point reported in initial dataset)
#Equation from Stull (2011)
test_data$t_wet <- with(test_data,
                     Air_Temp * atan(0.151977 *( (RH + 8.313659) ^ 0.5)) +
                       atan(Air_Temp + RH) -
                       atan(RH - 1.676331) +
                       ((0.00391838 * (RH^ 1.5)) * atan(0.023101 * RH)) -
                       4.86035)


###############################################################################
#Calculate success rate by relative humidity 

#Initialize list for storing datasets
success_rates_RH_thresholds <- list()

#Loop through RH bins and store successes
for(i in seq_along(rhbins$rhbin_max)){
  tmp = filter(test_data, RH >= rhbins[i, "rhbin_min"] & RH < rhbins[i, "rhbin_max"])
  success_rates_RH_thresholds[[i]] = tmp %>% 
    tbl_df %>% 
    mutate(type_Ta_neg1.0 = ifelse(Air_Temp > thresh_t_air[1],
                                  0, 1)) %>% 
    mutate(type_Ta_neg0.5 = ifelse(Air_Temp > thresh_t_air[2],
                                  0, 1)) %>% 
    mutate(type_Ta_0.0 = ifelse(Air_Temp > thresh_t_air[3],
                                  0, 1)) %>% 
    mutate(type_Ta_0.5 = ifelse(Air_Temp > thresh_t_air[4],
                               0, 1)) %>% 
    mutate(type_Ta_1.0 = ifelse(Air_Temp > thresh_t_air[5],
                               0, 1)) %>% 
    mutate(type_Ta_1.5 = ifelse(Air_Temp > thresh_t_air[6],
                               0, 1)) %>% 
    mutate(type_Ta_2.0 = ifelse(Air_Temp > thresh_t_air[7],
                               0, 1)) %>% 
    mutate(type_Ta_2.5 = ifelse(Air_Temp > thresh_t_air[8],
                               0, 1)) %>% 
    mutate(type_Ta_3.0 = ifelse(Air_Temp > thresh_t_air[9],
                               0, 1)) %>%
    mutate(type_Td_0.0 = ifelse(Dewpoint > thresh_t_dew_wet[1],
                                0, 1)) %>% 
    mutate(type_Td_0.5 = ifelse(Dewpoint > thresh_t_dew_wet[2],
                                0, 1)) %>% 
    mutate(type_Td_1.0 = ifelse(Dewpoint > thresh_t_dew_wet[3],
                                0, 1)) %>% 
    mutate(type_Tw_0.0 = ifelse(t_wet > thresh_t_dew_wet[1],
                                0, 1)) %>% 
    mutate(type_Tw_0.5 = ifelse(t_wet > thresh_t_dew_wet[2],
                                0, 1)) %>% 
    mutate(type_Tw_1.0 = ifelse(t_wet > thresh_t_dew_wet[3],
                                0, 1)) %>% 
    summarise(
      Success_Ta_neg1.0 = 1 - mean(type_Ta_neg1.0 != Snow_Phase, na.rm = T),
      Success_Ta_neg0.5 = 1 - mean(type_Ta_neg0.5 != Snow_Phase, na.rm = T),
      Success_Ta_0.0    = 1 - mean(type_Ta_0.0 != Snow_Phase,    na.rm = T),
      Success_Ta_0.5    = 1 - mean(type_Ta_0.5 != Snow_Phase,    na.rm = T),
      Success_Ta_1.0    = 1 - mean(type_Ta_1.0 != Snow_Phase,    na.rm = T),
      Success_Ta_1.5    = 1 - mean(type_Ta_1.5 != Snow_Phase,    na.rm = T),
      Success_Ta_2.0    = 1 - mean(type_Ta_2.0 != Snow_Phase,    na.rm = T),
      Success_Ta_2.5    = 1 - mean(type_Ta_2.5 != Snow_Phase,    na.rm = T),
      Success_Ta_3.0    = 1 - mean(type_Ta_3.0 != Snow_Phase,    na.rm = T),
      Success_Td_0.0    = 1 - mean(type_Td_0.0 != Snow_Phase,    na.rm = T),
      Success_Td_0.5    = 1 - mean(type_Td_0.5 != Snow_Phase,    na.rm = T),
      Success_Td_1.0    = 1 - mean(type_Td_1.0 != Snow_Phase,    na.rm = T),
      Success_Tw_0.0    = 1 - mean(type_Tw_0.0 != Snow_Phase,    na.rm = T),
      Success_Tw_0.5    = 1 - mean(type_Tw_0.5 != Snow_Phase,    na.rm = T),
      Success_Tw_1.0    = 1 - mean(type_Tw_1.0 != Snow_Phase,    na.rm = T),
      Total = n()
    )
}

#Loop 

#Load plyr and convert list to data frame
library(plyr)
success_rates_RH_thresholds_d <- ldply(success_rates_RH_thresholds, .fun = rbind)
detach("package:plyr", unload=TRUE)

success_rates_RH_thresholds_d$rhbin <- 1:30
success_rates_RH_thresholds_d$rhbin_mid <- seq(41, 99, by = 2)

#Melt the data frame
success_rates_RH_thresholds_melt <- melt(success_rates_RH_thresholds_d, 
                              measure.vars = c("Success_Ta_neg1.0","Success_Ta_neg0.5","Success_Ta_0.0","Success_Ta_0.5","Success_Ta_1.0","Success_Ta_1.5","Success_Ta_2.0","Success_Ta_2.5","Success_Ta_3.0","Success_Td_0.0","Success_Td_0.5","Success_Td_1.0","Success_Tw_0.0","Success_Tw_0.5","Success_Tw_1.0"), 
                              variable.name = "model", value.name = "success_rate" )
#Add algorithm type to data
success_rates_RH_thresholds_melt$model_type <-
  substr(success_rates_RH_thresholds_melt$model, 9, 10)

#Add the binary regression models
success_rates_RH_melt$model_type <- "bin_reg"
success_rates_RH_thresholds_melt <- bind_rows(success_rates_RH_thresholds_melt,
                                              success_rates_RH_melt)

###############################################################################
#Calculate success rate by air temperature 

#Initialize list for storing datasets
success_rates_T_thresholds <- list()

#Loop through T bins and store successes
for(i in seq_along(tbins$tbin_max)){
  tmp = filter(test_data, Air_Temp >= tbins[i, "tbin_min"] & Air_Temp < tbins[i, "tbin_max"])
  success_rates_T_thresholds[[i]] = tmp %>% 
    tbl_df %>% 
    mutate(type_Ta_neg1.0 = ifelse(Air_Temp > thresh_t_air[1],
                                   0, 1)) %>% 
    mutate(type_Ta_neg0.5 = ifelse(Air_Temp > thresh_t_air[2],
                                   0, 1)) %>% 
    mutate(type_Ta_0.0 = ifelse(Air_Temp > thresh_t_air[3],
                                0, 1)) %>% 
    mutate(type_Ta_0.5 = ifelse(Air_Temp > thresh_t_air[4],
                                0, 1)) %>% 
    mutate(type_Ta_1.0 = ifelse(Air_Temp > thresh_t_air[5],
                                0, 1)) %>% 
    mutate(type_Ta_1.5 = ifelse(Air_Temp > thresh_t_air[6],
                                0, 1)) %>% 
    mutate(type_Ta_2.0 = ifelse(Air_Temp > thresh_t_air[7],
                                0, 1)) %>% 
    mutate(type_Ta_2.5 = ifelse(Air_Temp > thresh_t_air[8],
                                0, 1)) %>% 
    mutate(type_Ta_3.0 = ifelse(Air_Temp > thresh_t_air[9],
                                0, 1)) %>%
    mutate(type_Td_0.0 = ifelse(Dewpoint > thresh_t_dew_wet[1],
                                0, 1)) %>% 
    mutate(type_Td_0.5 = ifelse(Dewpoint > thresh_t_dew_wet[2],
                                0, 1)) %>% 
    mutate(type_Td_1.0 = ifelse(Dewpoint > thresh_t_dew_wet[3],
                                0, 1)) %>% 
    mutate(type_Tw_0.0 = ifelse(t_wet > thresh_t_dew_wet[1],
                                0, 1)) %>% 
    mutate(type_Tw_0.5 = ifelse(t_wet > thresh_t_dew_wet[2],
                                0, 1)) %>% 
    mutate(type_Tw_1.0 = ifelse(t_wet > thresh_t_dew_wet[3],
                                0, 1)) %>% 
    summarise(
      Success_Ta_neg1.0 = 1 - mean(type_Ta_neg1.0 != Snow_Phase, na.rm = T),
      Success_Ta_neg0.5 = 1 - mean(type_Ta_neg0.5 != Snow_Phase, na.rm = T),
      Success_Ta_0.0    = 1 - mean(type_Ta_0.0 != Snow_Phase,    na.rm = T),
      Success_Ta_0.5    = 1 - mean(type_Ta_0.5 != Snow_Phase,    na.rm = T),
      Success_Ta_1.0    = 1 - mean(type_Ta_1.0 != Snow_Phase,    na.rm = T),
      Success_Ta_1.5    = 1 - mean(type_Ta_1.5 != Snow_Phase,    na.rm = T),
      Success_Ta_2.0    = 1 - mean(type_Ta_2.0 != Snow_Phase,    na.rm = T),
      Success_Ta_2.5    = 1 - mean(type_Ta_2.5 != Snow_Phase,    na.rm = T),
      Success_Ta_3.0    = 1 - mean(type_Ta_3.0 != Snow_Phase,    na.rm = T),
      Success_Td_0.0    = 1 - mean(type_Td_0.0 != Snow_Phase,    na.rm = T),
      Success_Td_0.5    = 1 - mean(type_Td_0.5 != Snow_Phase,    na.rm = T),
      Success_Td_1.0    = 1 - mean(type_Td_1.0 != Snow_Phase,    na.rm = T),
      Success_Tw_0.0    = 1 - mean(type_Tw_0.0 != Snow_Phase,    na.rm = T),
      Success_Tw_0.5    = 1 - mean(type_Tw_0.5 != Snow_Phase,    na.rm = T),
      Success_Tw_1.0    = 1 - mean(type_Tw_1.0 != Snow_Phase,    na.rm = T),
      Total = n()
    )
}

#Loop 

#Load plyr and convert list to data frame
library(plyr)
success_rates_T_thresholds_d <- ldply(success_rates_T_thresholds, .fun = rbind)
detach("package:plyr", unload=TRUE)

success_rates_T_thresholds_d$tbin <- 1:32
success_rates_T_thresholds_d$tbin_mid <- seq(-2.875, 4.875, by = 0.25)

#Melt the data frame
success_rates_T_thresholds_melt <- melt(success_rates_T_thresholds_d, 
                                         measure.vars = c("Success_Ta_neg1.0","Success_Ta_neg0.5","Success_Ta_0.0","Success_Ta_0.5","Success_Ta_1.0","Success_Ta_1.5","Success_Ta_2.0","Success_Ta_2.5","Success_Ta_3.0","Success_Td_0.0","Success_Td_0.5","Success_Td_1.0","Success_Tw_0.0","Success_Tw_0.5","Success_Tw_1.0"), 
                                         variable.name = "model", value.name = "success_rate" )
#Add algorithm type to data
success_rates_T_thresholds_melt$model_type <-
  substr(success_rates_T_thresholds_melt$model, 9, 10)

#Add the regression models
success_rates_T_melt$model_type <- "bin_reg"
success_rates_T_thresholds_melt <- bind_rows(success_rates_T_thresholds_melt,
                                             success_rates_T_melt)


###############################################################################
###############################################################################
###############################################################################
#Compare methods and method types										 

#Get average success rates for all models
success_rates_avg_bymodel <- success_rates_T_thresholds_melt %>% 
  group_by(model) %>% 
  summarise(success_rate_avg =sum((Total / sum(Total)) * success_rate))
success_rates_avg_bytype <- success_rates_T_thresholds_melt %>% 
  group_by(model_type) %>% 
  summarise(success_rate_avg =sum((Total / sum(Total)) * success_rate))
 
#Rank the models
success_rates_avg_bymodel <- success_rates_avg_bymodel %>% 
  mutate(rank = dense_rank(desc(success_rate_avg)))
