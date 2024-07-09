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

def merge_rewards(reward_fns):
    def reward_fn(current_state, next_state):
        reward = 0
        for fn in reward_fns:
            reward += fn(current_state, next_state)
        return reward
    return reward_fn

default_reward = build_end_reward(-100)