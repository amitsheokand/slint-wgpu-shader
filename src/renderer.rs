use slint::wgpu_24::wgpu;
use std::time::Instant;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TimeUniform {
    time: f32,
    _padding: [f32; 3], // Padding for 16-byte alignment
}

pub struct ShaderRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    time_buffer: wgpu::Buffer,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    start_time: Instant,
    frozen_time: Option<f32>,
    total_paused_duration: f32,
}

impl ShaderRenderer {
    pub async fn new(shader_source: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create time uniform buffer
        let time_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Time Uniform Buffer"),
            size: std::mem::size_of::<TimeUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: 320,
                height: 200,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            device: device.clone(),
            queue: queue.clone(),
            pipeline,
            bind_group,
            time_buffer,
            texture,
            texture_view,
            start_time: Instant::now(),
            frozen_time: None,
            total_paused_duration: 0.0,
        }
    }

    pub fn render(&mut self, animation_enabled: bool) -> wgpu::Texture {
        // Update time uniform - freeze time if animation is disabled
        let current_time = if animation_enabled {
            // If we were frozen, unfreeze and resume
            if let Some(frozen) = self.frozen_time.take() {
                // Resume from the frozen time
                self.total_paused_duration = self.start_time.elapsed().as_secs_f32() - frozen;
            }
            self.start_time.elapsed().as_secs_f32() - self.total_paused_duration
        } else {
            // If not frozen yet, freeze at current time
            if self.frozen_time.is_none() {
                let current = self.start_time.elapsed().as_secs_f32() - self.total_paused_duration;
                self.frozen_time = Some(current);
            }
            // Return the frozen time
            self.frozen_time.unwrap()
        };

        let time_uniform = TimeUniform {
            time: current_time,
            _padding: [0.0; 3],
        };

        self.queue
            .write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&[time_uniform]));

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.texture.clone()
    }
}

pub struct AnimatedShaderManager {
    renderers: Vec<ShaderRenderer>,
}

impl AnimatedShaderManager {
    pub async fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        use crate::shaders::*;

        let renderers = vec![
            ShaderRenderer::new(RAINBOW_SHADER, device, queue).await,
            ShaderRenderer::new(WAVE_SHADER, device, queue).await,
            ShaderRenderer::new(NOISE_SHADER, device, queue).await,
            ShaderRenderer::new(GRADIENT_SHADER, device, queue).await,
        ];

        Self { renderers }
    }

    pub fn update_and_render(
        &mut self,
        animation_enabled: bool,
    ) -> (slint::Image, slint::Image, slint::Image, slint::Image) {
        let textures: Vec<_> = self
            .renderers
            .iter_mut()
            .map(|r| r.render(animation_enabled))
            .collect();

        (
            slint::Image::try_from(textures[0].clone()).unwrap(),
            slint::Image::try_from(textures[1].clone()).unwrap(),
            slint::Image::try_from(textures[2].clone()).unwrap(),
            slint::Image::try_from(textures[3].clone()).unwrap(),
        )
    }
}

pub fn setup_shader_textures(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (slint::Image, slint::Image, slint::Image, slint::Image) {
    let mut manager = pollster::block_on(AnimatedShaderManager::new(device, queue));
    manager.update_and_render(true) // Start with animation enabled
}
