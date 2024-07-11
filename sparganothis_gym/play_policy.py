from tetris.env import *
from tetris.model import *
import os
import time
import torch


env = TetrisEnv()

obs, info = env.reset()

# if GPU is to be used
device = torch.device("cpu")

policy_net = DQN(TRAIN_MODEL_SIZE).to(device)
# with open("policy_net_states.pt", "rb") as f:
    # policy_net.load_state_dict(torch.load(f))
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
            * torch.tensor(info["action_mask"], dtype=torch.float32)
        )
        .argmax()
        .item()
    )
    obs, reward, terminated, _, info = env.step(action)
    print(reward)

    if terminated:
        break
print(env.render())