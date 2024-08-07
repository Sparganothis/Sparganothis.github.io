import random
import torch
import torch.nn as nn
import torch.nn.functional as F
from collections import namedtuple

from tetris.env import *
from tetris.hparams import *

import tqdm 

Transition = namedtuple("Transition", ("state", "action", "next_state", "reward"))

# if GPU is to be used
device = torch.device(
    CUDA_DEVICE
    if torch.cuda.is_available()
    else "mps" if torch.backends.mps.is_available() else "cpu"
)


class DQN(nn.Module):

    def __init__(self, num_features):
        self.num_features = num_features
        super(DQN, self).__init__()
        self.layer_hold = nn.Linear(len(ALL_PIECES) + 1, num_features)
        self.layer_next = nn.Linear(5 * len(ALL_PIECES), num_features)
        self.layer_hcf = nn.Linear(len(TRAIN_MODEL_HCF), num_features)
        self.layer_board = nn.Sequential(
            nn.Conv2d(1, num_features // 8, kernel_size=5, stride=2, padding=2),
            nn.ReLU(),
            nn.Conv2d(num_features // 8, num_features // 8, kernel_size=3, stride=1, padding=1),
            nn.ReLU(),
            nn.Conv2d(num_features // 8, num_features // 8, kernel_size=3, stride=1, padding=1),
            nn.ReLU(),
        )
        self.layer_hiden = nn.Linear(3 * num_features + BOARD_SHAPE[0] * BOARD_SHAPE[1] // 4 * num_features // 8, num_features)
        self.layer_actions = nn.Linear(num_features, len(ALL_ACTIONS))

    # Called with either one element to determine next action, or a batch
    def forward(self, board, next, hold, hcf):
        b, h, w = board.shape

        bx = F.relu(
            self.layer_board(board[:,None,:,:])
        ).view(b, w * h // 4 * self.num_features // 8)
        nx = F.relu(
            self.layer_next(next.view(b, 5 * next.shape[-1]))
        )
        hx = F.relu(self.layer_hold(hold))

        hcfx = F.relu(self.layer_hcf(hcf))

        r = F.relu(self.layer_hiden(torch.cat([bx, nx, hx, hcfx], dim=-1)))
        r = self.layer_actions(r)
        return r

def select_action(env, policy_net, state, info, eps_threshold):
    sample = random.random()
    
    if sample > eps_threshold:
        with torch.no_grad():
            return (
                (
                    policy_net(
                        state["board"][None, ::].to(device),
                        state["next"][None, ::].to(device),
                        state["hold"][None, ::].to(device),
                        state["hcf"][None, ::].to(device),
                    ).squeeze()
                )
                .argmax()
                .view(1, 1)
            ).to("cpu")
    else:
        return torch.tensor(
            [[env.action_space.sample(mask=info['action_mask'])]],
            dtype=torch.long,
        )

def optimize_model(policy_net, target_net, optimizer, memories, batch_sizes):
    transitions = []
    for mem_size, memory in zip(batch_sizes, memories):
        if len(memory) < mem_size:
            return
        transitions += memory.sample(mem_size)
    if not transitions:
        return
    # Transpose the batch (see https://stackoverflow.com/a/19343/3343043 for
    # detailed explanation). This converts batch-array of Transitions
    # to Transition of batch-arrays.
    batch = Transition(*zip(*transitions))

    # Compute a mask of non-final states and concatenate the batch elements
    # (a final state would've been the one after which simulation ended)
    non_final_mask = torch.tensor(
        tuple(map(lambda s: s is not None, batch.next_state)),
        dtype=torch.bool,
    ).to(device)
    non_final_next_states = [s for s in batch.next_state if s is not None]
    non_final_next_states = {
        "board": torch.cat([s["board"][None, ::] for s in non_final_next_states]).to(device),
        "next": torch.cat([s["next"][None, ::] for s in non_final_next_states]).to(device),
        "hold": torch.cat([s["hold"][None, ::] for s in non_final_next_states]).to(device),
        "hcf": torch.cat([s["hcf"][None, ::] for s in non_final_next_states]).to(device),
    }

    state_batch = {
        "board": torch.cat([s["board"][None, ::] for s in batch.state]).to(device),
        "next": torch.cat([s["next"][None, ::] for s in batch.state]).to(device),
        "hold": torch.cat([s["hold"][None, ::] for s in batch.state]).to(device),
        "hcf": torch.cat([s["hcf"][None, ::] for s in batch.state]).to(device),
    }
    action_batch = torch.cat(batch.action).to(device)
    reward_batch = torch.cat(batch.reward).to(device)

    # Compute Q(s_t, a) - the model computes Q(s_t), then we select the
    # columns of actions taken. These are the actions which would've been taken
    # for each batch state according to policy_net
    
    state_action_values = policy_net(
        state_batch['board'],
        state_batch["next"],
        state_batch['hold'],
        state_batch['hcf'],
    ).gather(1, action_batch)
    # Compute V(s_{t+1}) for all next states.
    # Expected values of actions for non_final_next_states are computed based
    # on the "older" target_net; selecting their best reward with max(1).values
    # This is merged based on the mask, such that we'll have either the expected
    # state value or 0 in case the state was final.
    next_state_values = torch.zeros(sum([s for s in batch_sizes])).to(device)
    with torch.no_grad():
        next_state_values[non_final_mask] = (
            target_net(
                non_final_next_states['board'],
                non_final_next_states["next"],
                non_final_next_states['hold'],
                non_final_next_states['hcf'],
            ).max(1).values
        )
    # Compute the expected Q values
    expected_state_action_values = (next_state_values * GAMMA) + reward_batch

    # Compute Huber loss
    criterion = nn.SmoothL1Loss()
    loss = criterion(state_action_values, expected_state_action_values.unsqueeze(1))

    # Optimize the model
    optimizer.zero_grad()
    loss.backward()
    # In-place gradient clipping
    torch.nn.utils.clip_grad_value_(policy_net.parameters(), 1)
    optimizer.step()

    return loss.item()


def s2t(o):
    return {
        "board": torch.tensor(o["board"], dtype=torch.float32),
        "next": F.one_hot(
            torch.tensor(o["next"], dtype=torch.long),
            num_classes=len(ALL_PIECES),
        ).float(),
        "hold": F.one_hot(
            torch.tensor(o["hold"], dtype=torch.long),
            num_classes=len(ALL_PIECES) + 1,
        ).float(),
        "hcf": torch.tensor(
            o["hcf"],
            dtype=torch.float
        )
    }