9002
#
# WEPPcloud v.0.1.0 (c) University of Idaho
#
# Build Date: 2026-04-28 19:18:07.992092
# Source Data: Surgo
#
# Mukey: 62571
# Major Component: 27032588 (comppct_r = 85.0)
# Texture: clay loam
#
# Chkey   hzname  mask hzdepb_r  ksat_r fraggt10_r frag3to10_r dbthirdbar_r    clay    sand     vfs      om
# ------------------------------------------------------------------------------------------------------------
# 80723447   Oi     X        3.0   373.0        0.0         0.0          0.2    15.0    35.0     5.0    75.0
# 80723448   H1             23.0     3.0        0.0         0.0          1.2    35.0    17.3    10.6     6.0
# 80723449   H2             99.0     3.0        0.0         0.0          1.2    52.5    18.2     5.6    1.75
# 80723450   H3            124.016.631501366310225         -           -        1.5196     7.0    66.8    10.0     7.0
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
# 80723448::wilt_pt estimated from wfifteenbar_r and rock
# 80723448::field_cap estimated from wthirdbar_r and rock
# 80723449::wilt_pt estimated from wfifteenbar_r and rock
# 80723449::field_cap estimated from wthirdbar_r and rock
# 80723450::using default rock content of 55.5%
# 80723450::ksat_r estimated from rosetta2
# 80723450::wilt_pt estimated from rosetta2
# 80723450::field_cap estimated from rosetta2
# 80723450::bd estimated from sand, vfs, and clay
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
# Source soil utility: 9002.0migration
# Build Date: 2026-04-28 19:18:19.956966
# Source File: soils/62571.sol
#
# Replacements
# --------------------------
# luse -> forest low sev fire
# stext -> clay loam
# ki -> 1500000
# kr -> 5.00E-05
# shcrit -> 0.5
# avke -> 18
# bd ->
# ksflag -> 0
# ksatadj -> 0
# ksatfac -> 1.3
# ksatrec -> 0.3
# pmet_kcb -> 0.95
# pmet_rawp -> 0.8
# rdmax -> 0.3
# xmxlai -> 4
# keffflag -> 1
# lkeff -> 10
# plant.data.decfct -> 1
# plant.data.dropfc -> 1
#
# h0_min_depth = None
# h0_max_om = None
#
# Source soil utility: modify_initial_sat(initial_sat=0.75)
Any comments:
1 0
0	 'forest low sev fire'	 'clay loam'	 1.3 	 0.3
'Peavine silty clay loam, 30 to 60 percent slopes'	 'SICL'	 4	 0.16	 0.75	 1500000	 5e-05	 0.5
	200.0	 1.2	 18	 10.0	 0.346	 0.23	 17.3	 35.0	 6.0	 20.0	 0.0	 0.1127	 0.4898	 0.005134	 1.398	 27.23	 0.1795	 0.3017
	230.0	 1.2	 10.8	 10.0	 0.346	 0.23	 17.3	 35.0	 6.0	 20.0	 0.0	 0.1127	 0.4898	 0.005134	 1.398	 27.23	 0.1795	 0.3017
	990.0	 1.2	 10.8	 1.0	 0.434	 0.283	 18.2	 52.5	 1.75	 17.5	 0.0	 0.1321	 0.5245	 0.00852	 1.302	 28.03	 0.2227	 0.3594
	1400.0	 1.52	 59.8734	 1.0	 0.188	 0.09	 66.8	 7.0	 7.0	 11.3	 55.5	 0.05875	 0.3645	 0.01673	 1.516	 46.72	 0.07638	 0.181
1 10000.0 0.01
