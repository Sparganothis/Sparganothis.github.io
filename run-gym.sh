#!/bin/bash
set -ex
cd sparganothis_vim
. venv/Scripts/activate

maturin develop

python test.py