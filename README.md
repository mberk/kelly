# kelly

The `kelly` package implements (very) fast Kelly staking calculations for a range of scenarios
that currently include:

* Backing and laying
* Any number of outcomes

Scenarios to be covered in the near future include:

* Placing more than one bet simultaneously
* Betting on events that have more than one winner

## Installation

Requires Python 3.7 or above

```
pip install git+https://github.com/mberk/kelly.git
```

## Usage

```
>>> kelly.calculate_kelly_stake?
Signature:
kelly.calculate_kelly_stake(
    price,
    is_back,
    probability,
    other_probabilities,
    position,
    other_positions,
    bankroll,
    kelly_fraction=1.0,
    verbose=False,
)
Docstring:
Calculate the optimal fractional Kelly stake when placing a single bet on an event that has one winner

:param price: The price at which the bet is being placed
:param is_back: Whether you are backing (True) or laying (False)
:param probability: Your fair probability for the outcome you are betting on
:param other_probabilities: A list of probabilities for the other possible outcomes. sum(other_probabilities) + probability should be equal to 1 subject to floating-point error but this is not yet enforced
:param position: Your position - i.e. how much you are currently standing to win or lose - on the outcome you are betting on
:param other_positions: A list of positions corresponding to the other possible outcomes. It is assumed there is a 1:1 correspondence between the elements of this list and other_probabilities
:param bankroll: Your notional Kelly bankroll
:param kelly_fraction: A fraction to multiply the optimal stake by. Defaults to 1
:param verbose: Whether to generate log statements when numerically optimising the stake. Defaults to False

:return: The optimal fractional Kelly stake
Type:      builtin_function_or_method
```

## Benchmarks

Coming soon...
