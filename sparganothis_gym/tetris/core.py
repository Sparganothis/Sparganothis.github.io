HARD_DROP = "HardDrop"
SOFT_DROP = "SoftDrop"
MOVE_LEFT = "MoveLeft"
MOVE_RIGHT = "MoveRight"
HOLD = "Hold"
ROTATE_LEFT = "RotateLeft"
ROTATE_RIGHT = "RotateRight"

ALL_ACTIONS = [
    HARD_DROP,
    SOFT_DROP,
    MOVE_LEFT,
    MOVE_RIGHT,
    HOLD,
    ROTATE_LEFT,
    ROTATE_RIGHT
]
ALL_PIECES = ["I", "J", "L", "O", "S", "T", "Z"]
BOARD_SHAPE = [20, 10]

def a2i(a):
    return ALL_ACTIONS.index(a)

def i2a(i):
    return ALL_ACTIONS[i]

def p2i(p):
    if not p:
        return -1
    return ALL_PIECES.index(p)

def i2p(i):
    return ALL_PIECES[i]