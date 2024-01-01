# Activation Functions

## General Model of a Neuron

$$ x*w + b $$

* w is how much weight or strength we apply to the given input, specific to the neuron to neuron connection

* b is bias, offset value, setting a threshold that establishes when x+w has an effect

We are able to manipulate the results of the individual neuron output by applying an activation function to it. This gives us the ability to filter and normalize outputs, and gains us the ability to have neurons produce a 0 output if they don't meet a certain threshold.

$$ z = x*w + b $$

* Pass z through activation function

## Step Function

The step function is a simple function that bounces input from 0 to 1 discretely. It is used by the simple preceptron model. Multiple activation functions will model a continuous, differential alternative to the Step Function.

![](/assets/resources/activation_functions/step.png)


## Sigmoid Function

The Sigmoid activation function effectively normalizes values to a range between 0 and 1, making it great for classification problems. It is non-linear and analog in nature, as opposed to the discrete nature of the step function.

$$ f(z) = \frac{1}{1+e^{(-z)}} $$

![](/assets/resources/activation_functions/sigmoid.png)

The Sigmoid poses a fundamental problem for activation functions called the **vanishing gradient**.

The vanishing gradient problem is where as inputs become greater in magnitude, the effect on the output is reduced by nature of the double asymptotic nature of the curve.

To better understand the problem, it helps to visualize the Sigmoid Function with its derivative.

$$ S'(x) = S(x) \cdot (1 - S(x)) $$

Looking at the graph, it becomes apparent that relatively small inputs have the most drastic effect, while much larger values have near-zero derivative values, producing a stunted effect.

![](/assets/resources/activation_functions/sigmoid_and_derivative.png)


## Hyperbolic Tangent

The Hyperbolic Tangent functions values are similar to the Sigmoid Function, but its values are centered around 0, producing a range of (-1,1) as opposed to the Sigmoid (0,1).

The Hyperbolic Tangent function is great for classification problems as well, providing a larger range of values for dividing into more classifications.

It too suffers from the vanishing gradient problem.

$$ cosh x = \frac{e^x + e^-x}{2} $$

$$ sinh x = \frac{e^x - e^-x}{2} $$

$$ tanh x = \frac{sinh x}{cosh x} $$

![](/assets/resources/activation_functions/htan.png)

  

## Rectified Linear Unit

Remarkebly simple activation function that is simply defined as 

$$f(z) = max(0, z)$$

Or, in more explicit mathematical notation:

$$ 
R\left(z \right) = \Biggl\{
	\begin{matrix}
		\\
		z , & z>0 \\
		\\
		0 , & z \leq 0
	\end{matrix}
$$


The RELU function is a great alternative to the Sigmoid Activation Function as it is significantly less computationally expensive, and also better handles the vanishing gradient problem.

* great performance, will often rely on this
  
![](/assets/resources/activation_functions/relu.png)

[Extra Functions](en.wikipedia.org/wiki/Activation_function)]
