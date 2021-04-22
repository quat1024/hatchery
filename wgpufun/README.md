wgpufun
=======

This is a toy project for playing around with `wgpu`. Mainly following this guide https://sotrh.github.io/learn-wgpu/

**!! On windows use the msvc toolchain or your head will explode from horrendously slow link times !!**

## Compiling

Unfortunately I'm a stinky Windows user and I can't really build `shaderc` from source. Like I coulddddd but i dun wanna download cmake and stuff it'll be a huge mess

* Go to shaderc downloads on google's github https://github.com/google/shaderc#downloads
* Get the one for Windows
* Look in the zip and pull `install/bin/shaderc_shared.dll` out
	* For some reason it doesn't work with `install/lib/shaderc_shared.lib`... build script just doesn't find it
* Put it... somewhere (i have it in `g:/dev/lib/`)
* Export that path to the environment variable `SHADERC_LIB_DIR` so the shaderc crate's build script can pick it up

I'm not sure how to get this environment variable into whatever happens when you click "run" in vscode so uh, build it in the terminal once