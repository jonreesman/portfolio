# Categorical Cross Entropy Loss Function and Softmax Activation Function Derivation
Surprisingly, the derivatives of the CCLE and Softmax can be condensed out and simplified further when used together.

Applying the chain rule to calculate the partial derivative of the Categorical Cross-Entropy Loss function with respect to the Softmax function inputs.

$$
\frac{\partial L_i}{\partial z_{i,k}} =
\frac{\partial L_i}{\partial \hat{y}_{i,j}} \cdot
\frac{\partial S_{i,j}}{\partial z_{i,k}}  \quad (1)
$$
This is the partial derivative of the loss function with respect to its inputs, multiplied by the partial derivative of the activation function with respect to its inputs.

Now, we know the inputs of the loss function, y-hat_i,j, are the outputs of the softmax activation function, S_i,j
$$
\hat{y}_{i,j} = S_{i,j}
$$
So we can update the equation to:
$$
= \frac{\partial L_i}{\partial \hat{y}_{i,j}} \cdot
\frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}} =
$$
Now, we can substitute the (a form of the) equation for the partial derivative of the CCE function. Specifically, we will use the form that still has a summation:
$$
\frac{\partial L_i}{\partial \hat{y}_{i,j}} = 
- \sum_j \frac{y_{i,j}}{\hat{y}_{i,j}}
$$
Now substitute this back into the combined derivative's equation:
$$
= - \sum_j \frac{y_{i,j}}{\hat{y}_{i,j}} \cdot
\frac{ \partial \hat{y}_{i,j}}{\partial z_{i,k}} =
$$
Now, we will steal from out Softmax derivative calculation:
$$
\frac{\partial S_{i,j}}{\partial z_{i,k}} = 
	\Biggl\{
	\begin{matrix}
		S_{i,j}\cdot(1-S_{i,k}) , & \text{j=k} \\ \\
		-S_{i,j} \cdot S_{i,k} , & j \ne k 
	\end{matrix}
\quad (2)
$$
We can substitute S_i,j with y-hat_i,j:
$$ \frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}} = 
	\Biggl\{
	\begin{matrix}
		\hat{y}_{i,j}\cdot(1-\hat{y}_{i,k}) , & \text{j=k} \\ \\
		-\hat{y}_{i,j} \cdot \hat{y}_{i,k} , & j \ne k 
	\end{matrix}
\quad (3) $$
Now, splitting again just like we did with the Softmax:
$$
-\sum_j \frac{{y}_{i,j}}{\hat{y}_{i,j}} \cdot
\frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}} = 
- \frac{y_{i,j}}{\hat{y}_{i,j}} \cdot
\frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}} - 
\sum_{j \ne k}\frac{y_{i,j}}{\hat{y}_{i,j}} \cdot
\frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}}
$$
For the j!=k case, we just update the summation to exclude k.
For the j=k case, we do not seem the sum operator as it will only sum one element, index k. For the same reason, we also replace j indices with k:
$$
-\frac{y_{i,k}}{\hat{y}_{i,k}} \cdot
\frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}}
$$
Back to main equation:
$$
-\frac{y_{i,k}}{\hat{y}_{i,k}} \cdot
\frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}} - 
\sum_{j \ne k} \frac{y_{i,j}}{\hat{y}_{i,j}} \cdot
\frac{\partial \hat{y}_{i,j}}{\partial z_{i,k}}
$$
We an then substitute the partial derivatives of the activation function for both cases with the newly defined solutions:
$$
= - \frac{y_{i,k}}{\hat{y}_{i,k}} \cdot \hat{y}_{i,k} \cdot
(1 - \hat{y}_{i,k}) -
\sum_{j \ne k} \frac{y_{i,j}}{\hat{y}_{i,j}}(-\hat{y}_{i,j}\hat{y}_{i,k}) =
$$
We can cancel out the y-hat_i,k on both sides and solve the double negative on the RHS
$$
= -y_{i,k} \cdot (1 - \hat{y}_{i,k}) +
\sum_{j \ne k}y_{i,j}\hat{y}_{i,k} = 
-y_{i,k} + y_{i,k}\hat{y}_{i,k} +
\sum_{j \ne k}y_{i,j}\hat{y}_{i,k}
$$
Interestingly, if we look at out equation, the value of y that is excluded from the summation (j=k) is present outside the summation in the form of y_i,k. So we can simplify the summation some:
$$
= -y_{i,k} + \sum_j y_{i,j}\hat{y}_{i,k} =
$$
Even more interestingly, each value of y_i,j, each index of i is the one-hot encoded vector of ground-truth values, the sum of all of its elements equals 1. In other words, the sum will multiply 0 by the y-hat_i,k except for a single situation, the truth label, where it'll multiply 1 by this value. Further simplifying this down:
$$
-y_{i,k} + \hat{y}_{i,k} = \hat{y}_{i,k} - y_{i,k}
$$
Hence, the entire chain rule of both partial derivatives simplifies down to the subtraction of the predicted and ground truth values. Way faster to compute.