
set terminal postscript eps enhanced color font 'Times-Roman,16'
set output "gateway_module_latency.eps"

set style fill solid 1.00 border 0
set style histogram errorbars gap 2 lw 1
set style data histogram

#set xtics rotate by 0
set grid ytics
set xlabel "Number of messages submitted in parallel"
set ylabel "Latency (ms)"
set yrange [0:*]
set key top left

set datafile separator "\t"

plot \
	'gateway_module_latency.dat' using 2:3:xtic(1) ti "Substrate Broadcast", \
	'gateway_module_latency.dat' using 4:5 ti "Substrate Finalized (Aura + GRANDPA)", \
	'gateway_module_latency.dat' using 6:7 ti "Fabric Commited (PBFT)"
