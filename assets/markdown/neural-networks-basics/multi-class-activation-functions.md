# Multiclass Activation Functions
* Many classification functions are multi-class (eg, not "cat" or "not cat")

*  2 types of multi-class situations
	* Non-Exclusive Classes
		* Can have multiple classes/categories assigned to it
	* Mutually Exclusive Classes
		* Only one class per data point
* Organizing data that has multiple classes
	* One output node per class (easiest way)
	* Not all classes easily translate over to classes on a computer...
	* **One-hot encoding**
* **One-hot encoding**
	* Treating everything as on/off, or 1/0
* Activation Function
	* Non-exclusive
		* Use sigmoid function

	* Exclusive

		* Soft-max Function

			* for i = 1,..., K

				* K is number of categories

			* Calculates the probablility distribution of the event over K different events

			* The range will be 0 to 1, and the sum of all probabilities will be equal to one.

			* Target class obviously has the highest probability...

				* Example output...

					* [Red, Green, Blue]

					* [0.1, 0.6, 0.3 ] *notice they all add up to 1 across all outputs

$$ \sigma(z)_i = \frac{e^{z_i}}{\sum_{j=1}^{K}e^{z_j}}$$

  