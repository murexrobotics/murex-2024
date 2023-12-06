from logger import logger

import numpy as np

def interpolate(current, target, duration, sps=10):
    """
    Uses smoothstep to interpolate between two numbers over a given duration.

    :param current: The current value.
    :param target: The target value.
    :param duration: The duration in seconds over which to interpolate.
    :param sps: The number of steps per second.
    :return: List of interpolated values.
    :rtype: list
    """
  
    steps = np.linspace(current, target, duration * sps)
    print(steps)
    interpolated_values = [smoothstep(current, target, step) for step in steps]
    return interpolated_values


# https://www.youtube.com/watch?v=60VoL-F-jIQ
def smoothstep(edge1, edge2, x):
    """
    Calculates the point x on a smooth Hermite interpolation between edge1 and edge2.
    :param edge1: The left edge of the Hermite function.
    :param edge2: The right edge of the Hermite function.
    :param x: The point between edge1 and edge2.
    :return: The interpolated value.
    :rtype: float
    """
    x = clamp((x - edge1) / (edge2 - edge1), 0.0, 1.0)
    return x * x * (3 - 2 * x)

def clamp(x, lower, upper):
    """
    Clamps a value between a lower and upper bound.
    :param x: The value to clamp.
    :param lower: The lower bound.
    :param upper: The upper bound.
    :return: The clamped value.
    :rtype: float
    """
    return min(max(x, lower), upper)

# Tests
if __name__ == "__main__":
    import matplotlib.pyplot as plt
    # import numpy as np

    # Test smoothstep
    x = np.linspace(0, 1, 100)
    y = [smoothstep(0, 1, i) for i in x]
    plt.plot(x, y)
    plt.show()

    # Test interpolate
    x = interpolate(0, 1, 1)
    logger.debug(x)
    plt.plot(x)
    plt.show()