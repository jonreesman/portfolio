# Softmax Activation Function
## Softmax Equation

$$ A = S_{i,j} = \frac{e^{z_{i,j}}}{\sum^L_{l=1}e^{z_{i,l}}} $$

 - *S_i,j*  - denotes the j-th Softmax's output of the i-th sample
 - *z* - the input array (output vectors from previous layer)
 - *z_i,j* - the j-th Softmax's input of the i-th sample
 - *L* - number of inputs

The Softmax function equals the exponentiated input divided by the sum of all exponentiated inputs.

We exponentiate all the values, then divide each by the sum of all of them, normalizing them in the process.

This means, each input affects the Softmax output of all of them. Hence, we calculate the partial derivative of each output with respect to each input.

## Softmax Derivative

$$ A \rightarrow \frac{\partial S_{i,j}}{\partial z_{i,k}} = \frac{\partial \frac{e^{z_{i,j}}}{\sum^L_{l=1}e^{z_{i,l}}}}{\partial z_{i,k}} = B$$
 - *z_i,k* - kth Softmax input of the i-th sample

Notice this derivative is the partial derivative of one output with respect to one input.

We will start by applying the *quotient rule*:

$$ h(x) = \frac{f(x)}{g(x)} \rightarrow h'(x) = \frac{f'(x)g(x) - f(x)g'(x)}{g(x)^2} $$

$$ B = \frac{ \frac{\partial}{\partial z_{i,k}} e^{z_{i,j}} \cdot \sum^L_{l=1}e^{z_{i,l}} - e^{z_{i,j}} \cdot \frac{\partial}{\partial z_{i,k}}\sum^L_{l=1}e^{z_{i,l}} }{(\sum^L_{l=1}e^{z_{i,l}})^2}$$
Now we can solve for each partial derivative separately:

$$ \frac{\partial}{\partial z_{i,k}}\sum^L_{l=1}e^{z_{i,l}} $$

Notice, this is the derivative of the sum of the constant, e, raised to the power z_i,l with respect to z_i,k. The derivative of a sum is the same as the sum of the derivatives.

Next, remember the derivative of euler's number raised to the power of some variable:

$$ \frac{d}{dx}e^x = e^x $$
Another interesting property here, is that with respect to the summation we are deriving, only one of the values of l, where l=1...L, will be k. So since this is a partial derivative, all other values are treated as constants and thus are 0.

$$ \frac{\partial}{\partial z_{i,k}}\sum^L_{l=1}e^{z_{i,l}}  = 
\frac{\partial}{\partial z_{i,k}}e^{z_{i,1}} + \frac{\partial}{\partial z_{i,k}} e^{z_{i,2}} + ... + \frac{\partial}{\partial z_{i,k}} e^{z_{i,k}} + ... \frac{\partial}{\partial z_{i,k}} e^{z_{i,L-1}} + \frac{\partial}{\partial z_{i,k}}e^{z_{i,L}} $$
$$ = 0 + 0 + ... + e^{z_{i,k}} + ... + 0 + 0 = e^{z_{i,k}} $$

Now, for the other derivative in the numerator.

$$ \frac{\partial}{\partial z_{i,k}}e^{z_{i,j}} $$
- if j != k, it becomes 0
- if j=k, it becomes e to the power of z_i,j
We must calculate the derivative for both cases.

Starting with j=k:
$$ = \frac{  e^{z_{i,j}} \cdot \sum^L_{l=1}e^{z_{i,l}} - e^{z_{i,j}} \cdot e^{z_{i,k}} }{(\sum^L_{l=1}e^{z_{i,l}})^2} $$
Lets pull out e^zi,j:
$$ = \frac{  e^{z_{i,j}} \cdot (\sum^L_{l=1}e^{z_{i,l}} -  e^{z_{i,k}}) }{(\sum^L_{l=1}e^{z_{i,l}})^2} = \frac{e^{z_{i,j}}}{\sum^L_{l=1}e^{z_{i,l}}} \cdot \frac{\sum^L_{l=1}e^{z_{i,l}} -  e^{z_{i,k}}}{\sum^L_{l=1}e^{z_{i,l}}} = $$
$$ = \frac{e^{z_{i,j}}}{\sum^L_{l=1}e^{z_{i,l}}} \cdot (\frac{\sum^L_{l=1}e^{z_{i,l}}}{\sum^L_{l=1}e^{z_{i,l}}} - \frac{e^{z_{i,k}}}{\sum^L_{l=1}e^{z_{i,l}}}) = $$
Notice, how both sides simplify down to the Softmax function itself, with one operand simplifying down to 1.

$$ = S_{i,j} \cdot (1-S_{i,k}) $$

Now in the other case when j!=k:

$$ = \frac{0\cdot\sum^L_{l=1}e^{z_{i,l}} - e^{z_{i,j}}\cdot e^{z_{i,k}}}{(\sum^L_{l=1}e^{z_{i,l}})^2} = \frac{- e^{z_{i,j}}\cdot e^{z_{i,k}}}{(\sum^L_{l=1}e^{z_{i,l}})^2} = -\frac{e^{z_{i,j}}}{\sum^L_{l=1}e^{z_{i,l}}} \cdot \frac{e^{z_{i,k}}}{\sum^L_{l=1}e^{z_{i,l}}} $$
Once again, we end up with the Softmax function itself...

$$ = -S_{i,j} \cdot S_{i,k} $$

So now:

$$ \frac{\partial S_{i,j}}{\partial z_{i,k}} = 
	\Biggl\{
	\begin{matrix}
		S_{i,j}\cdot(1-S_{i,k}) , & j=k \\ \\
		-S_{i,j} \cdot S_{i,k} , & j \ne k 
	\end{matrix}
\quad \left( 1 \right) $$
We can amend this further: 
$$ \frac{\partial S_{i,j}}{\partial z_{i,k}} = 
	\Biggl\{
	\begin{matrix}
		S_{i,j}\cdot(1-S_{i,k}) , & j=k \\ \\
		S_{i,j} \cdot (0-S_{i,k}) , & j \ne k 
	\end{matrix}
\quad \left( 2 \right) $$
Now that they are effectively the same, differing by only one value, we can pply the *Kronecker delta* function:
$$ \delta_{i,j} =  
\Biggl\{
\begin{matrix}
	1, & i=j \\ \\
	0 , & i \ne j 
\end{matrix}
$$
We can apply it here to further simplify:
$$ \frac{\partial S_{i,j}}{\partial z_{i,k}} = S_{i,j} \cdot (\delta_{j,k} - S_{i,k}) $$
Now, one last transformation to make it more easily applicable in python:

$$
\frac{\partial S_{i,j}}{\partial z_{i,k}} = S_{i,j} \cdot (\delta_{j,k} - S_{i,k}) = S_{i,j}\delta_{j,k} - S_{i,j}S_{i,k}
$$

### Python
Now, implementing it in python:
```Python
class Softmax:
	def backwards(self, dvalues):
		self.dinputs = np.empty_like(dvalues)
	for index, (single_output, single_dvalues) in enumerate(zip(self.output, dvalues)):
		single_output = single_output.reshape(-1,1)
		jacobian_matrix = np.diagflat(single_output) - np.dot(single_output, single_output.T)
		self.dinputs[index] = np.dot(jacobian_matrix, single_dvalues)
```

Here, `dvalues` is our gradient we are backpropogating from our loss function. We first make an empty array of the same dimensions.

We then zip the original softmax outputs `self.outputs` with the gradient `dvalues` creating pairs (tuples) and iterate over them. As we iterate, we are calculating the partial derivatives and calculating the final product of the jacobian matrix and gradient vector, storing the results as a row in `dinput`.

The *Jacobian matrix* is an array of partial derivates in all of the combinations of both input vectors. We need to calculate the partial derivatives of every output with respect to each input separately, because each input influences each output due to the normalization process. The result of this operation performed on a batch of samples is a list of jacobian matrices, which is effectively a 3D matrix.

Now, this is a problem because when we back propogate to the preceding layer, it will expect a 2D gradient. So, we need to sum the values of these vectors so that each of the inputs for each of the samples returns a single partial derivative value. Since each input influences all outputs, the returned vector of the partial derivates has to be summed up for the final partial derivative with respect to each input.

To solve this, we use `np.dot()`. For each sample, it'll take the row from the Jacobian matrix and multiply it by the corresponding value from the loss function's gradient.  The dot product of each of these vectors and values will return a single value, forming a vector of the partial derivates sample-wise and a 2D array batch-wise.