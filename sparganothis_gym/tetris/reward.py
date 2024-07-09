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

def build_history_reward(value):
    def history_reward(current_state, next_state, history):
        return 
    return history_reward

def build_per_state_reward():
    def per_state_reward(current_state, next_state, history):
        return next_state.score + 500 * next_state.total_garbage_sent + 250 * next_state.total_lines - 5 * next_state.total_move_count
    return per_state_reward

def merge_rewards(reward_fns):
    def reward_fn(current_state, next_state, history):
        reward = 0
        for fn in reward_fns:
            reward += fn(current_state, next_state, history)
        return reward
    return reward_fn