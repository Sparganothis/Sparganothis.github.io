# BATCH_SIZE is the number of transitions sampled from the replay buffer
# GAMMA is the discount factor as mentioned in the previous section
# EPS_START is the starting value of epsilon
# EPS_END is the final value of epsilon
# EPS_DECAY controls the rate of exponential decay of epsilon, higher means a slower decay
# TAU is the update rate of the target network
# LR is the learning rate of the ``AdamW`` optimizer

WANDB_MODEL_NAME = ""

CUDA_DEVICE = "cuda:1"

REWARD_END = -10
REWARD_SOFT = 0
REWARD_MOVE = 0
REWARD_ROTATE = 0
REWARD_SCORE = 0.01

TRAIN_MEMORY_EPISODES = 500
TRAIN_MEMORY_EPISODE_SIZE = 500
TRAIN_MEMORY_SIZE = 1_000_000
TRAIN_MEMORY_THREADS = 8

TRAIN_MODEL_SIZE = 256
TRAIN_MODEL_INIT_STEPS = 20_000
TRAIN_EPISODES_CPU = 50000
TRAIN_EPISODES_GPU = 50000
TRAIN_EPISODE_SIZE = 500
TRAIN_LOG_INTERVAL = 500

BATCH_SIZE = 128
GAMMA = 0.99
EPS_INTERVAL = 200_000
EPS_START = 0.9
EPS_END = 0.05
EPS_DECAY = 10_000
TAU = 0.005
LR = 1e-3