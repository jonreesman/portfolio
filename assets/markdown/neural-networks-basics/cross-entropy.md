# Cross Entropy Loss Function
* For classification problems, we often use the **cross entropy** loss function

	* Assumes model predicts a probability distribution p(y=i) for eacg class i=1,2,...,C

	* For binary classification this results in: (natural logs)

	* $$ -(ylog(p) + (1-y)log(1-p)) $$

	* For M number of classes > 2

	* $$ -\sum_{c=1}^{M}y_{o,c}log(p_{o,c}) $$