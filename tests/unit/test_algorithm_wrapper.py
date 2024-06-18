from pathlib import Path

import numpy as np
import pytest
from PIL import Image

import cedar_detect


def test_estimate_noise_from_image(data_path: Path):
    img = data_path / 'hale_bopp.jpg'
    with Image.open(img) as img:
        img = img.convert(mode='L')

    result = cedar_detect.algorithm.estimate_noise_from_image(
        img.width,
        img.height,
        np.asarray(img, dtype=np.uint8).flatten(),
    )

    # not sure if this is true for this image....
    assert result == pytest.approx(5.516929626464844)


def test_summarize_region_of_interest(data_path: Path):
    img = data_path / 'hale_bopp.jpg'
    with Image.open(img) as img:
        img = img.convert(mode='L')

    result = cedar_detect.algorithm.summarize_region_of_interest(
        img.width,
        img.height,
        np.asarray(img, dtype=np.uint8).flatten(),
        10, 10, 256, 256, 0.01, 0.01,
    )

    print(result)
    print(result.histogram)
    assert len(result.histogram) == 256
    print(result.peak_x)
    print(result.peak_y)
