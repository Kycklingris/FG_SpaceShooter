# Space Shooter
A simple 2D wave based asteroid shooter.

## Compilation
There are two options for compiling:
1. If you have Standalone CMake (not installed via VS Installer) and it's in path, just compiling like a normal rust program should work.
2. Remove the "bundled" feature from sdl2 in Cargo.toml and manually download/compile SDL2 >= 2.0.14 

## Program Design
First of, my original plan was to have almost all code nicely tucked away into struct, but because I didn't feel like it was worth it to deal with self referential fields, I ended up with what I have now, as in, a lot of things in fn main.

My main goal was to keep the different types of objects separated in their own Vectors, use the #[inline] attribute, also mostly for vectorization and minimize the amount of times the objects needed to be looped over. 

Only the last one really needs more explanation, and to be specific what I mean is that the difference in memory between separating out say collision and rendering from the asteroid struct didn't seem to be significant enough to warrant both needing to loop through the asteroids twice, once for rendering and the other for collision, but also, I would at some point need to loop over both of them at the same time, forcing me to read two very different parts of memory at the same time. All that only to save reading a memory pointer, a 64 bit float, four 32 bit integers, and one option which should be close to a bool.

## Profiling/Benchmarking
With the main changes I was working on, I never noticed any real performance difference but, when writing this readme, I realized that I may have just been incredibly stupid, because after dividing the frame count by about the runtime I got about 3,500 fps, meaning it wouldn't be strange if windows just wasn't allowing any more, this might also be supported by the fact that most of the cpu time was used by windows.

## Assets/Tutorials
- https://gafferongames.com/post/fix_your_timestep/
- https://arcadeisland.itch.io/space-shooter-wang-tiles
- https://sunjay.dev/learn-game-dev/rendering-an-image.html
- https://www.pixilart.com/art/a-ball-of-energy-afb035cc27792af