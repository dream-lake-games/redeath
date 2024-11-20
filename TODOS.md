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
