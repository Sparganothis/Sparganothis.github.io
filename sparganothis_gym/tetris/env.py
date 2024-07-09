
import gymnasium as gym
import numpy as np
from gymnasium import spaces
import sparganothis_vim
from tetris.reward import *

def al2m(al):
    r = np.zeros(len(ALL_ACTIONS))
    if not al:
        return r
    for a in al:
        r[a2i(a)] = 1
    return r

def v2s(v):
    return {
        "board": np.array(v.main_board).astype(int),
        "next": [p2i(p) for p in v.next_pcs[:5]],
        "hold": p2i(v.hold) + 1,
    }, {
        "action_mask": np.array(
            al2m([a for a, _ in v.next_actions_and_states]),
            dtype=np.int8,
        ),
        "game_over": v.game_over,
    }

class TetrisEnv(gym.Env):
    metadata = {"render_modes": ["human"], "render_fps": 4}

    def __init__(self, reward_fn=default_reward, soft_drop_int=4, render_mode=None):
        self.reward_fn = reward_fn
        self.soft_drop_int = soft_drop_int
        self.render_mode = render_mode
        self.move_history = []

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

    def reset(self, seed=None, options=None):
        super().reset(seed=seed)

        self.move_history = []
        if seed is None:
            self.vim_seed = sparganothis_vim.generate_random_seed()
        else:
            self.vim_seed = sparganothis_vim.generate_seed(seed)

        self.vim_state = sparganothis_vim.GameStatePy(self.vim_seed)

        obs, info = v2s(self.vim_state)

        if self.render_mode == "human":
            self.render()

        return obs, info

    def step(self, action):
        if len(self.move_history) % self.soft_drop_int == 0:
            last_vim_state = self.vim_state
            self.vim_state = dict(self.vim_state.next_actions_and_states)[SOFT_DROP]
            terminated = self.vim_state.game_over
            if terminated:
                reward = self.reward_fn(last_vim_state, self.vim_state, self.move_history) 
                obs, info = v2s(self.vim_state)
                return obs, reward, terminated, False, info
        self.move_history.append(action)
        # Perform action
        last_vim_state = self.vim_state
        self.vim_state = dict(self.vim_state.next_actions_and_states)[i2a(action)]

        # Determine reward and termination
        reward = self.reward_fn(last_vim_state, self.vim_state, self.move_history) 
        terminated = self.vim_state.game_over

        # Construct the observation state:
        obs, info = v2s(self.vim_state)

        # Render environment
        if self.render_mode == "human":
            self.render()

        # Return observation, reward, terminated, truncated (not used), info
        return obs, reward, terminated, False, info

    # Gym required function to render environment
    def render(self):
        return self.vim_state.matrix_txt