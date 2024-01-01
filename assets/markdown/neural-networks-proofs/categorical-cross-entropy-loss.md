# Categorical Cross Entropy Loss Function Derivation
## Formula
The standard CCE formula is:
	$$ L_i = - log(\hat{y}_{i,k}),\quad where\ k\ is\ an\ index\ of\ ''true''\ probability$$
- L_i denotes sample loss value, i -- i-th sample in a set, k -- index of the target label (ground-true label), y -- target values and y-hat -- predicted values

This is just the abbreviated version for calculating the loss value itself. All we need is the output of the Softmax at the index of the correct class.

For this example, we will use the full formula:

$$ L_i = -\sum_i y_{i,j}log(\hat{y}_{i,j}) $$
- Where L_i denotes the sample loss value, i -- ith sample in the set, j -- label/output index, y-- target values and y-hat -- predicted values

We need this full function to calculate the following gradient descent example, which is composed of partial derivates of the loss function with respect to each of its inputs (here, the outputs of the Softmax function)

## Gradient Descent Example

### Gradient Equation

$$ \frac{\partial L_i}{\partial \hat{y}_{i,j}} = \frac{\partial}{\partial \hat{y}_{i,j}} [- \sum_j y_{i,j}log(\hat{y}_{i,j})] = -\sum_jy_{i,j} * \frac{\partial}{\partial\hat{y}_{i,j}}log(\hat{y}_{i,j}) $$
This equation is more valuable for calculating gradient descent because it takes all inputs as parameters, which we need to calculate the partial derivatives with respect to all inputs. We can go ahead and move the summation outside the derivative, because the derivate of a sum is the same as the sum of its derivatives, and pull out y_i,j, as it is being treated as a constant in this partial derivative since we are not calculating the derivatives with respect to it.

Solving for the derivative of the log, which is the reciprocal of its parameters, and using Lagrange's notation (prime notation):

$$ f\left(x\right) = log\left(h\left(x\right)\right)\ \rightarrow f'\left(x\right) = \frac{1}{h\left (x \right) } \cdot h'\left( x \right)  $$
We can solve it further (now using Leibniz's notation):
$$ f(x) = log(x) \rightarrow \frac{d}{dx}f(x) = \frac{d}{dx}log(x) = \frac{1}{x}\cdot\frac{d}{dx}x = \frac{1}{x}\cdot1 = \frac{1}{x} $$
Now that we have the derivative, lets apply it:

$$ = -\sum_jy_{i,j} \cdot \frac{1}{\hat{y}_{i,j}}\cdot\frac{\partial}{\partial\hat{y}_{i,j}}\hat{y}_{i,j} $$
The partial derivative of a value with respect to this equals 1:

$$ -\sum_jy_{i,j} \cdot \frac{1}{\hat{y}_{i,j}}\cdot 1 = -\sum_j\frac{y_{i,j}}{\hat{y}_{i,j}}  $$
We are calculating the partial derivative with respect to the y given j, the sum is being performed over a single element and can be omitted:

$$ = -\frac{y_{i,j}}{\hat{y}_{i,j}} $$
Therefore, the derivative of this loss function with respect to its inputs (predicted values at the -ith sample) equals the negative ground-truth vector, divided by the vector of the predicted values (which is the output vector of the softmax, since it is our activation function)