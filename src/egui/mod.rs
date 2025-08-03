mod input;
mod painter;
pub use input::Input;
pub use painter::Painter;

#[derive(Default)]
pub struct EguiRaylib {
    pub painter: Painter,
    pub input: Input,
    shapes: Vec<egui::epaint::ClippedShape>,
    textures_delta: egui::TexturesDelta,
    ctx: egui::Context,
}

impl EguiRaylib {
    pub fn new() -> Self {
        Self {
            painter: Painter::new(),
            shapes: Default::default(),
            textures_delta: Default::default(),
            input: Input::default(),
            ctx: egui::Context::default(),
        }
    }

    pub fn run(&mut self, run_ui: impl FnMut(&egui::Context)) {
        self.input.update();
        let egui::FullOutput {
            textures_delta,
            shapes,
            ..
        } = self.ctx.run(self.input.take(), run_ui);

        self.shapes = shapes;
        self.textures_delta.append(textures_delta);
    }

    pub fn paint(&mut self) {
        let shapes = std::mem::take(&mut self.shapes);
        let textures_delta = std::mem::take(&mut self.textures_delta);
        let clipped_primitives = self.ctx.tessellate(shapes, self.ctx.pixels_per_point());
        self.painter.paint_and_update_textures(
            self.ctx.pixels_per_point(),
            &clipped_primitives,
            &textures_delta,
        );
    }
}
