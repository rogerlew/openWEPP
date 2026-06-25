9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-25 15:06:52.072587
# Source Data: Surgo
# 
# Mukey: 153349
# Major Component: 26396336 (comppct_r = 75.0)
# Texture: silt loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 78753739   Oi     X        3.0   350.0        0.0         0.0          0.2     6.0    27.8    20.7    75.0
# 78753740   Oe     X        6.0   100.0        0.0         0.0          0.2     6.0    27.8    20.7    70.0
# 78753737   A              11.0     9.0        0.0         0.0         0.85     6.0    27.8    20.7     6.5
# 78753738   Bw1            44.0     9.0        0.0        12.0         0.85     6.0    27.8    20.7     2.0
# 78753741   2Bw2           72.0    23.0        0.0        13.0          1.4     6.0    65.0    12.0    0.25
# 78753742   2C            152.0    28.0       16.0        16.0          1.5     3.5    65.5    12.4    0.25
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
# 78753737::wilt_pt estimated from wfifteenbar_r and rock
# 78753737::field_cap estimated from wthirdbar_r and rock
# 78753738::wilt_pt estimated from wfifteenbar_r and rock
# 78753738::field_cap estimated from wthirdbar_r and rock
# 78753741::wilt_pt estimated from wfifteenbar_r and rock
# 78753741::field_cap estimated from wthirdbar_r and rock
# 78753742::wilt_pt estimated from wfifteenbar_r and rock
# 78753742::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-25 15:06:52.469830
# Source File: :/wc1/runs/li/listed-scar/soils/153349.sol
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
0	 'forest'	 'silt loam'	 1.5 	 0.3
'Bouldercreek ashy silt loam, 35 to 65 percent slopes'	 'ASHY-SIL'	 5	 0.16	 0.75	 1000000	 5e-05	 1.5
	110.0	 0.85	 40	 10.0	 0.2418	 0.0769	 27.8	 6.0	 6.5	 16.5	 9.0	 0.07351	 0.5155	 0.002839	 1.638	 309.7	 0.1138	 0.1904
	200.0	 0.85	 40	 10.0	 0.2152	 0.0441	 27.8	 6.0	 2.0	 7.5	 25.2	 0.07351	 0.5155	 0.002839	 1.638	 309.7	 0.1138	 0.1904
	440.0	 0.85	 32.4	 10.0	 0.2152	 0.0441	 27.8	 6.0	 2.0	 7.5	 25.2	 0.07351	 0.5155	 0.002839	 1.638	 309.7	 0.1138	 0.1904
	720.0	 1.4	 82.8	 1.0	 0.114	 0.034	 65.0	 6.0	 0.25	 3.4	 65.2	 0.05915	 0.3873	 0.01454	 1.534	 67.92	 0.07766	 0.1819
	1600.0	 1.5	 100.8	 1.0	 0.072	 0.018	 65.5	 3.5	 0.25	 2.3	 83.68	 0.05348	 0.3616	 0.01624	 1.55	 58.46	 0.06846	 0.1679
1 10000.0 0.01
