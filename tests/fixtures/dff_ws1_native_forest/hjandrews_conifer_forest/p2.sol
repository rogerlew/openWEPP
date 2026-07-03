9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 20:26:41.938609
# Source Data: Surgo
# 
# Mukey: 3385510
# Major Component: 27353091 (comppct_r = 50.0)
# Texture: loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 81653761   Oi     X        4.0   400.0        0.0         0.0          0.2     7.0    66.8    10.0    75.0
# 81653760   A1             24.0    25.0        0.0         0.0         0.85    18.0    40.0    12.0     8.0
# 81653759   A2             43.0    25.0        0.0         4.0         0.95    20.0    40.0    12.0     5.0
# 81653758   BA             67.0    15.0        0.0         0.0          1.0    24.0    30.0    11.0     3.0
# 81653757   Bw1           105.0     5.0        0.0         0.0         1.15    30.0    25.0     9.0     1.5
# 81653756   Bw2           150.0     5.0        0.0        12.0          1.3    30.0    22.0     8.0    0.75
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
# 81653760::wilt_pt estimated from wfifteenbar_r and rock
# 81653760::field_cap estimated from wthirdbar_r and rock
# 81653759::wilt_pt estimated from wfifteenbar_r and rock
# 81653759::field_cap estimated from wthirdbar_r and rock
# 81653758::wilt_pt estimated from wfifteenbar_r and rock
# 81653758::field_cap estimated from wthirdbar_r and rock
# 81653757::wilt_pt estimated from wfifteenbar_r and rock
# 81653757::field_cap estimated from wthirdbar_r and rock
# 81653756::wilt_pt estimated from wfifteenbar_r and rock
# 81653756::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-26 20:26:42.410230
# Source File: :/wc1/runs/jo/joyous-armchair/soils/3385510.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> loam
# ki -> 400000
# kr -> 3.00E-05
# shcrit -> 1
# avke -> 50
# bd ->
# ksflag -> 1
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
1 1
0	 'forest'	 'loam'	 1.5 	 0.3
'Kinney, moist-Mooseridge complex, 0 to 30 percent slopes'	 'L'	 6	 0.16	 0.75	 400000	 3e-05	 1
	200.0	 0.85	 50	 10.0	 0.49	 0.138	 40.0	 18.0	 8.0	 25.0	 51.0	 0.0931	 0.5388	 0.004785	 1.471	 195.3	 0.1526	 0.2581
	240.0	 0.85	 90.0	 10.0	 0.49	 0.138	 40.0	 18.0	 8.0	 25.0	 51.0	 0.0931	 0.5388	 0.004785	 1.471	 195.3	 0.1526	 0.2581
	430.0	 0.95	 90.0	 10.0	 0.4472	 0.1473	 40.0	 20.0	 5.0	 25.0	 44.32	 0.09383	 0.5161	 0.005345	 1.456	 121.9	 0.1511	 0.2621
	670.0	 1.0	 54.0	 1.0	 0.4	 0.1347	 30.0	 24.0	 3.0	 18.0	 28.0	 0.09918	 0.5139	 0.004613	 1.456	 89.19	 0.1592	 0.2694
	1050.0	 1.15	 18.0	 1.0	 0.3674	 0.1584	 25.0	 30.0	 1.5	 18.0	 11.0	 0.1059	 0.4892	 0.005214	 1.416	 38.2	 0.1682	 0.2873
	1600.0	 1.3	 18.0	 1.0	 0.3864	 0.1818	 22.0	 30.0	 0.75	 16.0	 20.8	 0.1045	 0.4507	 0.00521	 1.407	 17.27	 0.163	 0.2827
1 10000.0 0.01
