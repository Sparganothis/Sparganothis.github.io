from tetris.env import *
from tetris.model import *
from tetris.hparams import *
from tetris.reward import *
from tetris.memory import *
from tqdm import tqdm
from itertools import count
import math
import torch.optim as optim

import wandb
run = wandb.init(
    project="sparganothis_gym_tet",
    config={
        "REWARD_END": REWARD_END,
        "REWARD_SOFT": REWARD_SOFT,
        "REWARD_MOVE": REWARD_MOVE,
        "REWARD_ROTATE": REWARD_ROTATE,
        "REWARD_SCORE": REWARD_SCORE,
        "TRAIN_MEMORY_EPISODES": TRAIN_MEMORY_EPISODES,
        "TRAIN_MEMORY_EPISODE_SIZE": TRAIN_MEMORY_EPISODE_SIZE,
        "TRAIN_MEMORY_SIZE": TRAIN_MEMORY_SIZE,
        "TRAIN_MEMORY_THREADS": TRAIN_MEMORY_THREADS,
        "TRAIN_MEMORY_THREADS": TRAIN_MEMORY_THREADS,
        "TRAIN_MODEL_INIT_STEPS": TRAIN_MODEL_INIT_STEPS,
        "TRAIN_EPISODES_CPU": TRAIN_EPISODES_CPU,
        "TRAIN_EPISODES_GPU": TRAIN_EPISODES_GPU,
        "TRAIN_EPISODE_SIZE": TRAIN_EPISODE_SIZE,
        "BATCH_SIZE": BATCH_SIZE,
        "GAMMA": GAMMA,
        "EPS_START": EPS_START,
        "EPS_END": EPS_END,
        "EPS_DECAY": EPS_DECAY,
        "TAU": TAU,
        "LR": LR,
    }
)


policy_net = DQN(TRAIN_MODEL_SIZE).to(device)
target_net = DQN(TRAIN_MODEL_SIZE).to(device)
target_net.load_state_dict(policy_net.state_dict())

optimizer = optim.AdamW(policy_net.parameters(), lr=LR, amsgrad=True)

import os
import pickle
if os.path.isfile("memory.pk"):
    with open('memory.pk', 'rb') as f:
        memory = pickle.load(f)
else:
    memory = init_memory(default_reward, 
        TRAIN_MEMORY_EPISODES, 
        TRAIN_MEMORY_EPISODE_SIZE, 
        TRAIN_MEMORY_SIZE, 
        TRAIN_MEMORY_THREADS)
    with open('memory.pk', 'wb') as f:
        pickle.dump(memory, f)


optimize_model_steps = 0
for i in tqdm(range(TRAIN_MODEL_INIT_STEPS)):
    loss = optimize_model(policy_net, target_net, optimizer, memory)
    optimize_model_steps+=1
    if optimize_model_steps % TRAIN_MEMORY_EPISODE_SIZE == 0:
        memory = add_episode(reward, memory, TRAIN_MEMORY_EPISODE_SIZE)
    if optimize_model_steps % 500 == 0:
        wandb.log({"loss": loss}, step=optimize_model_steps)

env = TetrisEnv()

steps_done = 0

if torch.cuda.is_available() or torch.backends.mps.is_available():
    num_episodes = TRAIN_EPISODES_GPU
else:
    num_episodes = TRAIN_EPISODES_CPU

for i_episode in tqdm(range(num_episodes)):
    # Initialize the environment and get its state
    state, info = env.reset()
    state = s2t(state)
    total_reward = 0
    for t in count():
        eps_threshold = EPS_END + (EPS_START - EPS_END) * math.exp(
            -1.0 * steps_done / EPS_DECAY
        )
        steps_done += 1
        action = select_action(env, policy_net, state, info, eps_threshold)

        item = action.item()
        next_act = [a2i(k[0]) for k in env.vim_state.next_actions_and_states]
        if item not in next_act:
            if not next_act:
                print("NO NEXT ERROR WTF")
                break
            item = random.choice(next_act)
        observation, reward, terminated, truncated, info = env.step(item)
        total_reward += reward
        reward = torch.tensor([reward], device=device)
        done = terminated or truncated

        if terminated:
            next_state = None
        else:
            next_state = s2t(observation)

        # Store the transition in memory
        memory.push(state, action, next_state, reward)

        # Move to the next state
        state = next_state

        # Perform one step of the optimization (on the policy network)
        loss = optimize_model(policy_net, target_net, optimizer, memory)
        optimize_model_steps+=1
        if optimize_model_steps % 500 == 0:
            wandb.log({"loss": loss}, step=optimize_model_steps)
        # Soft update of the target network's weights
        # θ′ ← τ θ + (1 −τ )θ′
        target_net_state_dict = target_net.state_dict()
        policy_net_state_dict = policy_net.state_dict()
        for key in policy_net_state_dict:
            target_net_state_dict[key] = policy_net_state_dict[
                key
            ] * TAU + target_net_state_dict[key] * (1 - TAU)
        target_net.load_state_dict(target_net_state_dict)

        if done or t > TRAIN_EPISODE_SIZE:
            wandb.log({"total_reward": total_reward, 
                "total_move_count": env.vim_state.total_move_count,
                "score": env.vim_state.score,
                "total_lines": env.vim_state.total_lines,
                "holes": env.vim_state.holes,
                "height": env.vim_state.height,}, step=optimize_model_steps)
            break
    policy_net_scripted = torch.jit.script(policy_net)
    policy_net_scripted.save("policy_net.pt")
    if i_episode % 20 == 0:
        run.log_model(path="policy_net.pt", name=WANDB_MODEL_NAME)
wandb.finish()