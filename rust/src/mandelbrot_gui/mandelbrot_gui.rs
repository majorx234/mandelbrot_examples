use eframe::egui;
use eframe::egui::{Color32, TextureHandle};
use egui::*;
use mandelbrot_utils::mandelbrot_handler::MandelbrotHandler;

struct MandelbrotWidget {
    pub tex_mngr: TextureManager,
    pub texture_id: Option<(egui::Vec2, egui::TextureId)>,
    pub texture_response: Option<Response>,
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
        }
    }
}

impl Default for MandelbrotWidget {
    fn default() -> Self {
        Self {
            tex_mngr: TextureManager(vec![Color32::from_rgb(255, 0, 255); 800 * 800], None),
            texture_id: None,
            texture_response: None,
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
            let mut test_vec: Option<Vec<Vec<u8>>> = None;
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
                } else {
                    println!("clicked on mandelbrot");
                }
            }
        }
    }
}
