# Hatchery

Monorepo for assorted projects in varying stages of completion.

## Projects

* `vdf/`: A VDF parser. Not complete.
* `tiny/`: A tinyv1 parser. Not complete.
* `wgpufun/`: Learning project for WebGPU graphics. Probably only compiles on Windows (Not built atm)
* `i-am-very-good-at-mastermind`: experiments with the four-letter-word mastermind game "mm" someone made
* `vulkanofun/`: Learning project for Vulkano graphics.

## Compiling

**Currently wgpufun is not included in the cargo workspace so this isnt important**

`shaderc` is a hard crate to compile (it either needs cmake or precompiled libraries)

On Windows use the `-msvc` toolchain or linking will take 10000 years for some reason

## License bullshit

[EUPL 1.2](https://spdx.org/licenses/EUPL-1.2.html).  
It's basically the AGPL but not three novels long, and lol imagine thinking the FSF is relevant in 2021 after they shot themselves in the foot.