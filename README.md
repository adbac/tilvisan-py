# tilvisan-py

Python bindings for the Rust [`tilvisan`](https://github.com/simoncozens/tilvisan) crate.

This package is distributed under the MIT license.

<!-- ## Documentation

The documentation is [here](https://adbac.github.io/xmpkit-py). Jump straight to the API reference [here](https://adbac.github.io/xmpkit-py/api-reference) -->

# Installation

- Ensure you have **Python 3.10+** installed on your machine.
- Install `tilvisan-py` through `pip` directly or using a package manager like `uv` or `poetry`.

# Use

You're all set! You can now use the package by importing it in a Python script:

``` py
from tilvisan import autohint

autohint(
    "UntitledFont_unhinted.ttf",  # input
    "UntitledFont_hinted.ttf",  # output
    no_info=True,
)
```

See the [stub file](https://github.com/adbac/tilvisan-py/blob/main/python/tilvisan/__init__.pyi) for a documentation of the function's parameters.

## Contributing

Contributions are welcome! Contributing guidelines are to be written.
