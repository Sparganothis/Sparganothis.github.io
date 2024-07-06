import sparganothis_vim
print(sparganothis_vim.sum_as_string(5, 20))

seed = sparganothis_vim.generate_random_seed()
print("seed1", seed)
print("seed1", seed.ts)
print("seed1", seed.seed)
"seed1", 
seed2 = sparganothis_vim.generate_seed(0, [0]*32)
print("seed2", seed2)
print("seed2", seed2.ts)
print("seed2", seed2.seed)


state = sparganothis_vim.GameStatePy(seed2)
print(state)

print("state getters:", [k for k in dir(state) if not k.startswith("_")])
print("seed getters:", [k for k in dir(seed) if not k.startswith("_")])

print(" current state  ", state.current_pcs_rotation )
print(" debug current info  ", state.debug_current_pcs_info)
print(" game over  ", state.game_over )
print(" hold ", state.hold)
print(" main ", str(state.main_board)[0:20] )
print(" next ", state.next_pcs )
print(" total_lines ", state.total_lines)
print(" score ", state.score)

print(" next actions ", [k[0] for k in state.next_actions_and_states])

