use egui::collapsing_header::CollapsingState;
use egui::{Color32, RichText, Ui};
use passivate_configuration::configuration::ConfigurationChange;
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_coverage::grcov::covdir_json::CovdirJson;

pub struct CoverageView;

impl CoverageView
{
    pub fn ui(&mut self, ui: &mut Ui, status: &CoverageStatus) -> Option<ConfigurationChange>
    {
        match status
        {
            CoverageStatus::Disabled => self.draw_disabled(ui),
            CoverageStatus::Error(coverage_error) =>
            {
                let text = RichText::new(coverage_error).size(16.0).color(Color32::RED);
                ui.heading(text);
                None
            }
            CoverageStatus::Preparing =>
            {
                ui.heading("Preparing...");
                None
            }
            CoverageStatus::Running =>
            {
                ui.heading("Running...");
                None
            }
            CoverageStatus::Done(json) =>
            {
                let egui_id = egui::Id::new(format!("root{}", json.name));
                Self::draw_coverage(ui, json, egui_id);
                None
            }
        }
    }

    fn draw_coverage(ui: &mut Ui, coverage: &CovdirJson, id: egui::Id)
    {
        if coverage.children.as_ref().is_none_or(|children| children.is_empty())
        {
            ui.horizontal(|ui| {
                ui.label(&coverage.name);
                ui.label(format!("{}%", &coverage.coverage_percent));
            });
        }
        else
        {
            let default_open = false;
            CollapsingState::load_with_default_open(ui.ctx(), id, default_open)
                .show_header(ui, |ui| {
                    ui.label(&coverage.name);
                    ui.label(format!("{}%", &coverage.coverage_percent));
                })
                .body(|ui| {
                    if let Some(children) = &coverage.children
                    {
                        for child in children.values()
                        {
                            let hierarchical_id = egui::Id::new(format!("{:?}{}", id, child.name));
                            Self::draw_coverage(ui, child, hierarchical_id);
                        }
                    }
                    else
                    {
                        ui.label("None");
                    }
                });
        }
    }

    fn draw_disabled(&mut self, ui: &mut Ui) -> Option<ConfigurationChange>
    {
        ui.heading("Code coverage is disabled");

        if ui.button("Enable").clicked()
        {
            Some(ConfigurationChange::CoverageEnabled(true))
        }
        else
        {
            None
        }
    }
}

#[cfg(test)]
mod tests
{
    use egui::accesskit::Role;
    use egui_kittest::{kittest::Queryable, Harness};
    use indexmap::IndexMap;
    use passivate_coverage::{coverage_status::CoverageStatus, grcov::covdir_json::CovdirJson};
    use passivate_hyp_names::test_name;

    use crate::CoverageView;

    #[test]
    pub fn show_coverage_hierarchy_fully_collapsed()
    {
        let mut coverage_view = CoverageView;

        let coverage_info = CovdirJson {
            children: Some(IndexMap::new()),
            coverage_percent: 88.0,
            lines_covered: 64,
            lines_missed: 16,
            lines_total: 80,
            name: "example.rs".to_string()
        };

        let coverage_status = CoverageStatus::Done(Box::new(coverage_info));

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui, &coverage_status);
        };

        let mut harness = Harness::new_ui(ui);
        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn show_coverage_hierarchy_expand_children()
    {
        let mut coverage_view = CoverageView;

        let coverage_info = CovdirJson {
            children: Some(IndexMap::from([
                (
                    "child1.rs".to_string(),
                    CovdirJson {
                        children: None,
                        coverage_percent: 88.0,
                        lines_covered: 64,
                        lines_missed: 16,
                        lines_total: 80,
                        name: "child1.rs".to_string()
                    }
                ),
                (
                    "child2.rs".to_string(),
                    CovdirJson {
                        children: Some(IndexMap::from([
                            (
                                "nested1.rs".to_string(),
                                CovdirJson {
                                    children: None,
                                    coverage_percent: 12.0,
                                    lines_covered: 64,
                                    lines_missed: 16,
                                    lines_total: 80,
                                    name: "nested1.rs".to_string()
                                }
                            ),
                            (
                                "nested2.rs".to_string(),
                                CovdirJson {
                                    children: None,
                                    coverage_percent: 24.0,
                                    lines_covered: 64,
                                    lines_missed: 16,
                                    lines_total: 80,
                                    name: "nested2.rs".to_string()
                                }
                            )
                        ])),
                        coverage_percent: 100.0,
                        lines_covered: 64,
                        lines_missed: 16,
                        lines_total: 80,
                        name: "child2.rs".to_string()
                    }
                )
            ])),
            coverage_percent: 69.0,
            lines_covered: 64,
            lines_missed: 16,
            lines_total: 80,
            name: "example.rs".to_string()
        };

        let coverage_status = CoverageStatus::Done(Box::new(coverage_info));

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui, &coverage_status);
        };

        let mut harness = Harness::new_ui(ui);

        harness.run();

        let top_level_header = harness.get_by_role(Role::Unknown);
        top_level_header.click();

        let top_level_header_id = top_level_header.id();

        harness.run();

        for header in harness.get_all_by_role(Role::Unknown)
        {
            if header.id() == top_level_header_id
            {
                continue;
            }

            header.click();
        }

        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn show_error()
    {
        let mut coverage_view = CoverageView;

        let coverage_status = CoverageStatus::Error("Something went wrong with the coverage!".to_string());

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui, &coverage_status);
        };

        let mut harness = Harness::new_ui(ui);
        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    // #[test]
    // pub fn when_grcov_is_not_installed_error_is_reported() {
    //     let mut run_tests = MockRunTests::new();
    //     run_tests.expect_run_tests().returning(|_sender| Ok(()));

    //     let mut compute_coverage = MockComputeCoverage::new();
    //     compute_coverage.expect_clean_coverage_output().returning(|| Ok(()));
    //     compute_coverage.expect_compute_coverage().returning(|| {
    //         Err(CoverageError::GrcovNotInstalled(std::io::ErrorKind::NotFound))
    //     });

    //     let (tests_sender, _tests_receiver) = tx_rx();
    //     let (coverage_sender, coverage_receiver) = tx_rx();
    //     let mut test_runner = ChangeEventHandler::new(Box::new(run_tests), Box::new(compute_coverage), tests_sender, coverage_sender);

    //     test_runner.handle_event(HypRunTrigger::File);

    //     let (change_event_sender, _change_event_receiver) = tx_rx();
    //     let mut coverage_view = CoverageView::new(coverage_receiver, change_event_sender);

    //     let ui = |ui: &mut egui::Ui|{
    //         coverage_view.ui(ui);
    //     };

    //     let mut harness = Harness::new_ui(ui);

    //     harness.run();

    //     harness.fit_contents();
    //     harness.snapshot("when_grcov_is_not_installed_error_is_reported");
    // }
}
