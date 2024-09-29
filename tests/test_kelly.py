import kelly


def test_calculate_kelly_stake():
    stake = kelly.calculate_kelly_stake(
        price=100,
        is_back=False,
        probability=1 / 100,
        other_probabilities=[99 / 100],
        position=0.0,
        other_positions=[0.0],
        bankroll=100.0,
        kelly_fraction=1.0,
        verbose=True,
    )
    assert stake > 0
    assert round(stake, 2) == 0.0
