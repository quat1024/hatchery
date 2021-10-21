use std::sync::Arc;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBufferUsage;
use vulkano::command_buffer::PrimaryCommandBuffer;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::physical::QueueFamily;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::device::Queue;
use vulkano::device::QueuesIter;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::Version;
use vulkano::sync::GpuFuture;

// (Vulkano uses a ton of Arc<_>s and it's useful to be consistent with my calls to .clone(), even if there's technically no need)
#[allow(clippy::redundant_clone)]
pub fn main() {
	// Hi. This is my little Vulkano playground.
	// The current version of the library is 0.26.0.
	// I've written the code with explicit type annotations where appropriate so you know what you're looking at.

	// First I need to create a Vulkan instance. This is the "entry point" to the library.
	let inst: Arc<Instance> = Instance::new(
		// Application info struct.
		// Graphics card vendors like to do application-specific hacks. In the past (directx/opengl)
		// the graphics driver would often query the executable path or the command-line to determine
		// which application is running. This sucks. (google "mojang tricks intel drivers" lol)
		// So, this is a chance for *you* to tell the driver about *your* application.
		// You don't need to do this.
		None,
		// The version of Vulkan you are requesting out of the driver.
		// This is a nice "Version" struct in Vulkano.
		// In Vulkan proper it's a u32 bitfield with sections cordoned off for major/minor/patch.
		Version::V1_1,
		// Vulkan instance extensions.
		// Currently not using any, although note that things like "drawing images to a monitor"
		// are Vulkan extensions.
		&InstanceExtensions::none(),
		// A list of additional layers for the Vulkan layers system, if you use that.
		// You can learn more about layers here: https://renderdoc.org/vulkan-layer-guide.html
		// Basically, "layers" are something you register with the graphics card driver.
		// Each layer can intercept any Vulkan function call, and do whatever it wants.
		// You might have heard of the "Vulkan validation layers": these do a bunch of
		// software runtime checks that the Vulkan API is being used correctly.
		// The RenderDoc graphics debugger uses a layer to log Vulkan function calls, too.
		// ...So anyway, I'm not requesting any layers.
		None,
	)
	.expect("failed to make instance");

	// Print the instance's API version. I requested version 1.1, and I hope I see that one,
	// but if you request a too-new version of Vulkan, you might not get what you want.
	// You get the nice "Version" struct out of this method again, btw.
	println!("api version: {}", inst.api_version());

	// Print off physical devices. A physical device is a graphics card or similar.
	// On my machine this prints "available device: NVIDIA GeForce GTX 1650 SUPER" because that's
	// the graphics card plugged into my pci express slot. There might be more than one
	// device, like an "integrated graphics" scenario.
	for physical_device in PhysicalDevice::enumerate(&inst) {
		println!("available device: {}", physical_device.properties().device_name)
	}

	// Okay, next I need to choose a physical device.
	// PhysicalDevice.properties() contains a whole load of things to examine, so in the event
	// of multiple graphics cards existing, you could look at those and pick the best one for your
	// application, such as whichever one has the most available memory.
	// Here, I'm not gonna bother with that. I'm just going to pick the first one.
	let physical_device: PhysicalDevice = PhysicalDevice::enumerate(&inst).next().expect("Expected at least one device");
	println!("selected device: {}", physical_device.properties().device_name);

	// Okay, next I need to open a channel of communication with this physical device. (Annoyingly, this structure is also called a "device".)\
	// Before I do that, I will introduce a concept of "queue families".
	// A queue family is a specific pipeline on the graphics card.
	// Vulkano's guide compares a queue to a CPU thread. Not all queues are the same, however.
	// Here are the queue *families* on my graphics card: http://vulkan.gpuinfo.org/displayreport.php?id=12531#queuefamilies
	for family in physical_device.queue_families() {
		// Each queue family may contain more than one queue. Else they wouldn't be called families, I guess.
		// "All queues of one family have the same characteristics."
		println!("queue family {} contains {} queues", family.id(), family.queues_count())
	}

	// In order to create a device, I have to ask which queue families I want to use.
	// Here, I will find the first queue family on this graphics card that supports the compute pipeline.
	let queue_family: QueueFamily = physical_device
		.queue_families()
		.find(|&q| q.supports_compute()) // (rust iterator function)
		.expect("Expected at least one queue family that supports compute.");

	// Next I will create a device. This also gives me an iterator of queues; the members of the family I requested.
	// (According to the Vulkano source, here you get a "QueuesIter" instead of a plain "impl Iter" type because of a Rust shortcoming.)
	let (device, mut queues): (Arc<Device>, QueuesIter) = {
		Device::new(
			physical_device,
			// Features you can request out of the device.
			// A list of features is here: https://docs.rs/vulkano/0.26.0/vulkano/device/struct.Features.html
			// A list of features on my own graphics card: http://vulkan.gpuinfo.org/displayreport.php?id=12531#features
			// Vulkano will always internally request the "robust buffer access" feature for itself.
			&Features::none(),
			// Device extensions.
			// I'm not sure what makes these different from "features"...
			// A list of extensions is here: https://docs.rs/vulkano/0.26.0/vulkano/device/struct.DeviceExtensions.html
			// I guess these are the extensions on my card: http://vulkan.gpuinfo.org/displayreport.php?id=12531#extensions
			&DeviceExtensions::none(),
			// And here are the queues I want to see. I only want a queue from the one family I asked for.
			// The number is for "priority": if I was requesting more queues, I could *hint* to the driver
			// which ones are most important to my application, and it *might* allot that one some more processing time.
			// (Exactly what this does is up the vendor.)
			[(queue_family, 0.5)],
		)
		.expect("Failed to create device and queue list")
	};

	// Above, I only asked for a queue from one queue family, so I'm going to pull out the queue I requested now.
	let queue: Arc<Queue> = queues.next().expect("Queue families contain at least one queue");

	println!("created device and queue");

	// Now I will create a buffer. Vulkano provides a number of different buffer types for different situations.
	// - `UnsafeBuffer` is the lowest level buffer primitive, and pretty much directly corresponds to a Vulkan buffer.
	//      The other buffers wrap it.
	// - `DeviceLocalBuffer` is stored on the GPU, so accessing it from the GPU is fast, but you can't access it from the CPU at all.
	//      Documentation states this is great for "scratch buffers" in fancy shader setups.
	// - `ImmutableBuffer` is stored on the GPU and allows exactly one write operation - one CPU upload, or one buffer-to-buffer copy.
	//      The buffer cannot be changed after it is created.
	// - `CpuAccessibleBuffer` is stored on the CPU, but still accessible from the GPU.
	//      Reads and writes are performed with a lock, and acquiring the lock may fail if the GPU is using the buffer.
	//      (Documentation states that this might be removed in favor of CpuBufferPool in the "far future",
	//      but currently this is the only buffer you can *read* from on the CPU.)
	// - `CpuBufferPool` is... I dunno exactly what it is lol.
	//      Looks like a successor of CpuAccessibleBuffer with similar performance characteristics.
	//      Documentation recommends this buffer as a good first option for uploading/downloading data really often.
	//      (Probably wrong though, since you can't read from it CPU side.)
	//
	// These impl Drop as appropriate, so dropping the Rust-side handle will also issue a command to free the buffer on the GPU.
	//
	// (This turbofish is not needed, I'm just calling out that they're typed Rust-side.)
	let src_buffer: Arc<_> = CpuAccessibleBuffer::<u32>::from_data(
		// Which device I'm allocating the buffer on.
		device.clone(),
		// Later I'm going to copy data out of this buffer, so I need to enable this BufferUsage mode.
		BufferUsage::transfer_source(),
		// "Host-cached memory". If "true", you can read (possibly incoherent) data out of the buffer while the GPU is writing to it.
		// This is probably best left "false" unless the buffer is under contention and memory-coherency problems are okay.
		// (I think.)
		false,
		// And here's the data I'm putting into the buffer.
		// Normally your buffers would probably be bigger than one single u32. :P
		69420,
	)
	.expect("failed to create source buffer");
	println!("allocated src_buffer, contents: {}", *src_buffer.read().expect("expect to be able to read src buffer"));

	// Okay, now I'll tell the GPU to do something.
	// As a first example (because this is a comparably easy operation to do), I'll copy data from one buffer to another.

	// First I'll make a destination buffer, using a CpuAccessibleBuffer again.
	let dst_buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::transfer_destination(), false, 0u32).expect("failed to create dest buffer");
	println!("allocated dst_buffer, contents: {}", *dst_buffer.read().expect("expect to be able to read dst buffer"));

	// The GPU accepts commands in "command buffers" (oh look another type of buffer) instead of immediately.
	// Vulkano's guide claims that OpenGL's lack-of-command-buffers really just means the buffers are hiding from you,
	// and just about every driver batches commands together internally, probably not in the way you want them to be batched.
	// So yeah, command buffers are a thing. I will allocate one of those, and add a "copy buffer" command into the buffer.
	let mut command_buffer_builder =
		AutoCommandBufferBuilder::primary(device.clone(), queue.family(), CommandBufferUsage::OneTimeSubmit).expect("failed to create buffer builder");

	command_buffer_builder
		// CpuAccessibleBuffers are like Arcs.
		.copy_buffer(src_buffer.clone(), dst_buffer.clone())
		// This operation might fail because of incorrect BufferUsages.
		// Maybe the GPU isn't allowed to copy the source buffer into the dest buffer.
		// Also, adding commands to buffers can always fail, in really exceptional circumstances like out-of-memory.
		.expect("failed to create buffer copy command");

	let command_buffer = command_buffer_builder
		.build()
		//This operation might fail if the builder is empty, or if there is no memory on the GPU.
		.expect("failed to build command buffer");

	// Finally, I will tell the GPU to execute the commands in this command buffer.
	// First I need to submit the buffer to the GPU.
	// .execute() is short for .execute_after(now(device)), which means the GPU will start processing it as soon as it receives it.
	// (I think you can also use the execute_after mechanism to cause two command buffers to execute one after the other, even if you
	// submitted them at nearly the same time.)
	let finished_handle = command_buffer.execute(queue.clone()).expect("failed to submit buffer");
	finished_handle
		// After executing this command, tell the GPU to signal a fence, then flush the command buffer.
		// This is a shortcut for "then_signal_fence().flush()".
		// I think `flush` is what actually uploads the command buffer.
		.then_signal_fence_and_flush()
		.expect("failed to flush command buffer")
		// Because of the signalled fence, it's possible to detect when the GPU is finished executing my command buffer.
		// I'm interested in the results now, so block the CPU thread until the GPU is done. (The parameter is an optional timeout.)
		.wait(None)
		.expect("failed to wait for command execution");
	println!("submitted src -> dst copy command");

	// Finally, I'll check the destination buffer to see if anything has been copied into there.
	// Now that the GPU is done using it, I can unlock it and read it.
	println!("dst_buffer contents: {}", *dst_buffer.read().expect("expect to be able to read the buffer"));
}
