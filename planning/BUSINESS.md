# Build woes

## Local Sanity First

### A NOTE ON WASM: I can't do rn. Why?

- Try to do a WASM build for the hell of it
    - If it doesn't work (stuff to wide) ask about any other solutions?
- Clean up wasm build (it should be a single command that does it, along with a single command to test that locally)

- Need to make everything vec4 in shaders (issue I had before)
- I am running into texture max size limits :(
- I am going to just skip for now and ignore. I might never prioritize this, we'll see

### Apple Silicon

- [x] Get the apple silicon build working locally
- [x] Clean up the local apple silicon build (it should be a single command)

### Windows

Learnings from fucking around in parallel:

- Holy shit I can cross compile from my native mac to x86_64-pc-windows-gnu (after brew installing something for a linker I forget what) and it runs
- I _think_ the issue is that the WGPU backend relies on DirectX12, which doesn't work in parallel
    - This is backed up by the fact that the breakout stuff loads some stuff, but not the mesh stuff

TLDR:

I shouldk be able to make a script _which I can run locally_ (hype) that produces a zip with the executable and the assets folder and runs on non-parallel windows

## CI insanity

I don't think I need to set this up for now. Eventually, maybe, but for now no.

# Itch Page

- Make a page for the game that is barebones
- Record exciting gameplay and put the trailer (with sound!!!) on the website
- Make some custom assets so the page looks clean and inviting
- Upload the builds

# Dream Lake Website

- Find next template to copy
- Prosper
