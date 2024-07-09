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
    bot_action = a2i(env.vim_state.generate_bot_episode("wordpress", 1)[0][0])
    obs, reward, terminated, _, info = env.step(bot_action)

    if terminated:
        break
print(env.render())