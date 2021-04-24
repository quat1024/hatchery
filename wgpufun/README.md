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
* Put it... somewhere (i have it in `g:/dev/Path/lib/`)
* Export that path to the environment variable `SHADERC_LIB_DIR` so the shaderc crate's build script can pick it up
	* i put this in my bashrc

## Uhm

~~Also the buildscript doesn't work unless you put shaderc_shared.dll in `/target/debug` idk why~~

~~Is there some standard search path for windows libraries~~ I put my libraries folder on my PATH and it seems to work

### random things

Some thing i want to investigate is drawing lines and strokes efficiently. I have a Processing background where i'm used to calling "strokeWidth" and instantly having nice chunky lines.

Processing's OpenGL shaders: https://github.com/processing/processing/tree/master/core/src/processing/opengl/shaders obviously not useful to just copy, but might be handy for emulating stroke effects. not sure how useful they will be or if p5 still implements stroke with magic gl functions

looks like it draws the line in software? https://github.com/processing/processing/blob/master/core/src/processing/opengl/LineStroker.java

Mapbox on drawing lines in opengl: https://blog.mapbox.com/drawing-antialiased-lines-with-opengl-8766f34192dc

Dashed lines http://jcgt.org/published/0002/02/08/paper.pdf

tesselating polylines on the cpu in C++: https://github.com/CrushedPixel/Polyline2D (oh, hey crushedpixel)

drawing lines with a focus on Javascript https://mattdesl.svbtle.com/drawing-lines-is-hard
* mentions expanding line points out in a vertex shader