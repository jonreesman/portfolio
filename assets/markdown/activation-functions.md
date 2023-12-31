* x*w + b

	* w is how much weight or strength we apply to the given input, specific to the neuron to neuron connection

	* b is bias, offset value, setting a threshold that establishes when x+w has an effect

* Now need to set boundaries for overall output

	* z = x*w + b

	* Pass z through activation function

* **Step Function**

	* Simple function that bounces input to 0 or 1

	* Works for classification problems

	* Sigmoid Function

$$ f(z) = \frac{1}{1+e^{(-z)}} $$

  

```

y

^

1 | ________

| |

| |

0 |________|

|_________________> x

```

* **Hyperbolic Tangent**

	* Outputs between -1 and 1 instead of 0 to 1 like sigmoid

	* Pretty analog so i cant graph it in ascii...

$$ cosh x = \frac{e^x + e^-x}{2} $$

$$ sinh x = \frac{e^x - e^-x}{2} $$

$$ tanh x = \frac{sinh x}{cosh x} $$

  

* **Rectified Linear Unit**

	* max(0,z)

		* Uncapped upper bound

		* Lower bound is zero

		* z = wx + b

	* ReLu function

	* great way to deal with a vanishing gradient

	* great performance, will often rely on this

[Extra Functions](en.wikipedia.org/wiki/Activation_function)]
