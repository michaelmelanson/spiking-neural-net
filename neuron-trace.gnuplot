set title 'Neuron trace'
set ylabel "Membrane potential (mV)"
set xlabel "Time (ms)"


set terminal png
set output 'neuron-trace.png'

plot 'neuron-trace.csv' using 1:2 with lines
