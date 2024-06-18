from typing import List

import numpy as np


class StarDescription:
    centroid_x: float
    centroid_y: float
    peak_value: int
    brightness: float
    num_saturated: int


class RegionOfInterestSummary:
    histogram: List[int]
    peak_x: float
    peak_y: float


def estimate_noise_from_image(width: int, height: int, data: np.ndarray) -> float: ...


def summarize_region_of_interest(
        width: int, height: int, data: np.ndarray,  # img
        x: int, y: int, w: int, h: int,  # roi Rect
        noise_estimage: float,
        sigma: float
) -> RegionOfInterestSummary: ...
