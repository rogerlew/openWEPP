9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-03-28 03:49:59.464672
# Source Data: Surgo
# 
# Mukey: 131976
# Major Component: 27097758 (comppct_r = 90.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80908122   A              13.0     9.0        0.0         0.0         1.45    12.5    70.0    15.8    2.75
# 80908123   Bt             46.0     2.7        0.0         0.0         1.35    50.0     5.3     2.5    0.25
# 80908124   Btg1           89.0     2.7        0.0         0.0         1.35    50.0     5.3     2.5    0.25
# 80908125   Btg2          130.0     2.7        0.0         0.0          1.3    50.0    22.1     5.3    0.25
# 80908126   BCg           185.0    23.0        0.0         0.0          1.4    27.0    55.0    11.1    0.25
# 
# Restricting Layer:
# ksat threshold: 2.00000
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
# 80908122::wilt_pt estimated from wfifteenbar_r and rock
# 80908122::field_cap estimated from wthirdbar_r and rock
# 80908123::wilt_pt estimated from wfifteenbar_r and rock
# 80908123::field_cap estimated from wthirdbar_r and rock
# 80908124::wilt_pt estimated from wfifteenbar_r and rock
# 80908124::field_cap estimated from wthirdbar_r and rock
# 80908125::wilt_pt estimated from wfifteenbar_r and rock
# 80908125::field_cap estimated from wthirdbar_r and rock
# 80908126::wilt_pt estimated from wfifteenbar_r and rock
# 80908126::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-03-28 03:50:01.142902
# Source File: :/wc1/runs/cl/clean-burning-griddle/soils/131976.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> sand loam
# ki -> 400000
# kr -> 8.00E-05
# shcrit -> 2
# avke -> 60
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
0	 'forest'	 'sand loam'	 1.5 	 0.3
'Wahee loam'	 'FSL'	 6	 0.16	 0.75	 400000	 8e-05	 2
	130.0	 1.45	 60	 10.0	 0.2167	 0.1219	 70.0	 12.5	 2.75	 11.3	 4.0	 0.07004	 0.394	 0.01667	 1.499	 53.0	 0.09066	 0.2012
	200.0	 1.35	 60	 10.0	 0.3396	 0.275	 5.3	 50.0	 0.25	 13.0	 4.0	 0.1309	 0.4831	 0.006519	 1.318	 10.46	 0.2129	 0.3445
	460.0	 1.35	 9.72	 10.0	 0.3396	 0.275	 5.3	 50.0	 0.25	 13.0	 4.0	 0.1309	 0.4831	 0.006519	 1.318	 10.46	 0.2129	 0.3445
	890.0	 1.35	 9.72	 1.0	 0.3396	 0.275	 5.3	 50.0	 0.25	 13.0	 4.0	 0.1309	 0.4831	 0.006519	 1.318	 10.46	 0.2129	 0.3445
	1300.0	 1.3	 9.72	 1.0	 0.4188	 0.2708	 22.1	 50.0	 0.25	 13.0	 4.0	 0.1281	 0.4919	 0.008843	 1.295	 17.89	 0.2139	 0.3476
	2000.0	 1.4	 82.8	 1.0	 0.249	 0.1646	 55.0	 27.0	 0.25	 7.5	 4.0	 0.09734	 0.4294	 0.01229	 1.354	 23.83	 0.1498	 0.2771
1 10000.0 0.01
