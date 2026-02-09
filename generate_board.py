import sys
import random

def create_snail_coords(n):
    """Generates coordinates in a snail/spiral pattern."""
    coords = []
    top, bottom = 0, n - 1
    left, right = 0, n - 1
    
    while top <= bottom and left <= right:
        # Move Right
        for i in range(left, right + 1):
            coords.append((top, i))
        top += 1
        # Move Down
        for i in range(top, bottom + 1):
            coords.append((i, right))
        right -= 1
        # Move Left
        if top <= bottom:
            for i in range(right, left - 1, -1):
                coords.append((bottom, i))
            bottom -= 1
        # Move Up
        if left <= right:
            for i in range(bottom, top - 1, -1):
                coords.append((i, left))
            left += 1
    return coords

def generate_board(n):
    snail_coords = create_snail_coords(n)
    board = [[0] * n for _ in range(n)]
    
    for i in range(n * n - 1):
        r, c = snail_coords[i]
        board[r][c] = i + 1
    
    empty_r, empty_c = snail_coords[-1]
    board[empty_r][empty_c] = 0
    
    moves = 500
    curr_r, curr_c = empty_r, empty_c
    
    for _ in range(moves):
        adjacents = []
        for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            nr, nc = curr_r + dr, curr_c + dc
            if 0 <= nr < n and 0 <= nc < n:
                adjacents.append((nr, nc))
        
        # Pick a random neighbor and swap
        next_r, next_c = random.choice(adjacents)
        board[curr_r][curr_c], board[next_r][next_c] = board[next_r][next_c], board[curr_r][curr_c]
        curr_r, curr_c = next_r, next_c
        
    return board

def print_board(board):
    for row in board:
        print(" ".join(f"{val:2}" for val in row))

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python script.py [n]")
    else:
        n = int(sys.argv[1])
        board = generate_board(n)
        print(n)
        print_board(board)