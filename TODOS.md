# Performance

Light system works but is not performant. I think there's plenty of room at the top:

- Combine the hitboxes where possible
- Reuse meshes instead of creating new ones

If it's still slow after this...

- Try and make a new system that's like AtMostFrameRate that works like old bullettime (this helps a little not a lot)

If it's still slow after that...

- Figure out a way to only recalculate meshes for things that have changed (hard)
