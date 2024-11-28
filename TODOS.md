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

- [ ] THEN break from reaper and do screen transitions
    - [ ] First make it take time and have the screen scroll
    - [ ] Then try and figure out how celeste does vertical ones
        - [ ] Maybe a platform you can squat down through?
        - [ ] And then always set yvel to same amount?

- [ ] THEN go for a bike ride

- [ ] THEN crush these quick things to get back into it
- [ ] Also make the circle transitions look smoother
- [ ] And add particle effects for head that do a better job showing if you can dash
- [ ] Also juice a look up animation
- [ ] Jump depend on press duration

- [ ] THEN go for more gameplay elements
    - [ ] Wall that eats you
    - [ ]

