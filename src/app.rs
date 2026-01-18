use egui::{Pos2, Rect, Scene, Vec2};
use egui_graphs::{
    FruchtermanReingold, FruchtermanReingoldState, Graph, GraphView, LayoutForceDirected,
    SettingsInteraction, SettingsNavigation, SettingsStyle,
};

use crate::{graph::build_skill_graph, parse_lua, skills::Skill};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GraphApp {
    // Example stuff:
    #[serde(skip)] // This how you opt-out of serialization of a field
    g: Option<Graph<Skill, ()>>,

    graph_scene: Rect,
}

impl Default for GraphApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            g: None,
            graph_scene: Rect::ZERO,
        }
    }
}

impl GraphApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, skills: Option<&[Skill]>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(skills) = skills {
            let g = build_skill_graph(skills);
            log::info!("Loaded graph with {} nodes", g.node_count());
            Self {
                g: Some(g),
                ..Default::default()
            }
        } else if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for GraphApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("ASK Skill Graph Viewer");

            ui.separator();

            if let Some(graph) = self.g.as_mut() {
                type S = FruchtermanReingoldState;
                type L = LayoutForceDirected<FruchtermanReingold>;
                let mut graph_view = GraphView::<_, _, _, _, _, _, S, L>::new(graph)
                    .with_styles(&SettingsStyle::default().with_labels_always(true))
                    .with_interactions(
                        &SettingsInteraction::default().with_node_selection_enabled(true),
                    )
                    .with_navigations(
                        &SettingsNavigation::default().with_zoom_and_pan_enabled(true),
                    );

                egui::Frame::group(ui.style())
                    .inner_margin(0.0)
                    .show(ui, |ui| {
                        let scene = Scene::new().zoom_range(0.1..=20.0);

                        let mut reset_view = false;
                        let mut inner_rect = Rect::NAN;
                        let response = scene
                            .show(ui, &mut self.graph_scene, |ui| {
                                reset_view = ui.button("Reset view").clicked();
                                ui.add_space(16.0);
                                ui.add(&mut graph_view);
                                inner_rect = ui.min_rect();
                            })
                            .response;

                        if reset_view || response.double_clicked() {
                            self.graph_scene = inner_rect;
                        }
                    });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
