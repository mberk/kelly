def calculate_kelly_stake(
    price: float,
    is_back: bool,
    probability: float,
    other_probabilities: list[float],
    position: float,
    other_positions: list[float],
    bankroll: float,
    kelly_fraction: float,
    verbose: bool,
) -> float: ...
