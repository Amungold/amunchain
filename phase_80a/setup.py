from setuptools import setup, find_packages

setup(
    name="phase80a",
    version="2.0.0",
    packages=find_packages(),
    install_requires=[],
    entry_points={
        "console_scripts": [
            "phase80a=phase_80a.__main__:main",
        ],
    },
    python_requires=">=3.8",
)
