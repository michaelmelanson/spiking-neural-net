set title 'Network activity'
set ylabel "Neuron"
set xlabel "Time (ms)"

set terminal png size 22000,3915
set output 'spikes.png'
set palette grey
set autoscale noextend

set xtics 1000
set ytics 100

# set xrange [:1000]
unset colorbox

plot 'spikes.out' matrix using 2:1:3 with image