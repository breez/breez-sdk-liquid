#!/usr/bin/env python

from setuptools import setup

LONG_DESCRIPTION = """# Breez Liquid SDK - NWC Plugin
Python language bindings for the Breez Liquid NWC Plugin.

## Installing

```shell
pip install breez_sdk_liquid_nwc
```
"""

setup(
    name="breez_sdk_liquid_nwc",
    version="0.0.1",
    description="Python language bindings for the Breez Liquid SDK NWC Plugin",
    long_description=LONG_DESCRIPTION,
    long_description_content_type="text/markdown",
    packages=["breez_sdk_liquid_nwc"],
    package_dir={"breez_sdk_liquid_nwc": "./src/breez_sdk_liquid_nwc"},
    include_package_data=True,
    package_data={"breez_sdk_liquid_nwc": ["*.dylib", "*.so", "*.dll"]},
    url="https://github.com/breez/breez-sdk-liquid",
    author="Breez <contact@breez.technology>",
    license="MIT",
    has_ext_modules=lambda: True,
)
