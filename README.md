## RL Arena Collision Dumper

[![Build and release](https://github.com/VirxEC/rl_arena_collision_dumper/actions/workflows/main.yml/badge.svg)](https://github.com/VirxEC/rl_arena_collision_dumper/actions/workflows/main.yml)

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

An Rocket League arena collision dumper for [RocketSim](https://github.com/ZealanL/RocketSim) that works on both Windows and Linux and works with the game closed.

### Disclaimer

When loaded into RocketSim, it will give a warning about the hashes not matching.

You can rest asured that the generated files work perfectly and all RocketSim optimizations work perfectly.

### Usage

In order for this collision dumper, you will need a few things:

 - A copy of the [umodel](https://www.gildor.org/en/projects/umodel) executable in the same folder as the collision dumper
   - ***Linux users*** may want to [download from the GitHub](https://github.com/gildor2/UEViewer#building-the-source-code) and compile it for themselves for compatibility reasons
 - Have the **ABSOLUTE PATH** to your `rocketleague/TAGame/CookedPCConsole` file at the ready - e.x. `C:/Program Files/Epic Games/rocketleague/TAGame/CookedPCConsole`
   - Put the path inside the file `assets.path` and the collision dumper will read it
   - If you run the collision dumper from the CLI, you will be prompted to enter the path if it can't be found

