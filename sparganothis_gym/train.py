from tetris.env import *
from tetris.model import *
from tetris.hparams import *
from tetris.reward import *
from tetris.memory import *
from tqdm import tqdm
from itertools import count
import math
import torch.optim as optim

policy_net = DQN(128).to(device)
target_net = DQN(128).to(device)
target_net.load_state_dict(policy_net.state_dict())

optimizer = optim.AdamW(policy_net.parameters(), lr=LR, amsgrad=True)
memory = init_memory(100_000)

# env = TetrisEnv(merge_rewards(
#     [build_end_reward(-100),
#     build_score_reward(0.01)]
# ))

env = TetrisEnv(build_per_state_reward())

steps_done = 0

if torch.cuda.is_available() or torch.backends.mps.is_available():
    num_episodes = 600
else:
    num_episodes = 50

for i_episode in tqdm(range(num_episodes)):
    # Initialize the environment and get its state
    state, info = env.reset()
    state = o2t(state)

    for t in count():
        eps_threshold = EPS_END + (EPS_START - EPS_END) * math.exp(
            -1.0 * steps_done / EPS_DECAY
        )
        steps_done += 1
        action = select_action(env, policy_net, state, info, eps_threshold)
        observation, reward, terminated, truncated, info = env.step(action.item())
        reward = torch.tensor([reward], device=device)
        done = terminated or truncated

        if terminated:
            next_state = None
        else:
            next_state = o2t(observation)

        # Store the transition in memory
        memory.push(state, action, next_state, reward)

        # Move to the next state
        state = next_state

        # Perform one step of the optimization (on the policy network)
        optimize_model(policy_net, target_net, optimizer, memory)

        # Soft update of the target network's weights
        # θ′ ← τ θ + (1 −τ )θ′
        target_net_state_dict = target_net.state_dict()
        policy_net_state_dict = policy_net.state_dict()
        for key in policy_net_state_dict:
            target_net_state_dict[key] = policy_net_state_dict[
                key
            ] * TAU + target_net_state_dict[key] * (1 - TAU)
        target_net.load_state_dict(target_net_state_dict)

        if done:
            break
policy_net_scripted = torch.jit.script(policy_net)
policy_net_scripted.save("policy_net.pt")