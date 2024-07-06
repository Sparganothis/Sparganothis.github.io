#!/bin/bash
set -ex
cd sparganothis_vim
. venv/Scripts/activate

set +x
export MATURIN_PYPI_TOKEN="$(cat /c/Users/john/Desktop/PASSWORDS/pypi-api-token.txt)"

set -x
maturin publish
