# Pacman
A recreation of the arcade game "Pacman" with rust and the bevy engine.

Despite its age and appearance, Pacman was a quite complex game. Therefore, this project aims to battle test bevy.

## TODOs
(This is a WIP list of things to implement. Will be updated frequently)

### Pacman
- slow down when eating an energizer
- "waka waka" animation + sound when eating dots (very important)

### Ghosts
- animations

### Lifecycle
- the game ends if pacman dies without remaining lives
- different ghost behaviour based on level/time

### Tunnel
- (Maybe) remove the tunnel component and only use the board resource

### Fruit
- enable fruit spawn
- fruit points and appearance change based on the current level
- eating the fruits gives pacman points

### Graphics
- sprites for pacman
- sprites for ghosts
- sprites for dots and energizer
- sprites for the maze (there are 3 different kinds of walls: outer -> thick, inner -> thin, ghost house -> square)
- return the correct z coordinate from the board (or only x and y)

### Appearance
- proper game start
- dramatic pauses when specific actions happen (like eating ghosts)
- sound (this one might be split later)

### UI
- main menu to start the game (and later access the high score)
- pause menu to quit the game

### Refactoring
- add every map element (walls, tunnels, ghost house) as an entity
- (Maybe) change the point to a field struct to remove these stupid getters
- (Maybe) remove the Position component (but keep using it in systems). It can be generated quickly from a transform. This way only the transform needs to be updated.