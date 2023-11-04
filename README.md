# Space Shooter
A simple 2D wave based asteroid shooter.

## Compilation
There are two options for compiling:
1. If you have Standalone CMake (not installed via VS Installer) and it's in path, just compiling like a normal rust program should work.
2. Remove the "bundled" feature from sdl2 in Cargo.toml and manually download/compile SDL2 >= 2.0.14 

## Program Design
First of, my original plan was to have almost all code nicely tucked away into struct, but because I didn't feel like it was worth it to deal with all the self referential stuff of sdl2, I ended up with what I have now, as in, a lot of things in fn main.

My main goals were to keep the different types of objects separated in their own Vectors, use the #[inline] attribute and minimize the amount of times the objects needed to be looped over. 

Only the last one really needs more explanation, and to be specific what I mean is that the difference in memory between separating out, say collision and rendering from the asteroid struct didn't seem to be significant enough to warrant both needing to loop through the asteroids twice, once for rendering and the other for collision, but also, I would at some point need to loop over both of them at the same time, forcing me to read two very different parts of memory at the same time. All that only to save reading a memory pointer, a 64 bit float, four 32 bit integers, and one Option which should be close to a bool.

Now I did end up testing separating the data needed for collisions due to the profiler always saying that most of the cpu time went to collisions, but after testing, it didn't seem to make a difference. This test is in the branch test-1 and I guess it might be a bit closer to a ECS system. It might have been faster than the alternatives if I tried to combine the update function (movement) and collision detection into loop, but, it's late, and considering that that's pretty much the only big difference between the "before" and "after" branches and it making pretty much no difference when profiling/benchmarking, it didn't really seem worth it to test.

## Profiling/Benchmarking
All of the profiling/benchmark attemps stayed within a reasonable margin of error of each other. I also counted every iteration of the main loop. All of these were run using the "benchmark" feature (I know features aren't the right way to do benchmarks, but it's a binary, so rusts inbuilt one wouldn't work, and I didn't have time to look into the correct way.) That tries to shoot constantly and makes the player unkillable so all asteroids can spawn. The wave amount were manually changed, with the last 3 waves' asteroid_amount values being the same as the "5k", "10k" or "75k".

### The frame counts:
- before: 213818
- before 5k: 114568
- before 10k: 93129
- after: 216279
- after 5k: 115001
- after 10k 92336
- after 75k: 71928
- test-1: 214133
- test-1 5k: 106200
- test-1 10k: 80273
- test-1 75k: 65512

I would have linked the screenshots from AMD uProf, but there are many of them and they take up a lot of space, I left them in the Screenshots directory. The TLDR being, almost all the cpu time went toward collisions and the only noteworthy ones being the two 75k tests due to the test-1 version getting less frames as seen above, but was waay smoother with the after version mostly being stuck on vcruntime.

As such, the only thing I really changed that showed on the profiler (this doesn't show in the screenshots, I apparently did that change before creating the before branch) was not doing square root in the collision detection, of course, that was something I considered at the moment I wrote it, but wanted to make sure it worked as it should first. 

I did do other changes based on the whole collision detection being the bottleneck, mainly doing the asteroid moving and collision detection in the same loop, but, it made almost no difference, as mentioned earlier.

## Possible changes
There is currently only one thing that I feel would truly work, which is doing something similar to [Fix Your Timestep!](https://gafferongames.com/post/fix_your_timestep/) and only doing collision detection/movement code at say 30FPS and lerping between the last state and the next one. But, I am already late in submitting, and it doesn't really feel like it fits the course.

## Assets/Tutorials
- https://arcadeisland.itch.io/space-shooter-wang-tiles
- https://sunjay.dev/learn-game-dev/rendering-an-image.html
- https://www.pixilart.com/art/a-ball-of-energy-afb035cc27792af