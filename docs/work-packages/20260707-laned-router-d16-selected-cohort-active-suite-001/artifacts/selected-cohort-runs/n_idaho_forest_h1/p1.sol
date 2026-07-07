9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-05-28 20:33:07.390440
# Source Data: Surgo
# 
# Mukey: 2396862
# Major Component: 26403445 (comppct_r = 35.0)
# Texture: silt loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 78776562   Oi     X        1.0   373.0        0.0         0.0          0.2     6.0    29.0     9.3    75.0
# 78776566   Oe     X        6.0   106.0        0.0         0.0          0.2     6.0    29.0     9.3    70.0
# 78776563   A              29.0     9.0       17.0         0.0          0.8     6.0    29.0     9.3     5.0
# 78776564   Bw1            57.0     9.0        0.0        25.0          0.8     6.0    29.0     9.3     2.0
# 78776567   Bw2           113.0     9.0       54.0        12.0          0.8     6.0    29.0     9.3     1.0
# 78776565   2BC           159.0     9.0       44.0         0.0          1.3    12.0    45.0    12.6    0.25
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: -
# ksat: -
# 
# defaults applied to missing chorizon data:
# sandtotal_r  ->      66.800
# claytotal_r  ->       7.000
# om_r         ->       7.000
# cec7_r       ->      11.300
# sandvf_r     ->      10.000
# smr          ->      55.500
# 
# Build Notes:
# 78776563::wilt_pt estimated from wfifteenbar_r and rock
# 78776563::field_cap estimated from wthirdbar_r and rock
# 78776564::wilt_pt estimated from wfifteenbar_r and rock
# 78776564::field_cap estimated from wthirdbar_r and rock
# 78776567::wilt_pt estimated from wfifteenbar_r and rock
# 78776567::field_cap estimated from wthirdbar_r and rock
# 78776565::wilt_pt estimated from wfifteenbar_r and rock
# 78776565::field_cap estimated from wthirdbar_r and rock
# res_lyr_i None
# 
# THIS FILE AND THE CONTAINED DATA IS PROVIDED BY THE UNIVERSITY OF IDAHO
# 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED
# TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
# PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL UNIVERSITY OF IDAHO
# BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
# CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
# SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
# INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHERE IN
# CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
# ARISING IN ANY WAY OUT OF THE USE OF THIS FILE, EVEN IF ADVISED OF THE
# POSSIBILITY OF SUCH DAMAGE.
# 
# 
# If you change the original contexts of this file please
# indicate it by putting an 'X' in the box here -> [ ]
# 
# 
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::9002.0migration
# Build Date: 2026-05-28 20:33:08.639326
# Source File: :/wc1/runs/un/unpalatable-rind/soils/2396862.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> silt loam
# ki -> 1000000
# kr -> 5.00E-05
# shcrit -> 1.5
# avke -> 40
# bd ->
# ksflag -> 0
# ksatadj -> 0
# ksatfac -> 1.5
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 2
# xmxlai -> 14
# keffflag -> 0
# lkeff -> -9999
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
# 
# h0_min_depth = None
# h0_max_om = None
# 
# wepppy.wepp.soils.utils.WeppSoilUtil::modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
0	 'forest'	 'silt loam'	 1.5 	 0.3
'Humic Udivitrands-Ahrs-Goatrock families, complex, steep breaklands with avalanche chutes, weakly weathered belts, timber-brush-fern glade mosaic, south aspects'	 'GRV-ASHY-SIL'	 5	 0.16	 0.75	 1000000	 5e-05	 1.5
	200.0	 0.8	 40	 10.0	 0.258	 0.07	 29.0	 6.0	 5.0	 13.9	 68.46	 0.07515	 0.5289	 0.0028	 1.632	 367.6	 0.1179	 0.1938
	290.0	 0.8	 32.4	 10.0	 0.258	 0.07	 29.0	 6.0	 5.0	 13.9	 68.46	 0.07515	 0.5289	 0.0028	 1.632	 367.6	 0.1179	 0.1938
	570.0	 0.8	 32.4	 1.0	 0.214	 0.042	 29.0	 6.0	 2.0	 8.9	 68.5	 0.07515	 0.5289	 0.0028	 1.632	 367.6	 0.1179	 0.1938
	1130.0	 0.8	 32.4	 1.0	 0.082	 0.014	 29.0	 6.0	 1.0	 6.7	 90.82	 0.07515	 0.5289	 0.0028	 1.632	 367.6	 0.1179	 0.1938
	1600.0	 1.3	 32.4	 1.0	 0.09	 0.026	 45.0	 12.0	 0.25	 7.2	 88.8	 0.07162	 0.4105	 0.007155	 1.485	 40.4	 0.1067	 0.2156
1 10000.0 0.01
