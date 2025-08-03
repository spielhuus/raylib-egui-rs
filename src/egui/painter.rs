use crate::color::Color;
use crate::math::Rectangle;
use crate::raylib;
use crate::rlgl;
use egui::Mesh;
use egui::Rect;
use egui::epaint::Primitive;
use egui::epaint::{ImageData, TextureId};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Painter {
    textures: HashMap<TextureId, raylib::Texture2D>,
    material: raylib::Material,
}
impl Painter {
    pub fn new() -> Self {
        Self {
            textures: Default::default(),
            material: raylib::LoadMaterialDefault(),
        }
    }
    pub fn destroy(&mut self) {
        for tex in &self.textures {
            raylib::UnloadTexture(*tex.1);
        }
        raylib::UnloadMaterial(self.material);
    }

    pub fn paint_and_update_textures(
        &mut self,
        pixels_per_point: f32,
        clipped_primitives: &[egui::ClippedPrimitive],
        textures_delta: &egui::TexturesDelta,
    ) {
        for (id, image_delta) in &textures_delta.set {
            self.set_texture(*id, image_delta);
        }

        self.paint_primitives(pixels_per_point, clipped_primitives);

        for &id in &textures_delta.free {
            println!("free texture!");
            self.free_texture(id);
        }
    }

    pub fn paint_primitives(
        &mut self,
        pixels_per_point: f32,
        clipped_primitives: &[egui::ClippedPrimitive],
    ) {
        for egui::ClippedPrimitive {
            clip_rect,
            primitive,
        } in clipped_primitives
        {
            match primitive {
                Primitive::Mesh(mesh) => {
                    self.paint_mesh(pixels_per_point, clip_rect, mesh);
                }
                Primitive::Callback(_) => {
                    panic!("Custom rendering callbacks are not implemented in egui_glium");
                }
            }
        }
    }

    // A new helper function to convert egui mesh to a raylib mesh and draw it
    fn draw_egui_mesh(&mut self, egui_mesh: &egui::Mesh) {
        if egui_mesh.indices.is_empty() || egui_mesh.vertices.is_empty() {
            return;
        }

        let texture = self
            .textures
            .get(&egui_mesh.texture_id)
            .expect("Texture not found for mesh");

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(egui_mesh.vertices.len());
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(egui_mesh.vertices.len());
        let mut colors: Vec<[u8; 4]> = Vec::with_capacity(egui_mesh.vertices.len());
        let indices: Vec<u16> = egui_mesh.indices.iter().map(|&i| i as u16).collect();

        for v in &egui_mesh.vertices {
            positions.push([v.pos.x, v.pos.y, 0.0]);
            uvs.push([v.uv.x, v.uv.y]);
            colors.push(v.color.to_srgba_unmultiplied());
        }

        let mut vbo_id_array = [0u32; 7];
        let mut mesh = raylib::Mesh {
            vertexCount: egui_mesh.vertices.len() as i32,
            triangleCount: (egui_mesh.indices.len() / 3) as i32,
            vertices: positions.as_mut_ptr() as *mut f32,
            texcoords: uvs.as_mut_ptr() as *mut f32,
            colors: colors.as_mut_ptr() as *mut u8,
            indices: indices.as_ptr() as *mut u16,
            vboId: vbo_id_array.as_mut_ptr(),
            ..Default::default()
        };

        raylib::UploadMesh(&mut mesh, true);

        std::mem::forget(positions);
        std::mem::forget(uvs);
        std::mem::forget(colors);
        std::mem::forget(indices);

        raylib::SetMaterialTexture(&mut self.material, 0, *texture);

        raylib::DrawMesh(mesh, self.material, raylib::MatrixIdentity());
        raylib::UnloadMesh(mesh);
    }

    // Your existing paint_mesh function gets much simpler
    fn paint_mesh(&mut self, pixels_per_point: f32, clip_rect: &Rect, mesh: &Mesh) {
        debug_assert!(mesh.is_valid());

        rlgl::rlDisableBackfaceCulling();
        rlgl::rlDisableDepthTest();

        let clip_min_x = (pixels_per_point * clip_rect.min.x).round() as i32;
        let clip_min_y = (pixels_per_point * clip_rect.min.y).round() as i32;
        let clip_max_x = (pixels_per_point * clip_rect.max.x).round() as i32;
        let clip_max_y = (pixels_per_point * clip_rect.max.y).round() as i32;

        raylib::BeginScissorMode(clip_min_x, clip_min_y, clip_max_x, clip_max_y);
        self.draw_egui_mesh(mesh);
        raylib::EndScissorMode();
    }

    pub fn set_texture(&mut self, tex_id: egui::TextureId, delta: &egui::epaint::ImageDelta) {
        let size = delta.image.size();

        let pixels = match &delta.image {
            ImageData::Color(color_data) => color_data
                .pixels
                .iter()
                .flat_map(|c| c.to_srgba_unmultiplied())
                .collect::<Vec<u8>>(),
        };

        let image = raylib::Image {
            data: pixels.as_ptr() as *mut std::ffi::c_void,
            width: size[0] as i32,
            height: size[1] as i32,
            mipmaps: 1,
            format: rlgl::rlPixelFormat::RL_PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
        };

        if let Some(pos) = delta.pos {
            if let Some(user_texture) = self.textures.get_mut(&tex_id) {
                let mut orig = raylib::LoadImageFromTexture(*user_texture);
                raylib::ImageDraw(
                    &mut orig,
                    image,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: size[0] as f32,
                        height: size[1] as f32,
                    },
                    Rectangle {
                        x: pos[0] as f32,
                        y: pos[1] as f32,
                        width: size[0] as f32,
                        height: size[1] as f32,
                    },
                    Color::WHITE,
                );

                let new_texture = raylib::LoadTextureFromImage(orig);
                self.textures.insert(tex_id, new_texture);

                // //TODO
                // raylib::ExportImage(orig, &format!("file-{}.png", new_texture.id));
            }
        } else {
            let tex = raylib::LoadTextureFromImage(image);
            self.textures.insert(tex_id, tex);
            //TODO
            // raylib::ExportImage(image, &format!("file-{}.png", tex.id));
        }
    }

    pub fn free_texture(&mut self, tex_id: egui::TextureId) {
        self.textures.remove(&tex_id);
    }
}
