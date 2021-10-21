# vulkanofun

Learning project for the Vulkano graphics crate.

## Build instructions

Please see https://github.com/vulkano-rs/vulkano/tree/master#setup-and-troubleshooting . `shaderc` will be the most annoying part to deal with.

On my machine, I followed the recommended setup for `windows-msvc`. As far as I can tell, it's appropriate to modify the directions by installing Python 3 with `mingw-w64-x86_64-python`, instead of Python 2 with `mingw-w64-x86_64-python2`, if having ancient Python 2 versions on your computer makes you feel icky. (Python's needed for Ninja, which is needed to build `shaderc` on msvc.)

By "the msys2 mingw64 binary path" they mean `C:\msys64\mingw64\bin`. I used the Windows graphical "edit environment variables" dialog to do this. You can use the `where` Windows command in `cmd` to verify that various programs inside that folder are first on the PATH.

## Learning instructions

The official Vulkano guide is [here](https://vulkano.rs/guide/introduction), but it is actually out of date and there have been numerous breaking changes. I'll list the ones I came across.

* `Instance::new` now takes four parameters instead of three.
	* `use vulkano::instance::Version` then insert `Version::V1_0` or `Version::V1_1` as the second parameter.
* `PhysicalDevice` is in `vulkano::device::physical::PhysicalDevice` instead of under `::instance`.
* `AutoCommandBufferBuilder::new` has been renamed to `primary` and takes a `CommandBufferUsage` parameter.
	* use `OneTimeSubmit`
* ~~Executing buffers feels kinda different somehow~~ dont think so actually