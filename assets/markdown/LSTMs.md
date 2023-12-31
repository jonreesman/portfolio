One of the issues we face with RNNs is that after some time, the network will begin to "forget" the initial inputs because information is lost at each step in the RNN, which means we need some form of long-term memory for the network.

  

Enter the LSTM, a special neuron that addresses the long-term memory and vanishing gradient problem for RNNs.

  

Instead of a single layer, a typical LSTM will actually have 4 layers.

  

![](LSTM-graphic.png)

  

In general, we have 4 components of the LSTM:

1. Forget Gate - Decides what to forget from the previous memory units

2. Input Gate - Decides what to accept into the neuron

3. Output Gate - Outputs new long term memory

4. Update Gate - Updates the memories

  

A gate optionally lets some memories through. Think of this like a sigmoid function.

  

#### LSTM Step By Step

  

1. Decide what information to throw away from the cell state (forget). We create a Forget gate layer, f_t

* See that we apply the sigmoid function to the input. The closer the output to 1, it means remember it.

$$ f_t = \sigma (W_f \cdot [h_{t-1},x_t] + b_f) $$

  

2. Decide what new information to store in the cell state.

$$ i_t = \sigma (W_i \cdot [h_{t-1},x_t] + b_i) $$

$$ \tilde{C}_t = tanh(W_C \cdot [h_{t-1},x_t] + b_c) $$

  

3. Update the old cell state. Essentially we multiply the old state by f_t, forgetting things we decided aren't important in the forget gate layer. Then we add i_t * the new candidate values for the cell state. The new cell state values are basically added scaled by how much we decided to update each cell value.

$$ C_t = f_t*C_{t-1} + i_t*\tilde{C}_t $$

  

4. Decide what to output

$$ o_t = \sigma (W_o[h_{t-1},x_t]+b_o) $$

$$ h_t = o_t*tanh(C_t) $$

  

[Variations of the LSTM](https://medium.com/nerd-for-tech/what-is-lstm-peephole-lstm-and-gru-77470d84954b):

* Peephole LSTM

* Linearly combines cell state inside the LSTM

* Gated Recurrent Unit (GRU)

* Combines the forget and input gates into a single update gate.

  

### LSTM in Pytorch

  

`CLASS torch.nn.LSTMCell(input_size, hidden_size, bias=True)`

  

Generally, the following is what occurs under the hood in nn.LSTMCell()

$$ i = \sigma (W_{ii}x + b_{ii} + W_{hi}h + b_{hi}) $$

$$ f = \sigma (W_{if}x + b_{if} + W_{hf}h + b_{hf}) $$

$$ g = tanh(W_{ig}x + b_{ig} + W_{hg}h + b_{hg}) $$

$$ o = \sigma (W_{io}x + b_{io} + W_{ho}h + b_{ho}) $$

$$ c' = f * c + i * g $$

$$ h' = o * tanh(c') $$

  

Here, * is the Hadamard product.