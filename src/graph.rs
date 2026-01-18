use eframe::{App, CreationContext, NativeOptions, run_native};
use egui::Context;
use egui_graphs::{
    FruchtermanReingold, FruchtermanReingoldState, Graph, GraphView, LayoutForceDirected, Node,
    SettingsInteraction, SettingsNavigation, SettingsStyle, to_graph_custom,
};
use petgraph::stable_graph::StableGraph;
use std::collections::HashMap;

use crate::skills::Skill;

pub struct BasicApp {
    pub g: Graph<Skill, ()>,
}

impl BasicApp {
    fn new(_: &CreationContext<'_>, skills: &[Skill]) -> Self {
        let g = build_skill_graph(skills);
        Self { g }
    }
}

impl App for BasicApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        type S = FruchtermanReingoldState;
        type L = LayoutForceDirected<FruchtermanReingold>;
        let mut graph_view = GraphView::<_, _, _, _, _, _, S, L>::new(&mut self.g)
            .with_styles(&SettingsStyle::default().with_labels_always(true))
            .with_interactions(&SettingsInteraction::default().with_node_selection_enabled(true))
            .with_navigations(&SettingsNavigation::default().with_zoom_and_pan_enabled(true));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ASK Skill Graph ");
            ui.add(&mut graph_view);
        });
    }
}

pub fn build_skill_graph(skills: &[Skill]) -> Graph<Skill, ()> {
    // First, build a petgraph StableGraph
    let mut sg: StableGraph<Skill, ()> = StableGraph::new();

    // Map skill IDs to node indices for edge creation
    let mut id_to_idx: HashMap<u32, petgraph::stable_graph::NodeIndex> = HashMap::new();

    // Add all nodes
    for skill in skills {
        let idx = sg.add_node(skill.clone());
        id_to_idx.insert(skill.id, idx);
    }

    // Add edges based on dependencies
    for skill in skills {
        if let Some(deps) = &skill.dependencies {
            for dep_id in deps {
                if let (Some(&from_idx), Some(&to_idx)) =
                    (id_to_idx.get(dep_id), id_to_idx.get(&skill.id))
                {
                    // Edge goes from dependency -> skill (dep must be learned first)
                    sg.add_edge(from_idx, to_idx, ());
                }
            }
        }
    }

    // Convert to egui_graphs::Graph with custom label transform
    to_graph_custom(
        &sg,
        |node: &mut Node<Skill, (), _, _, _>| {
            // Set label from skill name, falling back to ID
            let label = node
                .payload()
                .name
                .clone()
                .unwrap_or_else(|| node.payload().id.to_string());
            node.set_label(label);
        },
        |_edge| {
            // Default edge transform (or customize if needed)
        },
    )
}

pub fn app_with_skills(skills: &[Skill]) {
    run_native(
        "ASK Graph",
        NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(BasicApp::new(cc, skills)))),
    )
    .unwrap();
}
