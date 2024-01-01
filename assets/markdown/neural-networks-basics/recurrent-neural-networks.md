# Recurrent Neural Networks

![](/assets/resources/rnn_types.png)

  

### Deep NN's

With backpropogation, its possible that the gradient can vanish or blow up. So how do we solve this?

  

Take the sigmoid activation function as an example.

  

Below is a graph of the sigmoid function and its derivative. For n hidden layers, we are essentially multiplying these n small derivatives together, essentially really muting the outp we get towards the horizontal asymptotes. The gradient ends up decreasing exponentially as we propogate towards the initial layer.

    

Enter ReLu. It doesn't saturate positive values:

  
![](/assets/resources/ReLU-function-graph.png)
  

We can solve the lack of negative values by using the ["Leaky ReLu"](https://paperswithcode.com/method/leaky-relu#:~:text=Leaky%20Rectified%20Linear%20Unit%2C%20or,is%20not%20learnt%20during%20training.)

Then there's also the Exponential Linear Unit [(ELU)](https://paperswithcode.com/method/elu#:~:text=The%20Exponential%20Linear%20Unit%20(ELU)%20is%20an%20activation%20function%20for,but%20with%20lower%20computational%20complexity.)

Other solutions:

* [batch normalization](https://machinelearningmastery.com/batch-normalization-for-training-of-deep-neural-networks/)

* Choosing different initialization of weights, as in [Xavier initialization](https://cs230.stanford.edu/section/4/)

* Gradient Clipping - gradients are clipped off before some predetermined limit (eg, 1 and -1)

  

Why is this relevant to RNNs? The above solutions aren't convenient (possibly even a waste of time) due to the length of the time series input in RNNs, which slows down training. RNNs present their own gradient challenges that we use Long Short Term Memory (LSTM) and GRU to fix.