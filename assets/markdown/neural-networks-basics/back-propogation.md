# Back Propogation
* Hadamard Product

	*  $$ \begin{bmatrix} 1 \\ 2 \end{bmatrix} \odot \begin{bmatrix} 3 \\ 4 \end{bmatrix} = \begin{bmatrix} 1 \cdot 3 \\ 2 \cdot 4 \end{bmatrix} = \begin{bmatrix} 3 \\ 4 \end{bmatrix}$$
* Step 1

	* Using input x, set the activation function a for the input layer, then a feeds into the next layer and so on
$$ z = wx+b $$
$$ a = \sigma(z) $$
* Step 2
	* For each layer, compute:
$$ w^L = w^La^{l-1}+b^L $$
$$ a^L=\sigma(z^L) $$
* Step 3

	* Compute our error vector

	* Expresses the range of change of C with respect to the output activations
$$ \delta^L = \triangledown_aC\odot\sigma'(z^L)$$
$$ \triangledown_aC=(a^L-y) $$
* Step 4
	* $$ \delta^l=(w^{l+1})^T\delta^{l+1}\odot\sigma'(z^l) $$
	* Generalized error for any layer l

	* note: l != 1. l is any given layer. L is the last layer.

	* Here, T is the transpose.

	* (w^(l+1))^T is the transpose weight matrix. Think of it intuitively as moving the error backwards through the network. Gives a measure of the error at any given layer l

	* We then take the Hadamard product. This moves the error backward through the activation function in layer l, giving us the error sigma^l in the weighted input to layer l
	* $$ \odot\sigma'(z')$$
	* the gradient of the cost function is given by:
		* For each layer: L-1,L-2,... we compute
$$ \frac{\partial C}{\partial w_{jk}^l} = a_k^{l-1}\delta_j^l $$

$$ \frac{\partial C}{\partial b^l_j} = \delta^l_j $$

## Back-propagation Example
[See Blog](https://mattmazur.com/2015/03/17/a-step-by-step-backpropagation-example/)
![](/assets/resources/neural_network-9.png)

#### Backwards Pass
Figure out the *total net input* to each hidden layer neuron, squash the total net input using an *activation function*, then repeat the process with the output layer neurons. **Sigmoid Function Example** The target outputs are o1 = 0.01 and o2 = 0.99

$$ net_{h1} = w_i \cdot i_1 + w_2 \cdot i_2 + b_1 \cdot 1 $$

$$ net_{h1} = 0.15 \cdot 0.05 + 0.2 \cdot 0.1 + 0.35 \cdot 1 = 0.3775 $$

Apply Logistics function...

$$ out_{h1} = \frac{1}{1+e^{-net_{h1}}} = \frac{1}{1+e^{-0.3775}} = 0.593269992 $$

  

Now for h2

$$ net_{h2} = 0.25*0.05 + 0.3*0.1 + 0.35 * 1 = 0.3925 $$

$$ out_{h2} = \frac{1}{1+e^{-0.3925}} = 0.596884378$$

  

Output Layer

$$ net_{o1} = w_5*out_{h1} + w_6*out_{h2} + b_2 * 1 $$

$$ net_{o1} = 0.4*0.593269992+0.45*0.596884378 + 0.6*1 = 1.105905967 $$

$$ out_{o1} = \frac{1}{1+e^{-net_{o1}}} = \frac{1}{1+e^{-1.105905967}} = 0.75136507 $$

  

$$ net_{o2} = 0.5*0.593269992+0.55*0.596884378+0.6*1=1.2249214039 $$

$$ out_{o2} = \frac{1}{1+e^{-1.2249214039}} = 0.772928465 $$

  

Calculate the total error using the [*squared error function*](http://en.wikipedia.org/wiki/Backpropagation#Derivation)

$$ E_{total} = \sum\frac{1}{2}(target-output)^2 $$

$$ E_{o1} = \frac{1}{2}(target_{o1}-out_{o1})^2 = \frac{1}{2}(0.01-0.75136507)^2 = 0.274811083 $$

$$ E_{o2} = \frac{1}{2}(target_{o2}-out_{o2})^2 = \frac{1}{2}(0.99-0.772928465)^2 = 0.023560026 $$

$$ E_{total} = E_{o1}+E_{o2} = 0.274811083+0.023560026=0.298371109 $$
#### The Forward Pass
Figure out the *total net input* to each hidden layer neuron, squash the total net input using an *activation function*, then repeat the process with the output layer neurons. **Sigmoid Function Example** The target outputs are o1 = 0.01 and o2 = 0.99

  

$$ net_{h1} = w_i*i_1 + w_2*i_2 + b_1 * 1 $$

$$ net_{h1} = 0.15 * 0.05 + 0.2*0.1 + 0.35*1 = 0.3775 $$

  

Apply Logistics function...

$$ out_{h1} = \frac{1}{1+e^{-net_{h1}}} = \frac{1}{1+e^{-0.3775}} = 0.593269992 $$

  

Now for h2

$$ net_{h2} = 0.25*0.05 + 0.3*0.1 + 0.35 * 1 = 0.3925 $$

$$ out_{h2} = \frac{1}{1+e^{-0.3925}} = 0.596884378$$

  

Output Layer

$$ net_{o1} = w_5*out_{h1} + w_6*out_{h2} + b_2 * 1 $$

$$ net_{o1} = 0.4*0.593269992+0.45*0.596884378 + 0.6*1 = 1.105905967 $$

$$ out_{o1} = \frac{1}{1+e^{-net_{o1}}} = \frac{1}{1+e^{-1.105905967}} = 0.75136507 $$

  

$$ net_{o2} = 0.5*0.593269992+0.55*0.596884378+0.6*1=1.2249214039 $$

$$ out_{o2} = \frac{1}{1+e^{-1.2249214039}} = 0.772928465 $$

  

Calculate the total error using the [*squared error function*](http://en.wikipedia.org/wiki/Backpropagation#Derivation)

$$ E_{total} = \sum\frac{1}{2}(target-output)^2 $$

$$ E_{o1} = \frac{1}{2}(target_{o1}-out_{o1})^2 = \frac{1}{2}(0.01-0.75136507)^2 = 0.274811083 $$

$$ E_{o2} = \frac{1}{2}(target_{o2}-out_{o2})^2 = \frac{1}{2}(0.99-0.772928465)^2 = 0.023560026 $$

$$ E_{total} = E_{o1}+E_{o2} = 0.274811083+0.023560026=0.298371109 $$

  

#### The Backwards Pass

  

The goal of backpropogation is to update each of the weights in the network so that the actual output is closer to the target output, minimizing the error for each output neuron and the network as a whole

  

##### **Output Layer**

  

As an example, lets look at w5 to see how a change affects the total error. [Chain rule](http://en.wikipedia.org/wiki/Chain_rule)

$$ \frac{\partial E_{total}}{\partial w_5}=\frac{\partial E_{total}}{\partial out_{o1}}*\frac{\partial out_{o1}}{\partial net_{o1}} * \frac{\partial net_{o1}}{\partial w_5} $$

  

How does the total error change with respect to the output?

$$ E_{total} = \frac{1}{2}(target_{o1}-out_{o1})^2 + \frac{1}{2}(targer_{o2}-out_{o2})^2 $$

$$ \frac{\partial E_{total}}{\partial out_{o1}}=2*\frac{1}{2}(target_{o1}-out_{o1})^{2-1} *-1+0 $$

$$ \frac{\partial E_{total}}{\partial out_{o1}}=-(target_{o1}-out_{o1})=-(0.01-0.75136507) = 0.74136507 $$

  

Next, we will compute the partial derivative of o_1 with respect to its net total input, eg. how much out_o1 changes with respect to net_o1

The [partial derivative of the logistics function](http://en.wikipedia.org/wiki/Logistic_function#Derivative)

$$ out_{o1} = \frac{1}{1+e^{-net_{o1}}} $$

$$ \frac{\partial out_{o1}}{\partial net_{o1}} = out_{o1}(1-out_{o1}) = 0.75136507(1-0.75136507) = 0.186815602 $$

  

Lastly, how does the total net input of o_1 change with respect to w_5?

$$ net_{o1} = w_5*out_{h1} +w_6*out_{h2}+b_2*1 $$

$$ \partial net_{o1} = 1*out_{h1}*w_5^{1-1}+0+0=out_{h1}=0.593269992 $$

  

Putting it all together:

$$ \frac{\partial E_{total}}{\partial w_5}=\frac{\partial E_{total}}{\partial out_{o1}}*\frac{\partial out_{o1}}{\partial net_{o1}} * \frac{\partial net_{o1}}{\partial w_5} $$

$$ \frac{\partial E_{total}}{\partial w_5}=0.74136507*0.186815602*0.593269992= 0.082167041 $$

  

Applying this value, we subtract it from the current weight (optionally multiplied by some learning rate, which we set to 0.5)

$$ w_5^+ = w_5 - \eta * \frac{\partial E_{total}}{\partial w_5}=0.5-0.5*0.082167041 = 0.35891648 $$

  

We repeat this process for the other weights at the output layer to get:

$$ w_5^+=0.35891648 $$

$$ w_6^+=0.408666186 $$

$$ w_7^+=0.511301270 $$

$$ w_8^+=0.561370121 $$

And we perform the actual updates in the neural network after we have the new weights leading into the hidden layer neurons. So, we dont recalculate before backpropagating to the hidden layer input weights.

  
  

##### **Hidden Layer**

  

Next, we will calculate for w_1, w_2, w_3, and w_4

$$ \frac{\partial E_{total}}{\partial w_1}=\frac{\partial E_{total}}{\partial out_{h1}}*\frac{\partial out_{h1}}{\partial net_{h1}}*\frac{\partial net_{h1}}{\partial w_1} $$

  

![](/assets/resources/nn-calculation.png)

  

We do this slightly differently, because the output of h1 (out_h1) affecrt both out_o1 and out_o2. So we have to account for this effect on both output neurons.

$$ \frac{E_{total}}{\partial out_{h1}}=\frac{\partial E_{o1}}{\partial out_{h1}}+\frac{\partial E_{o2}}{\partial out_{h1}} $$

$$ \frac{\partial E_{o1}}{\partial out_{h1}}=\frac{\partial E_{o1}}{\partial net_{o1}}*\frac{\partial net_{o1}}{\partial out_{h1}} $$

  

We can calculate some of this using the values from earlier:

$$ \frac{\partial E_{o1}}{\partial net_{o1}}=\frac{\partial E_{o1}}{\partial out_{o1}}*\frac{\partial out_{o1}}{\partial net_{o1}}=0.74136507*0.186815602=0.138498562 $$

  

And the following:

$$ net_{o1} = w_5*out_{h1} + w_6*out_{h2}+b_2*1 $$

$$ \frac{\partial net_{o1}}{\partial out_{h1}} = w_5 = 0.40 $$

  

Plugging them in:

$$ \frac{\partial E_{o1}}{\partial out_{h1}}=\frac{\partial E_{o1}}{\partial net_{o1}}*\frac{\partial net_{o1}}{\partial out_{h1}}=0.138498562*0.40 = 0.055399425 $$

  

Doing the same for:

$$ \frac{\partial E_{o2}}{\partial out_{h1}}=-0.019049119 $$

  

Therefore:

$$ \frac{\partial E_{total}}{\partial out_{h1}}=\frac{\partial E_{o1}}{\partial out_{h1}}+\frac{\partial E_{o2}}{\partial out_{h1}}=0.055399425 - 0.019049119 = 0.36350306 $$

  

Next:

$$ out_{h1}=\frac{1}{1+e^{-net_{h1}}} $$

$$ \frac{\partial out_{h1}}{\partial net_{h1}}=out_{h1}(1-out_{h1})=0.59326999(1-0.59326999)=0.241300709 $$

  

We can calculate the partial derivative of the total net input to h1 with respect to w1 the same as we did for the output neron:

$$ net_{h1} = w_1*i_1 + w_3*i_2 + b_1*1 $$

$$ \frac{\partial net_{h1}}{\partial w_1}=i_1=0.05$$

  

Putting it all together:

$$ \frac{\partial E_{total}}{\partial w_1}=\frac{\partial E_{total}}{\partial out_{h1}}*\frac{\partial out_{h1}}{\partial net_{h1}}*\frac{\partial net_{h1}}{\partial w_1} $$

$$ \frac{\partial E_{total}}{\partial w_1}=0.036350306*0.241300709*0.05=0.000438568 $$

  

Now to update w1

$$ w_1^+ = w_1 - \eta*\frac{\partial E_{total}}{\partial w_1}=0.15-0.5*0.000438568=0.149780716 $$

  

And the rest:

$$ w_2^+ = 0.19956143 $$

$$ w_3^+ = 0.24975114 $$

$$ w_4^+ = 0.29950229 $$

  

If we ran this again, our network error will decrease from 0.298371109 to 0.291037924. Repeat this 10,0000 times, the error plummets to 0.0000351085.