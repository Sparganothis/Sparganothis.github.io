from tetris.env import *
from tetris.model import o2t
import os
import time
import torch

policy_net = torch.jit.load('policy_net.pt')
policy_net.eval()

env = TetrisEnv()

obs, info = env.reset()

# if GPU is to be used
device = torch.device(
    "cuda"
    if torch.cuda.is_available()
    else "mps" if torch.backends.mps.is_available() else "cpu"
)

# Take some policy_net actions
while True:
    print(env.render())
    time.sleep(0.25)
    os.system('cls')
    x = o2t(obs)
    action = (
        (
            policy_net(
                x["board"][None, ::],
                x["next"][None, ::],
                x["hold"][None, ::],
            ).squeeze()
            * torch.tensor(info["action_mask"], device=device, dtype=torch.float32)
        )
        .argmax()
        .item()
    )
    obs, reward, terminated, _, info = env.step(action)

    if terminated:
        break
print(env.render())