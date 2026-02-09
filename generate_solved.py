def generate_snail_goal(size):
    # Initialize a 2D grid with zeros
    board = [[0] * size for _ in range(size)]
    
    current_val = 1
    top, bottom = 0, size - 1
    left, right = 0, size - 1
    
    while top <= bottom and left <= right:
        # Move Right
        for i in range(left, right + 1):
            board[top][i] = current_val
            current_val += 1
        top += 1
        
        # Move Down
        for i in range(top, bottom + 1):
            board[i][right] = current_val
            current_val += 1
        right -= 1
        
        # Move Left
        if top <= bottom:
            for i in range(right, left - 1, -1):
                board[bottom][i] = current_val
                current_val += 1
            bottom -= 1
            
        # Move Up
        if left <= right:
            for i in range(bottom, top - 1, -1):
                board[i][left] = current_val
                current_val += 1
            left += 1

    # In N-Puzzle, the last number (N*N) is actually the blank (0)
    max_val = size * size
    flat_board = []
    for row in board:
        for val in row:
            flat_board.append(0 if val == max_val else val)
            
    return flat_board

# Example usage for 17x17:
size = 17
snail_goal = generate_snail_goal(size)

# Create the Reverse Lookup Table for Heuristics
# target_pos[tile_value] = (y, x)
target_pos = {}
for idx, val in enumerate(snail_goal):
    target_pos[val] = (idx // size, idx % size)

print(f"Goal generated. Tile 1 is at: {target_pos[1]}")
print(f"Blank (0) is at: {target_pos[0]}")


