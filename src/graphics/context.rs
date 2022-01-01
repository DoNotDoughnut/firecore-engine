use winit::{event_loop::EventLoop, window::WindowBuilder};

use crate::error::InitializeError;

pub(crate) struct GraphicsContext {
    pub window: WindowContext,
    pub wgpu: WgpuContext,
    pub defaults: GraphicsDefaults,
}

pub(crate) struct WindowContext {
    pub window: winit::window::Window,
}

pub(crate) struct WgpuContext {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GraphicsContext {
    pub fn new(builder: WindowBuilder) -> Result<(Self, EventLoop<()>), InitializeError> {
        let event_loop = EventLoop::new();

        let window = WindowContext {
            window: builder.build(&event_loop),
        };

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

        let surface = unsafe { instance.create_surface(&window.window) };

        let (adapter, device, queue) = pollster::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .ok_or(InitializeError::NoGraphicsAdapter)?;

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    },
                    None,
                )
                .await?;

            Ok((adapter, device, queue))
        });

        Ok((Self {
            window,
            wgpu: WgpuContext {
                surface,
                adapter,
                device,
                queue,
            },
        }, event_loop))
    }
}
