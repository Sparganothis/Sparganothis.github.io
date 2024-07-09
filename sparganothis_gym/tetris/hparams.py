# BATCH_SIZE is the number of transitions sampled from the replay buffer
# GAMMA is the discount factor as mentioned in the previous section
# EPS_START is the starting value of epsilon
# EPS_END is the final value of epsilon
# EPS_DECAY controls the rate of exponential decay of epsilon, higher means a slower decay
# TAU is the update rate of the target network
# LR is the learning rate of the ``AdamW`` optimizer

REWARD_END = -10
REWARD_SOFT = -1
REWARD_MOVE = -0.1
REWARD_ROTATE = 0
REWARD_SCORE = 0.01

TRAIN_MEMORY_EPISODES = 20
TRAIN_MEMORY_EPISODE_SIZE = 250
TRAIN_MEMORY_SIZE = 10_000

TRAIN_MODEL_SIZE = 128
TRAIN_MODEL_INIT_STEPS = 1000
TRAIN_EPISODES_CPU = 50
TRAIN_EPISODES_GPU = 600

TRAIN_EPISODE_SIZE = 100

BATCH_SIZE = 128
GAMMA = 0.99
EPS_START = 0.9
EPS_END = 0.05
EPS_DECAY = 1000
TAU = 0.005
LR = 1e-4