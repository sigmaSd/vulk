#[macro_export]
macro_rules! shader {
    ($code: literal) => {
        use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
        use vulkano::command_buffer::AutoCommandBufferBuilder;
        use vulkano::command_buffer::CommandBuffer;
        use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
        use vulkano::descriptor::PipelineLayoutAbstract;
        use vulkano::device::DeviceExtensions;
        use vulkano::device::Features;
        use vulkano::instance::Instance;
        use vulkano::instance::InstanceExtensions;
        use vulkano::instance::PhysicalDevice;
        use vulkano::pipeline::ComputePipeline;
        use vulkano::sync::GpuFuture;

        vulkano_shaders::shader! {
            ty: "compute",
            src: $code
        }

        fn gpu_compute<I, T>(data_iter: I) -> Vec<T>
        where
            I: ExactSizeIterator<Item = T>,
            T: Clone + vulkano::memory::Content + Send + 'static + Sync,
        {
            let instance = Instance::new(None, &InstanceExtensions::none(), None)
                .expect("failed to create instance");

            let physical = PhysicalDevice::enumerate(&instance)
                .next()
                .expect("no device available");
            let queue_family = physical
                .queue_families()
                .find(|&q| q.supports_graphics())
                .expect("couldn't find a graphical queue family");

            let (device, mut queues) = {
                Device::new(
                    physical,
                    &Features::none(),
                    &DeviceExtensions {
                        khr_storage_buffer_storage_class: true,
                        ..DeviceExtensions::none()
                    },
                    [(queue_family, 0.5)].iter().cloned(),
                )
                .expect("failed to create device")
            };

            let queue = queues.next().unwrap();

            let data_buffer = CpuAccessibleBuffer::from_iter(
                device.clone(),
                BufferUsage::all(),
                false,
                data_iter,
            )
            .expect("failed to create buffer");

            let shader = Shader::load(device.clone()).unwrap();
            let compute_pipeline = Arc::new(
                ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
                    .expect("failed to create compute pipeline"),
            );

            let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
            let set = Arc::new(
                PersistentDescriptorSet::start(layout.clone())
                    .add_buffer(data_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            let mut builder =
                AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
            builder
                .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ())
                .unwrap();
            let command_buffer = builder.build().unwrap();
            let finished = command_buffer.execute(queue.clone()).unwrap();
            finished
                .then_signal_fence_and_flush()
                .unwrap()
                .wait(None)
                .unwrap();
            let content = data_buffer.read().unwrap();

            content.to_vec()
        }
    };
}
