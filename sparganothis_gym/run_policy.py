from tetris.env import *
from tetris.model import *
from tetris.core import * 
import torch


device = torch.device(
    "cuda"
    if torch.cuda.is_available()
    else "mps" if torch.backends.mps.is_available() else "cpu"
)

policy_net = torch.jit.load('policy_net.pt', map_location=device)
policy_net.eval()


env = TetrisEnv()

s, i = v2s(env.vim_state)
x = s2t(s)
action = (
    (
        policy_net(
            x["board"][None, ::],
            x["next"][None, ::],
            x["hold"][None, ::],
            x["hcf"][None, ::],
        ).squeeze()
        * torch.tensor(i["action_mask"], dtype=torch.float32)
    )
    .argmax()
    .item()
)
action = i2a(action)
print(action)