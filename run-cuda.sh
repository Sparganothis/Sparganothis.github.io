cd sparganothis_gym
pipenv run bash -c "cd ../sparganothis_vim; pip install maturin; maturin develop"
pipenv run python original_alex.py
