from tetris.core import * 
from tetris.hparams import *

def build_score_reward(alpha):
    def score_reward(current_state, next_state):
        return (next_state.score - current_state.score) * alpha
    return score_reward

def build_end_reward(value):
    def end_reward(current_state, next_state):
        if next_state.game_over:
            return value
        return 0
    return end_reward

def build_bumpi_reward(alpha):
    def end_bumpi_reward(current_state, next_state):
        return alpha * (next_state.bumpi - current_state.bumpi)
    return end_bumpi_reward

def build_holes_reward(alpha):
    def holes_reward(current_state, next_state):
        return alpha * (next_state.holes - current_state.holes)
    return holes_reward

def build_height_reward(alpha):
    def height_reward(current_state, next_state):
        return alpha * (next_state.height - current_state.height)
    return height_reward

def build_lines_reward(alpha):
    def lines_reward(current_state, next_state):
        return alpha * (next_state.total_lines - current_state.total_lines)
    return lines_reward

def merge_rewards(reward_fns):
    def reward_fn(current_state, next_state):
        reward = 0
        for fn in reward_fns:
            reward += fn(current_state, next_state)
        return reward
    return reward_fn

default_reward = merge_rewards(
    [build_end_reward(REWARD_END),
    build_score_reward(REWARD_SCORE),
    build_bumpi_reward(REWARD_BUMPI),
    build_holes_reward(REWARD_HOLES),
    build_lines_reward(REWARD_LINES),
    build_height_reward(REWARD_HEIGHT),]
)