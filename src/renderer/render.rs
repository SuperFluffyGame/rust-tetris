use super::data::Vertex;

impl super::Renderer {
    pub fn render(&self, vertex_data: &Vec<Vertex>) -> Result<(), wgpu::SurfaceError> {
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertex_data));

        let output_texture = self.surface.get_current_texture()?;
        let output_view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command Encoder"),
                });

        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_bind_group(0, &self.bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..vertex_data.len() as u32, 0..1);

        drop(render_pass);
        self.queue.submit(std::iter::once(command_encoder.finish()));

        output_texture.present();
        Ok(())
    }
}
