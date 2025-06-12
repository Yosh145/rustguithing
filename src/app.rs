// manage what tab is visible
#[derive(PartialEq, Debug)]
enum Tab {
    Tracker,
    Settings,
}

// example radio selection
#[derive(PartialEq, Clone, Copy, Debug)]
enum RadioSelection {
    First,
    Second,
    Third,
}

// dropdown 1
#[derive(PartialEq, Clone, Copy, Debug)]
enum DropdownSelection {
    OptionA,
    OptionB,
    OptionC,
}

pub struct UrlTrackerApp {
    selected_tab: Tab,
    radio_selection: RadioSelection,
    dropdown_two_inputs: DropdownSelection,
    dropdown_n_inputs_1: DropdownSelection,
    dropdown_n_inputs_2: DropdownSelection,
    dropdown_type: DropdownSelection,
    enable_dark_mode: bool,
    slider: f32,
    url: String,
}

impl Default for UrlTrackerApp {
    fn default() -> Self {
        Self {
            selected_tab: Tab::Tracker,
            radio_selection: RadioSelection::First,
            dropdown_two_inputs: DropdownSelection::OptionA,
            dropdown_n_inputs_1: DropdownSelection::OptionA,
            dropdown_n_inputs_2: DropdownSelection::OptionA,
            dropdown_type: DropdownSelection::OptionA,
            enable_dark_mode: true,
            slider: 30.0,
            url: String::new(),
        }
    }
}

// first tab func
impl UrlTrackerApp {
    fn draw_tracker_tab(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .resizable(true)
            // ! change here for side panel length 
            .default_width(350.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(15.0);
                    if ui.add_sized([250.0, 40.0], egui::Button::new(egui::RichText::new("Fetch").heading())).clicked() {
                        // fetch url
                    }
                });

                ui.add_space(15.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.indent("left_panel_indent", |ui| {
                        ui.radio_value(&mut self.radio_selection, RadioSelection::First, "Radio 1");
                        ui.radio_value(&mut self.radio_selection, RadioSelection::Second, "Radio 2");
                        ui.radio_value(&mut self.radio_selection, RadioSelection::Third, "Radio 3");
                        ui.add_space(25.0);
                        
                        egui::Grid::new("grid_one_dropdown").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                            ui.label("text");
                            ui.label("text");
                            ui.end_row();
                            ui.label("");
                            add_dropdown(ui, "dd_two_inputs", &mut self.dropdown_two_inputs);
                            ui.end_row();
                        });
                        ui.add_space(15.0);
                        
                        egui::Grid::new("grid_two_dropdowns").num_columns(2).spacing([10.0, 8.0]).show(ui, |ui| {
                            ui.label("text");
                            ui.label("text");
                            ui.end_row();
                            add_dropdown(ui, "dd_n1", &mut self.dropdown_n_inputs_1);
                            add_dropdown(ui, "dd_n2", &mut self.dropdown_n_inputs_2);
                            ui.end_row();
                        });
                    });
                });
                
                ui.add_space(25.0);
                ui.vertical_centered(|ui| {
                    ui.label("Text");
                    add_dropdown(ui, "dd_type", &mut self.dropdown_type);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label("Element here that will display information later developed");
            });
        });
    }

    /// settings tab
    fn draw_settings_tab(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ew grids
            egui::Grid::new("settings_grid")
                .num_columns(2)
                .spacing([40.0, 8.0])
                .striped(true) // red white and blue mfsss USA USA USA
                .show(ui, |ui| {
                    ui.label("Theme");
                    ui.checkbox(&mut self.enable_dark_mode, "Enable Dark Mode");
                    ui.end_row();

                    ui.label("Slider yay");
                    ui.add(egui::Slider::new(&mut self.slider, 1.0..=120.0).suffix(" s"));
                    ui.end_row();

                    ui.label("URL");
                    ui.add(egui::TextEdit::singleline(&mut self.url).password(false));
                    ui.end_row();
                });
        });
    }
}

impl eframe::App for UrlTrackerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // darkmode toggle
        if self.enable_dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, Tab::Tracker, "URL Tracker");
                ui.selectable_value(&mut self.selected_tab, Tab::Settings, "Settings");
            });
        });

        match self.selected_tab {
            Tab::Tracker => self.draw_tracker_tab(ctx),
            Tab::Settings => self.draw_settings_tab(ctx),
        }
    }
}


// helper for dropdown maker (taken from some website hope it works???)
fn add_dropdown(ui: &mut egui::Ui, id_source: &str, selection: &mut DropdownSelection) {
    ui.set_min_width(150.0);
    egui::ComboBox::from_id_salt(id_source)
        .selected_text(format!("{:?}", selection))
        .show_ui(ui, |ui| {
            ui.selectable_value(selection, DropdownSelection::OptionA, "Option A");
            ui.selectable_value(selection, DropdownSelection::OptionB, "Option B");
            ui.selectable_value(selection, DropdownSelection::OptionC, "Option C");
        });
}