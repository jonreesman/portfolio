# Stochastic Gradient Descent
## Learning Rate Decay
The idea of learning rate decay is to start with a large learning rate, then decrease it as we train. The reason for this, is it allows us to utilize a large learning rate early to more quickly get in the right ballpark, and then adjust down to a much lower learning rate. The justification for this, is too low a learning rate early could cause us to find ourselves trapped in a local minima.

**Decay Rate**
- Also known as **1/t decaying** or **exponential decaying**.
- Steadily reduces the learning rate per batch or epoch.
- We essentially update the learning rate each step by the reciprocal of the step count fraction. This fraction is a new hyper-parameter that we'll add to the optimizer called the **learning rate decay**
	- This parameter works by multiplying the step and decaying ratio. We then take the reciprocal and multiply it by the initial learning rate