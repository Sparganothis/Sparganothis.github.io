
import gymnasium as gym
import numpy as np
from gymnasium import spaces
import sparganothis_vim
from tetris.reward import *

ALL_ACTIONS = [
    "HardDrop",
    "SoftDrop",
    "MoveLeft",
    "MoveRight",
    "Hold",
    "RotateLeft",
    "RotateRight",
]
ALL_PIECES = ["I", "J", "L", "O", "S", "T", "Z"]
BOARD_SHAPE = [20, 10]

def a2i(a):
    return ALL_ACTIONS.index(a)

def i2a(i):
    return ALL_ACTIONS[i]

def al2m(al):
    r = np.zeros(len(ALL_ACTIONS))
    if not al:
        return r
    for a in al:
        r[a2i(a)] = 1
    return r

def p2i(p):
    if not p:
        return -1
    return ALL_PIECES.index(p)

def i2p(i):
    return ALL_PIECES[i]

class TetrisEnv(gym.Env):
    metadata = {"render_modes": ["human"], "render_fps": 4}

    def __init__(self, reward_fn=default_reward, render_mode=None):
        self.reward_fn = reward_fn
        self.render_mode = render_mode

        self.action_space = spaces.Discrete(len(ALL_ACTIONS))

        self.observation_space = spaces.Dict(
            {
                "board": spaces.Box(low=0, high=1, shape=BOARD_SHAPE, dtype=np.int32),
                "next": spaces.Box(
                    low=0, high=len(ALL_PIECES), shape=(5,), dtype=np.int32
                ),
                "hold": spaces.Discrete(len(ALL_PIECES) + 1),
            }
        )

        self.vim_seed = sparganothis_vim.generate_random_seed()
        self.vim_state = sparganothis_vim.GameStatePy(self.vim_seed)

    def obs_vim_state(self):
        return {
            "board": np.array(self.vim_state.main_board).astype(int),
            "next": [p2i(p) for p in self.vim_state.next_pcs[:5]],
            "hold": p2i(self.vim_state.hold) + 1,
        }, {
            "action_mask": np.array(
                al2m([a for a, _ in self.vim_state.next_actions_and_states]),
                dtype=np.int8,
            ),
            "game_over": self.vim_state.game_over,
        }

    def reward_vim_state(self, prev_state):
        terminated = self.vim_state.game_over

        reward = self.reward_fn(prev_state, self.vim_state)

        return reward, terminated

    def reset(self, seed=None, options=None):
        super().reset(seed=seed)

        if seed is None:
            self.vim_seed = sparganothis_vim.generate_random_seed()
        else:
            self.vim_seed = sparganothis_vim.generate_seed(seed)

        self.vim_state = sparganothis_vim.GameStatePy(self.vim_seed)

        obs, info = self.obs_vim_state()

        if self.render_mode == "human":
            self.render()

        return obs, info

    def step(self, action):
        # Perform action
        last_vim_state = self.vim_state
        self.vim_state = dict(self.vim_state.next_actions_and_states)[i2a(action)]

        # Determine reward and termination
        reward, terminated = self.reward_vim_state(last_vim_state)

        # Construct the observation state:
        obs, info = self.obs_vim_state()

        # Render environment
        if self.render_mode == "human":
            self.render()

        # Return observation, reward, terminated, truncated (not used), info
        return obs, reward, terminated, False, info

    # Gym required function to render environment
    def render(self):
        return self.vim_state.matrix_txt