from collections import deque
import random
from tetris.env import *
from tetris.reward import *
from tetris.model import *
import tqdm
import time

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

def init_memory(reward, episodes, episode_size, memory_size, threads):
    memory = ReplayMemory(memory_size)
    from concurrent.futures import ThreadPoolExecutor
    
    def add():
        env = TetrisEnv()
        moves = env.vim_state.generate_bot_episode("wordpress", episode_size)
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
                torch.tensor([[a2i(la)]], device=device, dtype=torch.long), 
                s2t(v2s(s)[0]), 
                torch.tensor([reward(ls, s, history)], device=device, dtype=torch.long), 
            )
            ls = s
            la = a
    executor = ThreadPoolExecutor(max_workers=threads)
    results = []
    for _ in range(episodes):
        results.append(executor.submit(add))
    for r in tqdm.tqdm(results):
        r.result()
    return memory