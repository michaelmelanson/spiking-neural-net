set title 'Neuron plot'
set ylabel "Membrane potential (mV)"
set xlabel "Time (ms)"


set xrange [0:10000]
set terminal png
set output 'output.png'

plot 'output.csv' using 1:2 with lines, 'output.csv' using 1:3 with lines, 'output.csv' using 1:4 with lines, 'output.csv' using 1:5 with lines
