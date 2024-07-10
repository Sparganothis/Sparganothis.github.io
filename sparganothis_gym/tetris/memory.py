from collections import deque
import random
from tetris.env import *
from tetris.reward import *
from tetris.model import *
import tqdm
class ReplayMemory(object):

    def __init__(self, capacity):
        self.memory = deque([], maxlen=capacity)

    def push(self, *args):
        """Save a transition"""
        self.memory.append(Transition(*args))

    def sample(self, batch_size):
        return random.sample(self.memory, batch_size)

    def __len__(self):
        return len(self.memory)

def add_episode(reward, memory, moves):
    ls = None
    la = None
    history = []
    for a, s in moves:
        history.append(a2i(a))
        if not ls:
            ls = s
            la = a
            continue
        memory.push(
            s2t(v2s(ls)[0]), 
            torch.tensor([[a2i(la)]], dtype=torch.long), 
            s2t(v2s(s)[0]), 
            torch.tensor([reward(ls, s)], dtype=torch.long), 
        )
        ls = s
        la = a

def init_memory(reward, episodes, generator, memory_size):
    memory = ReplayMemory(memory_size)
    for _ in tqdm.tqdm(range(episodes)):
        add_episode(reward, memory, generator())
    return memory