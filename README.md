# Cedat Detect Python Bindings

> This project is a work in progress and not fully developed

This repository builds python bindings for the [cedar-detect](https://github.com/smroid/cedar-detect) rust library.

## Example

```shell
pip install cedar-detect-python
```

```python
import numpy as np
from PIL import Image

import cedar_detect

if __name__ == '__main__':
    img = 'hale_bopp.jpg'
    with Image.open(img) as img:
        img = img.convert(mode='L')
    
    summary = cedar_detect.algorithm.summarize_region_of_interest(
        img.width,
        img.height,
        np.asarray(img, dtype=np.uint8).flatten(),
        10, 10, 256, 256, 0.01, 0.01,
    )
```

## Local Development

Requirements:

- Rust `1.79+`
- Python `3.7+`
- [Hatch](https://hatch.pypa.io/latest/install/)
  - Install `hatch` following the procedure for your operating system.

Setup the project locally for development. 
This will create a `.venv/` directory in the repository root.
```shell
hatch run setup
```

Run tests

```shell
hatch test
```

Run a release build to create wheels in the `dist/` directory

```shell
hatch build -t wheel
```