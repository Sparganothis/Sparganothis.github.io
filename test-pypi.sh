#!/bin/bash
set -ex
cd sparganothis_vim

rm -rf test-venv || true
python -m venv test-venv
. test-venv/Scripts/activate
pip install --no-cache-dir sparganothis_vim
python test.py