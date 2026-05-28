#!/bin/bash
cd "$(dirname "$0")/.."
python -m phase_80a.tests.run_all_tests
