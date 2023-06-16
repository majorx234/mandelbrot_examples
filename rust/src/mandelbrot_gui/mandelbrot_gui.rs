use std::collections::VecDeque;

use eframe::egui;
use eframe::egui::{Color32, TextureHandle};
use egui::*;
use image::codecs::webp::vp8::Vp8Decoder;
use mandelbrot_utils::calc::calculate_mandelbrot;
use mandelbrot_utils::mandelbrot_handler::MandelbrotHandler;

struct Zoom {
    pub x_pos: f64,
    pub y_pos: f64,
    pub range_x: f64,
    pub range_y: f64,
}

struct MandelbrotWidget {
    pub tex_mngr: TextureManager,
    pub texture_id: Option<(egui::Vec2, egui::TextureId)>,
    pub texture_response: Option<Response>,
    pub zoom: Zoom,
    pub changed: bool,
}

impl MandelbrotWidget {
    fn new(mandelbrot_data: Vec<u8>) -> Self {
        let mandelbrot_data_rgba = mandelbrot_data
            .into_iter()
            .map(|x| egui::epaint::Color32::from_gray(x))
            .collect::<Vec<Color32>>();
        Self {
            tex_mngr: TextureManager(mandelbrot_data_rgba, None),
            texture_id: None,
            texture_response: None,
            zoom: Zoom {
                x_pos: -0.5,
                y_pos: 0.0,
                range_x: 1.0,
                range_y: 1.0,
            },
            changed: false,
        }
    }
}

impl Default for MandelbrotWidget {
    fn default() -> Self {
        Self {
            tex_mngr: TextureManager(vec![Color32::from_rgb(255, 0, 255); 800 * 800], None),
            texture_id: None,
            texture_response: None,
            zoom: Zoom {
                x_pos: -0.5,
                y_pos: 0.0,
                range_x: 1.0,
                range_y: 1.0,
            },
            changed: false,
        }
    }
}

impl MandelbrotWidget {
    fn ui(&mut self, ui: &mut Ui, mandelbrot_data: Option<Vec<Vec<u8>>>) {
        if let Some(mandelbrot_data) = mandelbrot_data {
            self.set_values(ui.ctx(), mandelbrot_data);
        } else {
            self.tex_mngr.repaint_mandelbrot_texture(ui.ctx(), 800, 800);
            if let Some(ref texture) = self.tex_mngr.1 {
                self.texture_id = Some((egui::Vec2::new(800.0, 800.0), texture.into()));
            }
        }

        if let Some((size, texture_id)) = self.texture_id {
            let texture_response =
                ui.add(egui::Image::new(texture_id, size))
                    .interact(egui::Sense {
                        click: true,
                        drag: true,
                        focusable: true,
                    });
            self.texture_response = Some(texture_response);
            ui.ctx().request_repaint();
        }
    }

    fn set_values(&mut self, ctx: &egui::Context, mandelbrot_img_new: Vec<Vec<u8>>) {
        self.tex_mngr
            .update_mandelbrot_texture(ctx, mandelbrot_img_new, 800, 800);
        if let Some(ref texture) = self.tex_mngr.1 {
            self.texture_id = Some((egui::Vec2::new(800.0, 800.0), texture.into()));
        }
        //ToDo: change resolution
    }
}

//#[derive(Default)]
struct TextureManager(Vec<egui::epaint::Color32>, Option<TextureHandle>);

impl TextureManager {
    pub fn update_mandelbrot_texture(
        &mut self,
        ctx: &egui::Context,
        mandelbrot_img_new: Vec<Vec<u8>>,
        width: usize,
        height: usize,
    ) {
        let mandelbrot_img = mandelbrot_img_new
            .into_iter()
            .flatten()
            .map(|x| egui::epaint::Color32::from_gray(x))
            .collect::<Vec<Color32>>();
        self.0 = mandelbrot_img;
        let pixels: Vec<egui::epaint::Color32> = self.0.clone();
        self.1 = Some(ctx.load_texture(
            "color_test_gradient",
            egui::ColorImage {
                size: [width, height],
                pixels,
            },
        ));
    }
    pub fn repaint_mandelbrot_texture(&mut self, ctx: &egui::Context, width: usize, height: usize) {
        let pixels: Vec<egui::epaint::Color32> = self.0.clone();
        self.1 = Some(ctx.load_texture(
            "color_test_gradient",
            egui::ColorImage {
                size: [width, height],
                pixels,
            },
        ));
    }
}

pub struct MandelbrotGui {
    mandelbrot_handler: Option<MandelbrotHandler>,
    mandelbrot: MandelbrotWidget,
}

impl MandelbrotGui {
    pub fn new(mandelbrot_data: Vec<u8>) -> Self {
        Self {
            mandelbrot_handler: None,
            mandelbrot: MandelbrotWidget::new(mandelbrot_data),
        }
    }
}
impl Default for MandelbrotGui {
    fn default() -> Self {
        Self {
            mandelbrot_handler: None,
            mandelbrot: MandelbrotWidget::default(),
        }
    }
}
impl eframe::App for MandelbrotGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut test_vec: Option<Vec<Vec<u8>>> = if self.mandelbrot.changed == false {
                None
            } else {
                let mut buf_vec: Vec<Vec<u8>> = Vec::new();
                let mut buf: Vec<u8> = vec![0; 800 * 800];
                let zoom = &self.mandelbrot.zoom;
                let x_min = zoom.x_pos - zoom.range_x;
                let x_max = zoom.x_pos + zoom.range_x;
                let y_min = zoom.y_pos - zoom.range_y;
                let y_max = zoom.y_pos + zoom.range_y;
                calculate_mandelbrot(&mut buf[..], x_min, x_max, y_min, y_max, 800, 800);
                for buf_row in buf.windows(800).step_by(800) {
                    buf_vec.push(buf_row.to_vec());
                }
                self.mandelbrot.changed = false;
                Some(buf_vec)
            };
            // ToDo: async Mandelbrot creation through handler
            if let Some(mandelbrot_handler) = &mut self.mandelbrot_handler {
                if let Some(new_mandelbrot) = mandelbrot_handler.get_mandelbrot() {
                    test_vec = Some(new_mandelbrot);
                }
            }
            self.mandelbrot.ui(ui, test_vec);
        });
        if let Some(ref texture_response) = self.mandelbrot.texture_response {
            if texture_response.clicked() {
                if let Some(clicked_pos) = texture_response.interact_pointer_pos() {
                    println!(
                        "clicked on mandelbrot x: {} y: {}",
                        clicked_pos.x, clicked_pos.y
                    );
                    let x_shift_value = 10.0;
                    let y_shift_value = 10.0;
                    // ToDo: scaling and correct offset
                    self.mandelbrot.zoom.x_pos = self.mandelbrot.zoom.range_x
                        * ((clicked_pos.x - x_shift_value) / 800.0) as f64;
                    self.mandelbrot.zoom.y_pos = self.mandelbrot.zoom.range_x
                        * ((clicked_pos.y - y_shift_value) / 800.0) as f64;
                    self.mandelbrot.zoom.range_x = self.mandelbrot.zoom.range_x / 2.0;
                    self.mandelbrot.zoom.range_y = self.mandelbrot.zoom.range_y / 2.0;
                    self.mandelbrot.changed = true;
                } else {
                    println!("clicked on mandelbrot");
                }
            }
        }
    }
}
