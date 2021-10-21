# vulkanofun

Learning project for the Vulkano graphics crate.

## Build instructions

Please see https://github.com/vulkano-rs/vulkano/tree/master#setup-and-troubleshooting . `shaderc` will be the most annoying part to deal with.

On my machine, I followed the recommended setup for `windows-msvc`. As far as I can tell, it's appropriate to modify the directions by installing Python 3 with `mingw-w64-x86_64-python`, instead of Python 2 with `mingw-w64-x86_64-python2`, if having ancient Python 2 versions on your computer makes you feel icky. (Python's needed for Ninja, which is needed to build `shaderc` on msvc.)

By "the msys2 mingw64 binary path" they mean `C:\msys64\mingw64\bin`. I used the Windows graphical "edit environment variables" dialog to do this. You can use the `where` Windows command in `cmd` to verify that various programs inside that folder are first on the PATH.

## Learning instructions

The official Vulkano guide is [here](https://vulkano.rs/guide/introduction), but it is actually out of date and there have been numerous breaking changes.

* `Instance::new` now takes four parameters instead of three.
	* `use vulkano::instance::Version` then insert `Version::V1_0` or `Version::V1_1` as the second parameter.
* `PhysicalDevice` is in `vulkano::device::physical::PhysicalDevice` instead of under `::instance`.
* `AutoCommandBufferBuilder::new` has been renamed to `primary` and takes a `CommandBufferUsage` parameter.
	* use `OneTimeSubmit`
* ~~Executing buffers feels kinda different somehow~~ dont think so actually

There is a `vulkano-examples` repository but it is [known to be very out of date](https://github.com/vulkano-rs/vulkano/issues/1698#issuecomment-915304205). The intention was to update `vulkano-examples` on every major Vulkano release but that has not happened very often and it's out-of-date again. The best place to find Vulkano examples is in the `examples` folder of the regular Vulkano repository.

Many of the vulkano examples create a window with `winit` to draw realtime graphics in. This is very complicated under Vulkan and there's actually a lot you can do without creating a window at all (listing device information, toy buffer-copies, compute shaders, ...). The guide starts out by allocating and copying buffers, then doing compute shaders, and only *then* bothering to create a window. *(It might be a good idea to contribute more examples along that line)*