# Cost Functions
* Also known as a loss function

* Must be an average so that it outputs a single value

* Tells you how far off you are

* y will represent the true value

* a represents the neuron's prediction

* Quadratic Cost Function

	* Kind of the root-mean squared error?

	* $$ C = \frac{1}{2n} \sum_{x} || y(x) - a^L(x) ||^2 $$

	* y(x) is the true value

	* a^L is the activation function output for layer L, where L is the last layer

	* n is the number of points

	* Corresponds to vector inputs and outputs

	* we are subtracting our predictions from the true value

	* the squaring punishes large errors

* We can think of our cost function as such:

	* $$ C(W,B,S^r,E^r)$$

	* W is our neural networks weights

	* B is our network's biases

	* S^r ks the input of a single training sample

	* E^r is the desired output of that training sample

	* In the quadratic function, W and B are encoded

		* a(x) actually accounts for W and B

			* a(x) accounts for z passed into the activation function

	* C gets quite complex for large networks with huge vectors of weights and biases

		* cost function ends up being dependent on lots of weights

			* C(w1,w2,w3,...wn)

	* We need to figure out which weights lead us to the lowest cost

	* **Gradient Descent!**