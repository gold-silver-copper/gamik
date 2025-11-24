use crate::structs::*;
use egui::{FontId, RichText};
pub struct TemplateApp {
    player_id: Entity,
    grid_cols: usize,
    grid_rows: usize,
    button_size: Option<f32>,
    world: GameWorld,
    font_size: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            player_id: Entity(0),
            grid_cols: 1,      // Will be recalculated
            grid_rows: 1,      // Will be recalculated
            button_size: None, // Will be calculated on first frame
            world: GameWorld::create_test_world(),
            font_size: 20.0, // Default font size
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Include font files at compile time
        const SC_FONT_DATA: &[u8] = include_bytes!("../assets/fonts/noto-sc-regular.ttf");
        const TC_FONT_DATA: &[u8] = include_bytes!("../assets/fonts/noto-tc-regular.ttf");

        // Load Chinese font
        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "sc_font".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(SC_FONT_DATA)),
        );
        fonts.font_data.insert(
            "tc_font".to_owned(),
            std::sync::Arc::new(egui::FontData::from_static(TC_FONT_DATA)),
        );

        // Put sc_font first (highest priority):
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "sc_font".to_owned());

        // Add tc_font as well
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(1, "tc_font".to_owned());

        // Set fonts for monospace text
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "sc_font".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(1, "tc_font".to_owned());

        // Apply the fonts to the context
        cc.egui_ctx.set_fonts(fonts);

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        let client_info = self.world.send_client_info(self.player_id);
        self.input(ctx);
        // Process all events
        self.world.process_events();

        // Right sidebar
        self.right_panel(ctx);

        // Bottom bar
        self.bottom_panel(ctx);
        // Central panel with letter grid
        self.rogue_screen(ctx);
    }
}

impl TemplateApp {
    pub fn input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            if i.key_pressed(egui::Key::W) || i.key_pressed(egui::Key::ArrowUp) {
                self.world.event_queue.push(GameEvent::Move {
                    entity: self.world.player,
                    direction: Direction::Up,
                });
            }

            if i.key_pressed(egui::Key::Q) {
                panic!();
            }

            if i.key_pressed(egui::Key::S) || i.key_pressed(egui::Key::ArrowDown) {
                self.world.event_queue.push(GameEvent::Move {
                    entity: self.world.player,
                    direction: Direction::Down,
                });
            }
            if i.key_pressed(egui::Key::A) || i.key_pressed(egui::Key::ArrowLeft) {
                self.world.event_queue.push(GameEvent::Move {
                    entity: self.world.player,
                    direction: Direction::Left,
                });
            }
            if i.key_pressed(egui::Key::D) || i.key_pressed(egui::Key::ArrowRight) {
                self.world.event_queue.push(GameEvent::Move {
                    entity: self.world.player,
                    direction: Direction::Right,
                });
            }
        });
    }
    pub fn right_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Right Sidebar");
                ui.separator();
                ui.label("Sidebar content here");
            });
    }

    pub fn bottom_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .default_height(100.0)
            .show(ctx, |ui| {
                ui.heading("Bottom Bar");
                ui.separator();
                ui.label("Bottom bar content here");
            });
    }

    fn rogue_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Customize button styling for tighter spacing
            let style = ui.style_mut();
            style.spacing.button_padding = egui::vec2(0.0, 0.0);
            style.visuals.widgets.inactive.bg_stroke.width = 0.0;
            style.visuals.widgets.hovered.bg_stroke.width = 0.0;
            style.visuals.widgets.active.bg_stroke.width = 0.0;

            // Calculate button size on first frame if not already calculated
            if self.button_size.is_none() {
                let chinese_char = "中";
                let font_id = egui::FontId::new(self.font_size, egui::FontFamily::Proportional);
                let letter_galley = ui.fonts_mut(|f| {
                    f.layout_no_wrap(
                        chinese_char.to_string(),
                        font_id.clone(),
                        egui::Color32::WHITE,
                    )
                });

                // Get letter dimensions - use the larger dimension to make square buttons
                let letter_width = letter_galley.size().x;
                let letter_height = letter_galley.size().y;
                let letter_size = letter_width.max(letter_height);

                // Minimal padding for tight roguelike feel
                let padding = -4.0;
                self.button_size = Some(letter_size + padding);
            }

            let button_size = self.button_size.unwrap();

            // Calculate available space
            let available_size = ui.available_size();

            // Calculate maximum number of buttons that can fit
            let max_cols = (available_size.x / button_size).floor() as usize;
            let max_rows = (available_size.y / button_size).floor() as usize;

            // Use all available space
            self.grid_cols = max_cols.max(1); // At least 1 column
            self.grid_rows = max_rows.max(1); // At least 1 row

            // Calculate center position
            let center_row = self.grid_rows / 2;
            let center_col = self.grid_cols / 2;

            // Set spacing to zero for the grid
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

            // Center the grid
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    // Create the grid
                    for row in 0..self.grid_rows {
                        ui.horizontal(|ui| {
                            for col in 0..self.grid_cols {
                                let point = Point {
                                    x: col as i32,
                                    y: row as i32,
                                };

                                // Get the character to display at this position
                                let button_text = self.world.get_display_char(&point);

                                let button = egui::Button::new(
                                    RichText::new(button_text)
                                        .font(FontId::proportional(self.font_size)),
                                )
                                .min_size(egui::vec2(button_size, button_size))
                                .small()
                                .corner_radius(0.0);
                                if ui.add(button).clicked() {
                                    println!("Button clicked at row: {}, col: {}", row, col);
                                }
                            }
                        });
                    }
                });
            });
        });
    }
}
