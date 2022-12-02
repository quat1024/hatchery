# Hatchery

Monorepo for assorted projects in varying stages of completion.

## Projects

* `vdf/`: A VDF parser. Not complete.
* `tiny/`: A tinyv1 parser. Not complete.
* `wgpufun/`: Learning project for WebGPU graphics. Probably only compiles on Windows (Not built atm)
* `i-am-very-good-at-mastermind`: experiments with the four-letter-word mastermind game "mm" someone made
* `vulkanofun/`: Learning project for Vulkano graphics (Not built atm)
* `advent2022`: advent of code 2022 hell zone

## Compiling

**Currently wgpufun is not included in the cargo workspace so this isnt important**

`shaderc` is a hard crate to compile (it either needs cmake or precompiled libraries)

On Windows use the `-msvc` toolchain or linking will take 10000 years for some reason