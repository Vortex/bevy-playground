# Bevy Playground

Just a place to fool around with the Bevy game development framework.

## How to run

If you have Rust installed just checkout the code and do 

```bash
cargo run
```

It's that simple. :)

If you don't, you can read how to install it [here](https://www.rust-lang.org/learn/get-started).

 ## Changelog

 08.12.2022
 ---
 - Player and enemy stats

 07.12.2022
 ---
 - Introduced enemy spawning 
 - Added fadeout during the transition from overworld to combat and vice versa
 
 06.12.2022
 ---
 - Introduced debug with bevy-egui
 - Introduced simple tile map using text file
 - Moved all the tile entities under single parent (Map)
 - Added simple collision detection
 - Camera follow added to player
 - Introduced combat state
 - Hide player and map in combat state (at the moment combat state is activated when the player hits the grass which is ~ tiles), hitting space bar brings the state back
 - Timed encounters

 05.12.2022
 ---
 - Switched camera to 2D orthogonal
 - Added simple sprite (will use better sprite sheet in the future)
 - Moved the player stuff to separate plugin (PlayerPlugin)
 - Basic movement with (WSAD) and updated tile size

 02.12.2022
 ---
 - Created the project
 - Basic scene with simple plane and a cube
 - Simple light with enabled shadows
 
