# Performance

Light system works but is not performant. I think there's plenty of room at the top:

- Combine the hitboxes where possible
- Reuse meshes instead of creating new ones

If it's still slow after this...

- Try and make a new system that's like AtMostFrameRate that works like old bullettime (this helps a little not a lot)

If it's still slow after that...

- Figure out a way to only recalculate meshes for things that have changed (hard)

# Conversation

A single text box:

- Speaker
- Emotion
- Text

A speaker:

- Map from emotion to:
    - Portrait
    - Voice

A conversation:

- Collection of text boxes

# More conversation

Farm the rest of those sound effects, and refactor the system so it just takes a string. Then make the conversation feel great.

# Ahhhhhhh Okay let's Lock THE dffcuck infdskj

Tonight
- [x] Fix the conversation bug
- [x] Fix the pushing bug
- [x] Reaper animations
- [x] Reaper scary moments
- [x] And also fix clouds
- [x] Weird player wall jump thing with passup
- [x] Reaper conversation

- [x] THEN break from reaper and do screen transitions
    - [x] First make it take time and have the screen scroll
    - [x] Then try and figure out how celeste does vertical ones
        - [x] And then always set yvel to same amount?
        - I should maybe still make the squatable vertical? Idk

- [x] THEN go for a bike ride

- [ ] THEN crush these quick things to get back into it
- [x] Also make the circle transitions look smoother
- [x] And add particle effects for head that do a better job showing if you can dash
- [x] Also juice a look up animation
- [x] Jump depend on press duration
- [x] Fix compilation warnings
- [x] Fix gh lfs
- [x] Fix despawn in animation crate to try to get comms (I think)

- [x] Psych try to do the CRT thing quickly and easily
    - Jury still out on whether it is good to have...

- [x] THEN go for more gameplay elements
    - [x] Wall that eats you
        - QUICK FIX: It shouldnt have the dark bg at top/bottom, so I could just put it on single platform if I wanted (don't have to surround to get the right edges)
    - [x] Reaper fucking stuff up

- [x] Scythes should know their parent and despawn when parent doesn't exist of x is greater than parent

# Moresavefiles

I need to figture out what the overworld is going to be

## Yay it works (if I can easily copy intgrid to new project, and maybe update later (through json I think))

- Resume / restart portal thingys
- Figure out how to make this work with states... (maybe it's as easy as just making sure saving current run isn't fucked? Idk)

## Noooo it doesn't

Then make it work. This is the way to make the overworld

# More menus

Oh boy. Idk what to do here. Maybe try to make some kind of button system? Maybe try to find a library? It will not be pretty

I think I should just come up with my own minmal one. I really only need simple buttons at positions, with defined next so that you can control them with keyboard
