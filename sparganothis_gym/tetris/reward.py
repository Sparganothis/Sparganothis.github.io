from tetris.core import * 
from tetris.hparams import *

def build_score_reward(alpha):
    def score_reward(current_state, next_state, history):
        return (next_state.score - current_state.score) * alpha
    return score_reward

def build_end_reward(value):
    def end_reward(current_state, next_state, history):
        if next_state.game_over:
            return value
        return 0
    return end_reward

def build_history_reward(targets):
    def history_reward(current_state, next_state, history):
        history = list(reversed(history))
        def try_def(a):
            try:
                return history.index(a2i(a))
            except ValueError:
                return len(history)
        r = 0
        for v, al in targets:
            r += min(
                [
                    try_def(a) for a in al
                ]
            ) * v
        return r
    return history_reward

def merge_rewards(reward_fns):
    def reward_fn(current_state, next_state, history):
        reward = 0
        for fn in reward_fns:
            reward += fn(current_state, next_state, history)
        return reward
    return reward_fn

default_reward = merge_rewards(
    [build_end_reward(REWARD_END),
    build_score_reward(REWARD_SCORE),
    build_history_reward([
        (REWARD_SOFT, [SOFT_DROP]),
        (REWARD_MOVE, [MOVE_LEFT, MOVE_RIGHT]),
        (REWARD_ROTATE, [ROTATE_LEFT, ROTATE_RIGHT]),
    ])]
)