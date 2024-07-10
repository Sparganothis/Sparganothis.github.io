from tetris.env import *
from tetris.model import s2t
import os
import time
import torch


env = TetrisEnv()

obs, info = env.reset()

# if GPU is to be used
device = torch.device(
    "cuda"
    if torch.cuda.is_available()
    else "mps" if torch.backends.mps.is_available() else "cpu"
)

policy_net = torch.jit.load('policy_net.pt', map_location=device)
policy_net.eval()

# Take some policy_net actions
while True:
    print(env.render())
    time.sleep(0.1)
    os.system('cls')
    x = s2t(obs)
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
    print(reward)

    if terminated:
        break
print(env.render())