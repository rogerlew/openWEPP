9002
# 
# WEPPcloud v.0.1.0 (c) University of Idaho
# 
# Build Date: 2026-06-25 15:23:18.013251
# Source Data: Surgo
# 
# Mukey: 2756438
# Major Component: 26612693 (comppct_r = 50.0)
# Texture: sand loam
# 
# Chkey   hzname  mask hzdepb_r(cm) ksat_r(um/s) fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 79425845   Oi     X        2.0   400.0        0.0         0.0          0.2     7.0    66.8    10.0    75.0
# 79425850   A              10.0    75.0        0.0         0.0          1.0     4.0    65.0    12.0    10.0
# 79425851   Bw1            14.0   225.0        0.0         0.0          1.0     3.0    65.0     8.3     7.0
# 79425852   Bw2            26.0    25.0        0.0         0.0          1.0     5.0    55.0    14.4     7.0
# 79425846   Bw3            37.0   225.0        0.0         0.0          1.0     1.0    85.0    16.2     6.0
# 79425853   Agb1           44.0    75.0        0.0         0.0          1.0     2.0    65.0    11.8     9.0
# 79425854   Bgb1           51.0    75.0        0.0         0.0          1.0     4.0    65.0    12.0     3.0
# 79425847   Bgb2           60.0   225.0        0.0         0.0          1.0     3.0    70.0     8.9     3.0
# 79425855   Bgb3           66.0    25.0        0.0         0.0          1.0     3.0    65.0    17.0     3.0
# 79425856   2Agb2          85.0    75.0        0.0         0.0          1.0     2.0    65.0    11.8     9.0
# 79425848   2Bgb4         120.0    75.0        0.0        22.0          1.1     5.0    60.0    10.7     1.5
# 79425849   2Bgb5         150.0    75.0        0.0        28.0          1.1     4.0    60.0    10.7    1.25
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
# 79425850::wilt_pt estimated from wfifteenbar_r and rock
# 79425850::field_cap estimated from wthirdbar_r and rock
# 79425851::wilt_pt estimated from wfifteenbar_r and rock
# 79425851::field_cap estimated from wthirdbar_r and rock
# 79425852::wilt_pt estimated from wfifteenbar_r and rock
# 79425852::field_cap estimated from wthirdbar_r and rock
# 79425846::wilt_pt estimated from wfifteenbar_r and rock
# 79425846::field_cap estimated from wthirdbar_r and rock
# 79425853::wilt_pt estimated from wfifteenbar_r and rock
# 79425853::field_cap estimated from wthirdbar_r and rock
# 79425854::wilt_pt estimated from wfifteenbar_r and rock
# 79425854::field_cap estimated from wthirdbar_r and rock
# 79425847::wilt_pt estimated from wfifteenbar_r and rock
# 79425847::field_cap estimated from wthirdbar_r and rock
# 79425855::wilt_pt estimated from wfifteenbar_r and rock
# 79425855::field_cap estimated from wthirdbar_r and rock
# 79425856::wilt_pt estimated from wfifteenbar_r and rock
# 79425856::field_cap estimated from wthirdbar_r and rock
# 79425848::wilt_pt estimated from wfifteenbar_r and rock
# 79425848::field_cap estimated from wthirdbar_r and rock
# 79425849::wilt_pt estimated from wfifteenbar_r and rock
# 79425849::field_cap estimated from wthirdbar_r and rock
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
# Build Date: 2026-06-25 15:23:18.385095
# Source File: :/wc1/runs/op/open-source-thirtieth/soils/2756438.sol
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
'Mountwow, moist-Williwakas-Unicornpeak complex, 0 to 40 percent slopes'	 'MEDL-LS'	 12	 0.23	 0.75	 400000	 8e-05	 2
	100.0	 1.0	 60	 10.0	 0.209	 0.115	 65.0	 4.0	 10.0	 17.0	 0.0	 0.06419	 0.4731	 0.009898	 1.54	 230.8	 0.09168	 0.1963
	140.0	 1.0	 60	 10.0	 0.144	 0.066	 65.0	 3.0	 7.0	 13.0	 0.0	 0.06241	 0.4704	 0.009824	 1.55	 240.6	 0.08858	 0.1913
	200.0	 1.0	 60	 10.0	 0.169	 0.082	 55.0	 5.0	 7.0	 13.0	 0.0	 0.06537	 0.4673	 0.006905	 1.542	 192.6	 0.09785	 0.1974
	260.0	 1.0	 90.0	 10.0	 0.169	 0.082	 55.0	 5.0	 7.0	 13.0	 0.0	 0.06537	 0.4673	 0.006905	 1.542	 192.6	 0.09785	 0.1974
	370.0	 1.0	 810.0	 10.0	 0.145	 0.058	 85.0	 1.0	 6.0	 12.0	 0.0	 0.05965	 0.487	 0.01846	 1.776	 476.7	 0.06509	 0.1449
	440.0	 1.0	 270.0	 10.0	 0.194	 0.098	 65.0	 2.0	 9.0	 16.0	 0.0	 0.06069	 0.4676	 0.009754	 1.561	 250.9	 0.08555	 0.1864
	510.0	 1.0	 270.0	 1.0	 0.111	 0.037	 65.0	 4.0	 3.0	 8.0	 0.0	 0.06419	 0.4731	 0.009898	 1.54	 230.8	 0.09168	 0.1963
	600.0	 1.0	 810.0	 1.0	 0.121	 0.039	 70.0	 3.0	 3.0	 8.0	 0.0	 0.06274	 0.4754	 0.01169	 1.561	 264.8	 0.08547	 0.1893
	660.0	 1.0	 90.0	 1.0	 0.134	 0.043	 65.0	 3.0	 3.0	 8.0	 0.0	 0.06241	 0.4704	 0.009824	 1.55	 240.6	 0.08858	 0.1913
	850.0	 1.0	 270.0	 1.0	 0.194	 0.098	 65.0	 2.0	 9.0	 16.0	 0.0	 0.06069	 0.4676	 0.009754	 1.561	 250.9	 0.08555	 0.1864
	1200.0	 1.1	 270.0	 1.0	 0.142	 0.044	 60.0	 5.0	 1.5	 5.0	 66.46	 0.06325	 0.4471	 0.009215	 1.537	 152.0	 0.09047	 0.1938
	1600.0	 1.1	 270.0	 1.0	 0.118	 0.032	 60.0	 4.0	 1.25	 4.0	 70.48	 0.06149	 0.4445	 0.009153	 1.547	 159.4	 0.08746	 0.189
1 10000.0 0.01
