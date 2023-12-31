* Dendrites

	* Input

* Axon

	* Output

* Nucleus

	* Some sort of calculation happens in here

* Perceptron Model introduced in 50s.

	* Not enough computational power to use it

	* AI Winter in the 70s

* Mathematically/In Practice

	* Replace Dendrites with 2 inputs x1 & x2
	
	* Replace Axon with one output y
	
	* Nucleus becomes f(x)
	
	* Biological neuron modeled as a simple perceptron below*

```

*w1 + b ________

x1 --------> | |

*w2 + b | f(x) | ----> y (Output)

x2 --------> |________|

```

* Generalized mathematically:
	* will be expanding this for x to be a *tensor*

$$ \hat{y} = \sum_{i=1}^{n} x_iw_i + b_i $$


* Single perceptron isnt enough to model a neural network