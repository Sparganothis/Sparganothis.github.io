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
        "MEMORY_EPISODES": TRAIN_MEMORY_EPISODES,
        "TRAIN_MEMORY_EPISODE_SIZE": TRAIN_MEMORY_EPISODE_SIZE,
        "TRAIN_MEMORY_SIZE_LONG": TRAIN_MEMORY_SIZE_LONG,
        "TRAIN_MEMORY_SIZE_SHORT": TRAIN_MEMORY_SIZE_SHORT,
        "TRAIN_MEMORY_SIZE_HUMAN": TRAIN_MEMORY_SIZE_HUMAN,
        "TRAIN_MODEL_INIT_STEPS": TRAIN_MODEL_INIT_STEPS,
        "TRAIN_EPISODES_CPU": TRAIN_EPISODES_CPU,
        "TRAIN_EPISODES_GPU": TRAIN_EPISODES_GPU,
        "TRAIN_EPISODE_SIZE": TRAIN_EPISODE_SIZE,
        "TRAIN_LOG_INTERVAL": TRAIN_LOG_INTERVAL,
        "BATCH_SIZE": BATCH_SIZE,
        "BATCH_SIZE_SHORT": BATCH_SIZE_SHORT,
        "BATCH_SIZE_HUMAN": BATCH_SIZE_HUMAN,
        "PPO_GAMMA": GAMMA,
        "PPO_EPS_INTERVAL": EPS_INTERVAL,
        "PPO_EPS_START": EPS_START,
        "PPO_EPS_END": EPS_END,
        "PPO_EPS_DECAY": EPS_DECAY,
        "PPO_TAU": TAU,
        "PPO_LR": LR,
    }
)

print(device)


policy_net = DQN(TRAIN_MODEL_SIZE).to(device)
target_net = DQN(TRAIN_MODEL_SIZE).to(device)
target_net.load_state_dict(policy_net.state_dict())

optimizer = optim.AdamW(policy_net.parameters(), lr=LR, amsgrad=True)



def wordpress_bot():
    env = TetrisEnv()
    return env.vim_state.generate_bot_episode("wordpress", TRAIN_MEMORY_EPISODE_SIZE)
import os
import pickle
if os.path.isfile("memory.pk"):
    with open('memory.pk', 'rb') as f:
        memory = pickle.load(f)
else:
    memory = init_memory(default_reward, 
        TRAIN_MEMORY_EPISODES, 
        wordpress_bot, 
        TRAIN_MEMORY_SIZE_LONG)
    with open('memory.pk', 'wb') as f:
        pickle.dump(memory, f)

short_memory = ReplayMemory(TRAIN_MEMORY_SIZE_SHORT)
add_episode(default_reward, short_memory, wordpress_bot())

human_memory = ReplayMemory(TRAIN_MEMORY_SIZE_HUMAN)

import glob
human_data = glob.glob('data')
for replay in human_data:
    with open(replay, "rb") as f:
        replay_bytes = f.read()
    _, ep = sparganothis_vim.GameStatePy.load_replay_from_bytes(replay_bytes)
    add_episode(default_reward, human_memory, ep)

optimize_model_steps = 0

if os.path.isfile("policy_net_states.pt") and os.path.isfile("optimizer_states.pt"):
    with open("policy_net_states.pt", "rb") as f:
        policy_net.load_state_dict(torch.load(f))
    with open("optimizer_states.pt", "rb") as f:
        optimizer.load_state_dict(torch.load(f))
else:
    for i in tqdm(range(TRAIN_MODEL_INIT_STEPS),desc="pretrain"):
        loss = optimize_model(policy_net, target_net, optimizer, [human_memory], [BATCH_SIZE_HUMAN])
        optimize_model_steps += 1
        if optimize_model_steps % TRAIN_LOG_INTERVAL == 0:
            wandb.log({"pretrain_loss": loss}, step=optimize_model_steps)
    torch.save(policy_net.state_dict(), "policy_net_states.pt")
    torch.save(optimizer.state_dict(), "optimizer_states.pt")

env = TetrisEnv()

steps_done = 0

if torch.cuda.is_available() or torch.backends.mps.is_available():
    num_episodes = TRAIN_EPISODES_GPU
else:
    num_episodes = TRAIN_EPISODES_CPU

for i_episode in tqdm(range(num_episodes), desc="episodes"):
    # Initialize the environment and get its state
    state, info = env.reset()
    state = s2t(state)
    total_reward = 0
    for t in count():
        eps_threshold = EPS_END + (EPS_START - EPS_END) * math.exp(
            -1.0 * steps_done / EPS_DECAY
        )
        steps_done += 1
        steps_done = steps_done % EPS_INTERVAL
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
        reward = torch.tensor([reward])
        done = terminated or truncated

        if terminated:
            next_state = None
        else:
            next_state = s2t(observation)

        # Store the transition in memory
        memory.push(state, action, next_state, reward)
        short_memory.push(state, action, next_state, reward)

        # Move to the next state
        state = next_state

        # Perform one step of the optimization (on the policy network)
        loss = optimize_model(policy_net, target_net, optimizer, 
            [memory, short_memory, human_memory], 
            [BATCH_SIZE, BATCH_SIZE_SHORT, BATCH_SIZE_HUMAN])
        optimize_model_steps+=1
        if optimize_model_steps % TRAIN_LOG_INTERVAL == 0:
            wandb.log({"loss": loss, "eps_threshold": eps_threshold}, step=optimize_model_steps)
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
                "bumpi": env.vim_state.bumpi,
                "height": env.vim_state.height,}, step=optimize_model_steps)
            break
    torch.save(policy_net.state_dict(), "policy_net_states.pt")
    torch.save(optimizer.state_dict(), "optimizer_states.pt")
    policy_net_scripted = torch.jit.script(policy_net)
    policy_net_scripted.save("policy_net.pt")
    if i_episode % TRAIN_LOG_INTERVAL == 0:
        run.log_model(path="policy_net_states.pt", name=WANDB_MODEL_NAME + "policy")
        run.log_model(path="optimizer_states.pt", name=WANDB_MODEL_NAME + "optimizer")
wandb.finish()