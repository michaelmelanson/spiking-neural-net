# spiking-neural-net [![Build Status](https://travis-ci.org/michaelmelanson/spiking-neural-net.svg?branch=master)](https://travis-ci.org/michaelmelanson/spiking-neural-net)
A spiking neural network simulation library.

## tl;dr

    make

You should see output like this:

![Sample output](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/sample%20output.png)

It should create output images like this:

Filename      | Image
--------------|-----------
`neuron-trace.png` | ![spikes.png](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/neuron-trace.png)
`spikes.png` | ![spikes.png](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/spikes.png) 
Cropped:     | ![spikes.png](https://github.com/michaelmelanson/spiking-neural-net/blob/master/images/spikes%20(cropped).png) This represents about 1s of time across 1100 neurons.

## About the simulation

This library simulates networks of biologically-inspired neurons. It's highly 
* Spiking neural models are implemented as ordinary differential equations integrated using Euler integration.
* The simuation uses (Specs)[https://github.com/slide-rs/specs], an Entity-Component-System framework with excellent parallelization and performance.
* It's capable of simulation 

## Features

### Neural models

* Izhikevich neurons
  * Model: https://www.izhikevich.org/publications/spikes.htm
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/models/izhikevich.rs
* Hindmarsh-Rose neurons
  * Model: https://en.wikipedia.org/wiki/Hindmarsh%E2%80%93Rose_model
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/models/hindmarsh_rose.rs

### Learning models

* Spike-timing dependent plasticity
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/learning/stdp.rs

### Network organization

* Columnar organization
  * Networks are organized into 100-neuron columns composed of five layers (motor, sensory, afferent, efferent, internal)
  * Code: https://github.com/michaelmelanson/spiking-neural-net/blob/master/src/simulation/mod.rs#L70-L160

## Usage

You will need a Rust development environment, which you can install by visiting https://rustup.rs/ and following the instructions.

Once you have Rust and Cargo installed, you can run a simulation with:

    make
  
This will produce a file called `spikes.png` that looks something like the image above

Note that you can also build the debug version by omitting the `--release` flag, but it will run slowly. This is good if you want to use a debugger, but if you want to simulate any reasonably large networks in real-time you will need to use release mode.

