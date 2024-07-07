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

state3 = state.next_actions_and_states[0][1]


print(" is_t_spin ", state.is_t_spin)
print(" is_t_mini_spin ", state.is_t_mini_spin)
print(" is_b2b ", state.is_b2b)
print(" combo_counter ", state.combo_counter)
print(" total_garbage_sent ", state.total_garbage_sent)
print(" garbage_recv ", state.garbage_recv)

print("total_move_count\n", state3.total_move_count)
print("matrix_txt\n", state3.matrix_txt)
print("html\n", state3.html)