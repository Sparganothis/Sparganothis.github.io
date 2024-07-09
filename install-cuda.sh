cd sparganothis_gym
pipenv install
pipenv run pip3 install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu118
pipenv run pip install -r requirements2.txt
echo "INSTALL CUDA 11.8 from HERE: https://developer.nvidia.com/cuda-11-8-0-download-archive?target_os=Windows&target_arch=x86_64&target_version=11&target_type=exe_local"

