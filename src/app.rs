/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)] // Recalculate on startup
    grid_cols: usize,
    #[serde(skip)]
    grid_rows: usize,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            grid_cols: 1, // Will be recalculated
            grid_rows: 1, // Will be recalculated
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Right sidebar
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Right Sidebar");
                ui.separator();
                ui.label("Sidebar content here");
            });

        // Bottom bar
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(100.0)
            .show(ctx, |ui| {
                ui.heading("Bottom Bar");
                ui.separator();
                ui.label("Bottom bar content here");
            });

        // Central panel with letter grid
        egui::CentralPanel::default().show(ctx, |ui| {
            // Get the font and calculate letter size
            let font_id = egui::TextStyle::Button.resolve(ui.style());
            let letter_galley = ui.fonts_mut(|f| {
                f.layout_no_wrap("A".to_string(), font_id.clone(), egui::Color32::WHITE)
            });

            // Get letter dimensions - use the larger dimension to make square buttons
            let letter_width = letter_galley.size().x;
            let letter_height = letter_galley.size().y;
            let letter_size = letter_width.max(letter_height);

            // Add padding around the letter for the button
            let padding = ui.spacing().button_padding;
            let button_size = letter_size + padding.x * 2.0;

            // Calculate available space
            let available_size = ui.available_size();

            // Calculate maximum number of buttons that can fit (no longer need square grid)
            let max_cols = (available_size.x / button_size).floor() as usize;
            let max_rows = (available_size.y / button_size).floor() as usize;

            println!("max cols: {max_cols}");
            println!("max rows: {max_rows}");

            // Use all available space
            self.grid_cols = max_cols.max(1); // At least 1 column
            self.grid_rows = max_rows.max(1); // At least 1 row

            // Store original spacing to restore later
            let original_spacing = ui.spacing().clone();

            // Set spacing to zero for the grid
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

            // Center the grid
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    // Create the grid
                    for row in 0..self.grid_rows {
                        ui.horizontal(|ui| {
                            for col in 0..self.grid_cols {
                                let button = egui::Button::new("A")
                                    .min_size(egui::vec2(button_size, button_size))
                                    .rounding(0.0); // No rounding - sharp corners

                                if ui.add(button).clicked() {
                                    println!("Button clicked at row: {}, col: {}", row, col);
                                }
                            }
                        });
                    }
                });
            });

            // Restore original spacing
            *ui.spacing_mut() = original_spacing;

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
