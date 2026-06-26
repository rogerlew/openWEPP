9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-26 20:45:13.787585
# Source Data: Surgo
# 
# Mukey: 662082
# Major Component: 14197539 (comppct_r = 30.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 40838549   H1            152.04.619861490641729         -           -        1.5196     7.0    66.8    10.0     7.0
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
# 40838549::using default rock content of 55.5%
# 40838549::ksat_r estimated from rosetta2
# 40838549::wilt_pt estimated from rosetta2
# 40838549::field_cap estimated from rosetta2
# 40838549::bd estimated from sand, vfs, and clay
# albedo estimated from om_r (7.0%)
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
# Build Date: 2026-06-26 20:45:14.290200
# Source File: :/wc1/runs/pr/praetorian-talcum/soils/662082.sol
# 
# Replacements
# --------------------------
# luse -> forest
# stext -> sand loam
# ki -> 400000
# kr -> 8.00E-05
# shcrit -> 2
# avke -> 60
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
0	 'forest'	 'sand loam'	 1.5 	 0.3
'Typic Cryumbrepts-Typic Cryorthents-Lithic Cryumbrepts-Lithic Cryorthents (s2225)'	 'VAR'	 2	 0.0365	 0.75	 400000	 8e-05	 2
	200.0	 1.52	 60	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
	1600.0	 1.52	 16.6315	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
