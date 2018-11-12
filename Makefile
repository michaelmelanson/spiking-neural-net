
all: build run plot

build:
	cargo build --release

run:
	RUST_LOG=info time target/release/neuron

plot:
	gnuplot neuron-trace.gnuplot
	gnuplot spikes.gnuplot
