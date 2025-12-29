use crate::renderer::depth_texture::DepthTexture;
use crate::renderer::fps::FpsCounter;
use crate::renderer::msaa_texture::MultisampledTexture;
use crate::wgpu_surface::WgpuSurface;
use std::iter;
use wgpu::{CompareFunction, DepthStencilState, SurfaceError, TextureFormat};
use wgpu_text::glyph_brush::ab_glyph::FontRef;
use wgpu_text::glyph_brush::{OwnedSection, OwnedText};
use wgpu_text::{BrushBuilder, TextBrush};

mod depth_texture;
mod fps;
mod msaa_texture;

pub struct WgpuRenderer {
    depth_texture: DepthTexture,
    msaa_texture: MultisampledTexture,
    canvas: Box<dyn WgpuSurface>,
    pub just_counter: i32,
    fps_counter: FpsCounter<120>,
    text_brush: TextBrush<FontRef<'static>>,
}

impl WgpuRenderer {
    pub async fn new(canvas: Box<dyn WgpuSurface>) -> anyhow::Result<WgpuRenderer> {
        let device = canvas.device();
        let config = canvas.config();

        let depth_texture = DepthTexture::new(&device, config.width, config.height);
        let msaa_texture =
            MultisampledTexture::new(device, config.width, config.height, config.format);
        let depth_state = DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: false,
            depth_compare: CompareFunction::Less,
            stencil: Default::default(),
            bias: Default::default(),
        };
        let multisample_state = wgpu::MultisampleState {
            count: MultisampledTexture::SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let fps_counter = FpsCounter::new();

        let text_brush = BrushBuilder::using_font_bytes(include_bytes!("font.ttf"))?
            .with_depth_stencil(Some(depth_state))
            .with_multisample(multisample_state)
            .build(device, config.width, config.height, config.format);

        Ok(Self {
            depth_texture,
            msaa_texture,
            canvas,
            just_counter: 0,
            fps_counter,
            text_brush,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.canvas.on_resize();
            let config = self.canvas.config();
            let device = self.canvas.device();
            let queue = self.canvas.queue();

            self.depth_texture = DepthTexture::new(&device, config.width, config.height);
            self.msaa_texture =
                MultisampledTexture::new(device, config.width, config.height, config.format);

            self.text_brush
                .resize_view(width as f32, height as f32, queue);
        }
    }

    pub fn update(&mut self) {
        let queue = self.canvas.queue();
        let device = self.canvas.device();
        let current_fps = format!("{:.1}", self.fps_counter.update());

        let mut text_section = OwnedSection::default().with_screen_position((130f32, 50f32));
        text_section
            .text
            .push(OwnedText::new(current_fps.as_str()).with_scale(60.0));
        text_section.text.push(
            OwnedText::new(format!("\nCounter: {:?}", self.just_counter).as_str()).with_scale(70.0),
        );

        self.text_brush
            .queue(&device, &queue, [&text_section])
            .unwrap();
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        self.canvas.on_pre_render();

        let output = self.canvas.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let device = self.canvas.device();
        let queue = self.canvas.queue();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.msaa_texture.view,
                    resolve_target: Some(&view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.741,
                            b: 0.961,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            self.text_brush.draw(&mut render_pass)
        }

        queue.submit(iter::once(encoder.finish()));
        output.present();

        self.canvas.on_post_render();

        Ok(())
    }
}
