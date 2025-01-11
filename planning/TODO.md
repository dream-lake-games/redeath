# 0.1.1

BRAIN DUMP:

- [x] Double check up + drift right does not go as far right as diagonal right
- [x] Input buffering
- [x] Ability to change dash direction until very last moment
- [x] Celeste keybindings
- [x] Controller support
- [x] Fix chasing bug

# reDEATH version v0.1.1 is here!

This is small update, mainly to address some quality-of-life issues that the first playtesters found in the previous build. But it's also got some new stuff!

## Controls

### Controller support

The game now has controller support! Use the left stick to move, south button to jump, and west button to dash. Start = Enter, select = escape. NOTE: As long as you have _a_ controller connected, the game will assume you want to use it. To get back to keyboard, you'll have to completely disconnect your controller. Obviously this is sub-optimal, but I'll fix it later when I overhaul bindings and settings UI.

### Multiple keyboard input modes

You can now chose between two keyboard input modes:
- WASD + J to jump, K to dash. To select, hold down AWD at the same time.
- Arrow keys + C to jump, K to dash. To select, hold down left, up, right arrow keys at the same time.

## Input

### Dash "suspense"

Previously when you dashed, the direction of your dash would be decided the frame you pressed the button. Then the physics would stop for 0.1s as an effect, and then you would move again. Seems like a wasted 0.1s, right?

Now, you can change the direction of your dash during that stopped time. Hopefully this opens the door for some more interesting sequences later on. You may have to slightly adjust your muscle memory, sorry.

### Input buffering

Jumps and dashes are now buffered for 0.1s as well. This should make it a little more forgiving if you hit jump a few frames earlier, and will hopefully make complicated sequences more repeatable.

## Bugs

Fixed a bug where collecting an egg, leaving the level, then reentering the level would sometimes crash the game.

## What's next?

The next patch will also be mostly quality of life fixes. After that, I'm going to focus on the lighting system, bloom, rain, and particle effects to hopefully make the world more eye-catching.

# 0.1.2

- [ ] Improve lighting FPS
    - [ ] Divide the lights like I do the on-screen calcs
- [ ] Wall jumps even more forgiving 
- [ ] Look into framerate limiting
- [ ] More clear what is solid and what isn't
- [ ] Scythe shoot more obvious
- [ ] Maybe make scythes go the same distance
- [ ] Wall kick without horizontal input does something different
- [ ] Say press enter on title screen

OTHER STUFF:

- Blog post about what I learned from GBJam 12
- Maybe blog post about Bevy 0.15 (after 0.1.1, I've seen it a bit more)
