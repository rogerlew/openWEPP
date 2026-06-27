9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 22:04:02.824555
# Source Data: Surgo
# 
# Mukey: 278708
# Major Component: 27355513 (comppct_r = 45.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 81662269   Oi     X        5.0    55.0        0.0         0.0         0.26     7.0    66.8    10.0    80.0
# 81662276   E              10.0    10.0        0.0         0.0         1.48     3.0    58.0    24.0     4.0
# 81662275   Bhs            13.0    10.0        0.0         0.0         1.02     6.0    53.0    14.0     7.0
# 81662274   Bs1            18.0    10.0        0.0         0.0         1.12     6.0    53.0    14.0     4.0
# 81662273   Bs2            36.0    10.0        0.0         0.0         1.12     3.0    53.0    14.0     4.0
# 81662272   Bs3            61.0    10.0        0.0         0.0          1.2     4.0    65.0    14.0     2.0
# 81662271   BC             84.0    10.0        0.0         7.0         1.46     4.0    65.0    14.0     2.0
# 81662270   Cd     R      165.0     1.0        0.0        13.0         1.74     2.0    80.0    16.0     0.5
# 
# Restricting Layer:
# ksat threshold (um/s): 2.00000
# type: Densic material
# ksat (um/s): 1.00000
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
# 81662276::wilt_pt estimated from wfifteenbar_r and rock
# 81662276::field_cap estimated from wthirdbar_r and rock
# 81662275::wilt_pt estimated from wfifteenbar_r and rock
# 81662275::field_cap estimated from wthirdbar_r and rock
# 81662274::wilt_pt estimated from wfifteenbar_r and rock
# 81662274::field_cap estimated from wthirdbar_r and rock
# 81662273::wilt_pt estimated from wfifteenbar_r and rock
# 81662273::field_cap estimated from wthirdbar_r and rock
# 81662272::wilt_pt estimated from wfifteenbar_r and rock
# 81662272::field_cap estimated from wthirdbar_r and rock
# 81662271::wilt_pt estimated from wfifteenbar_r and rock
# 81662271::field_cap estimated from wthirdbar_r and rock
# 81662270::wilt_pt estimated from wfifteenbar_r and rock
# 81662270::field_cap estimated from wthirdbar_r and rock
# res_lyr_i 7
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
# Build Date: 2026-06-26 22:04:04.256263
# Source File: :/wc1/runs/un/undescended-conserve/soils/278708.sol
# 
# Replacements
# --------------------------
# luse -> short grass
# stext -> sand loam
# ki -> 400000
# kr -> 0.0001
# shcrit -> 2
# avke -> 25
# bd ->
# ksflag -> 1
# ksatadj -> 0
# ksatfac -> 1.5
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 4
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
0	 'short grass'	 'sand loam'	 1.5 	 0.3
'Becket-Skerry association, 0 to 15 percent slopes, extremely stony'	 'GR-FSL'	 7	 0.11	 0.75	 400000	 0.0001	 2
	100.0	 1.48	 25	 10.0	 0.1835	 0.0813	 58.0	 3.0	 4.0	 11.3	 9.0	 0.05309	 0.3616	 0.01288	 1.529	 51.12	 0.07211	 0.173
	130.0	 1.02	 25	 10.0	 0.2244	 0.1128	 53.0	 6.0	 7.0	 11.3	 22.0	 0.06658	 0.4637	 0.006619	 1.538	 168.9	 0.09997	 0.1997
	180.0	 1.12	 25	 10.0	 0.1949	 0.0835	 53.0	 6.0	 4.0	 11.3	 21.0	 0.06425	 0.44	 0.007364	 1.537	 120.9	 0.09424	 0.1953
	200.0	 1.12	 25	 10.0	 0.1759	 0.0671	 53.0	 3.0	 4.0	 11.3	 21.0	 0.05915	 0.433	 0.007203	 1.568	 143.0	 0.08533	 0.181
	360.0	 1.12	 36.0	 10.0	 0.1759	 0.0671	 53.0	 3.0	 4.0	 11.3	 21.0	 0.05915	 0.433	 0.007203	 1.568	 143.0	 0.08533	 0.181
	610.0	 1.2	 36.0	 1.0	 0.1597	 0.0514	 65.0	 4.0	 2.0	 11.3	 28.0	 0.05962	 0.4261	 0.01206	 1.557	 134.9	 0.07985	 0.1819
	1000.0	 1.46	 36.0	 1.0	 0.1781	 0.0642	 65.0	 4.0	 2.0	 11.3	 37.69	 0.05487	 0.3706	 0.01544	 1.548	 63.18	 0.07082	 0.1714
1 10000.0 0.0036
