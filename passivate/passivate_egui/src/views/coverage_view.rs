use egui::collapsing_header::CollapsingState;
use egui::{Color32, RichText};
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_coverage::coverage_status::CoverageStatus;
use passivate_coverage::grcov::covdir_json::CovdirJson;
use passivate_delegation::Rx;

use crate::docking::docking_layout::DockId;
use crate::docking::view::View;

pub struct CoverageView
{
    receiver: Rx<CoverageStatus>,
    configuration: ConfigurationManager,
    status: CoverageStatus
}

impl View for CoverageView
{
    fn id(&self) -> DockId
    {
        DockId::from("coverage_view")
    }

    fn title(&self) -> String
    {
        String::from("Coverage")
    }
}

impl CoverageView
{
    pub fn new(receiver: Rx<CoverageStatus>, configuration: ConfigurationManager) -> CoverageView
    {
        CoverageView {
            receiver,
            configuration,
            status: CoverageStatus::Disabled
        }
    }

    pub fn ui(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        if let Ok(status) = self.receiver.try_recv()
        {
            self.status = status;
        }

        match &self.status
        {
            CoverageStatus::Disabled => self.draw_disabled(ui),
            CoverageStatus::Error(coverage_error) =>
            {
                let text = RichText::new(coverage_error).size(16.0).color(Color32::RED);
                ui.heading(text);
            }
            CoverageStatus::Preparing =>
            {
                ui.heading("Preparing...");
            }
            CoverageStatus::Running =>
            {
                ui.heading("Running...");
            }
            CoverageStatus::Done(json) => Self::draw_coverage(ui, json, egui::Id::new(format!("root{}", json.name)))
        };
    }

    fn draw_coverage(ui: &mut egui_dock::egui::Ui, coverage: &CovdirJson, id: egui::Id)
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

    fn draw_disabled(&mut self, ui: &mut egui_dock::egui::Ui)
    {
        ui.heading("Code coverage is disabled");

        if ui.button("Enable").clicked()
        {
            _ = self.configuration.update(|c| {
                c.coverage_enabled = true;
            });
        }
    }
}

#[cfg(test)]
mod tests
{
    use egui::accesskit::Role;
    use egui_kittest::{kittest::Queryable, Harness};
    use indexmap::IndexMap;
    use passivate_configuration::{configuration::PassivateConfiguration, configuration_manager::ConfigurationManager};
    use passivate_coverage::{compute_coverage::MockComputeCoverage, coverage_status::CoverageStatus, grcov::covdir_json::CovdirJson};
    use passivate_delegation::{Rx, Tx};
    use passivate_hyp_execution::{test_run_handler::TestRunHandler, hyp_runner::HypRunner};
    use passivate_hyp_names::test_name;

    use crate::coverage_view::CoverageView;

    #[test]
    pub fn show_coverage_hierarchy_fully_collapsed()
    {
        let (coverage_tx, coverage_rx) = Tx::new();
        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());

        let mut coverage_view = CoverageView::new(coverage_rx, configuration);

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        let coverage_info = CovdirJson {
            children: Some(IndexMap::new()),
            coverage_percent: 88.0,
            lines_covered: 64,
            lines_missed: 16,
            lines_total: 80,
            name: "example.rs".to_string()
        };

        coverage_tx.send(CoverageStatus::Done(Box::new(coverage_info)));

        harness.run();
        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn show_coverage_hierarchy_expand_children()
    {
        let (coverage_tx, coverage_rx) = Tx::new();

        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());

        let mut coverage_view = CoverageView::new(coverage_rx, configuration);

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

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

        coverage_tx.send(CoverageStatus::Done(Box::new(coverage_info)));

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
    pub fn enable_button_when_coverage_is_disabled_triggers_configuration_event()
    {
        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());
        let test_run_handler = TestRunHandler::builder()
            .configuration(configuration.clone())
            .coverage(Box::new(MockComputeCoverage::new()))
            .coverage_status_sender(Tx::stub())
            .runner(HypRunner::faux())
            .hyp_run_tx(Tx::stub())
            .build();

        let mut coverage_view = CoverageView::new(Rx::stub(), configuration.clone());

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui);
        };

        let mut harness = Harness::new_ui(ui);

        let enable_button = harness.get_by_label("Enable");
        enable_button.click();

        harness.run();

        assert!(test_run_handler.coverage_enabled());

        harness.fit_contents();
        harness.snapshot(&test_name!());
    }

    #[test]
    pub fn show_error()
    {
        let (coverage_tx, coverage_rx) = Tx::new();
        let configuration = ConfigurationManager::new(PassivateConfiguration::default(), Tx::stub());

        let mut coverage_view = CoverageView::new(coverage_rx, configuration);

        let ui = |ui: &mut egui::Ui| {
            coverage_view.ui(ui);
        };

        coverage_tx.send(CoverageStatus::Error("Something went wrong with the coverage!".to_string()));

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
