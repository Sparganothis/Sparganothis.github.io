import sparganothis_vim
print(sparganothis_vim.sum_as_string(5, 20))

seed = sparganothis_vim.generate_random_seed()
print(seed)

seed2 = sparganothis_vim.generate_seed(0, [0]*32)
print(seed2)

state = sparganothis_vim.GameStatePy(seed)
print(state)

print(dir(state))
print(dir(seed))