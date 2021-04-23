use std::fs::{self};
use std::path::Path;

use anyhow::*;

fn main() {
	compile(Path::new("./shader_src"), &mut shaderc::Compiler::new().context("cant make shaderc").unwrap()).expect("problem walking file tree");
}

fn compile(path: &Path, compiler: &mut shaderc::Compiler) -> Result<()> {
	for entry in fs::read_dir(path)? {
		let entry = entry?;
		let meta = entry.metadata()?;
		if meta.is_dir() {
			compile(&entry.path(), compiler)?;
		} else {
			let result: Result<()> = {
				//tell cargo to rebuild shaders if *any* shader has changed (kinda crappy)
				println!("cargo:rerun-if-changed={}", entry.path().as_os_str().to_str().unwrap());

				//read the shader source code
				let src = fs::read_to_string(entry.path())?;

				//guess the stage based off the file extension
				let kind = match entry.path().extension().map(|s| s.to_str().unwrap()) {
					Some("vert") => shaderc::ShaderKind::Vertex,
					Some("frag") => shaderc::ShaderKind::Fragment,
					Some("comp") => shaderc::ShaderKind::Compute,
					Some(other) => bail!("unknown file extension '{}'", other),
					None => bail!("no file extension"),
				};

				//compile the shader into spir-v with naga
				let compiled = compiler.compile_into_spirv(&src, kind, &entry.path().to_string_lossy(), "main", None)?;

				//choose the output path "./shader_glsl/pog.vert -> ./shader_spv/pog.vert.spv"
				let mut out_path = Path::new("./assets/compiled_shaders").join(entry.path().strip_prefix("./shader_src")?);
				let mut out_ext = out_path.extension().unwrap().to_str().unwrap().to_string();
				out_ext.push_str(".spv");
				out_path.set_extension(out_ext);

				//write the output
				std::fs::write(out_path, compiled.as_binary_u8())?;

				Ok(())
			};

			result.with_context(|| format!("problem compiling {}", entry.path().to_string_lossy()))?
		}
	}

	Ok(())
}
