from tetris.env import *
import os
import time

env = TetrisEnv()

obs, info = env.reset()

# Take some random actions
while True:
    print(env.render())
    time.sleep(0.25)
    os.system('cls')
    rand_action = env.action_space.sample(mask=info["action_mask"])
    obs, reward, terminated, _, info = env.step(rand_action)

    if terminated:
        break
print(env.render())