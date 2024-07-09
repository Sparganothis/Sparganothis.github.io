from tetris.env import *
from tetris.model import *
from tetris.core import * 
import torch

policy_net = torch.jit.load('policy_net.pt')
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
        ).squeeze()
        * torch.tensor(i["action_mask"], dtype=torch.float32)
    )
    .argmax()
    .item()
)
action = i2a(action)
print(action)