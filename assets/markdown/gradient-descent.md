### Gradient Descent

* Imagine we are trying to find the trough of a sloped curve using derivatives

	* Suspend belief on just taking a derivative and setting it equal to zero... not feasible for real cost functions

	* We can take the slope at different intervals until we find the trough (has a slope of zero), these intervals are steps

	* If we take too small of steps, it takes forever.

	* If we take too large of steps, we risk overshooting

	* The step size is known as the **learning rate**

	* The learning rate doesnt have to be constant

		* We can adapt it over time, starting with a large step size and getting smaller as we approach zero

		* **adaptive gradient descent**

* *Adam: A Method for Stochastic Optimization*

	* Adaptive step size that gets smaller and smaller

	* Paper by Kingma and Ba

	* Theres other alternatives like RMSPrep, SGDNesterov, AdaDelta

	* AdaGrad

	* Very performant algorithm

* When dealing with N-dimensional vectors (tensors), the notation changes from derivative to gradient

	* Meaning our cost function becomes

$$ \triangledown C(w_1,w_2,...,w_n) $$