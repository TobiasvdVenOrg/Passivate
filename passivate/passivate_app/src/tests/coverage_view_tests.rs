use egui::accesskit::Role;
use egui_kittest::{Harness, kittest::Queryable};
use passivate_delegation::{channel, Rx, Tx};
use passivate_core::test_helpers::fakes::test_run_actor_fakes;
use passivate_core::{configuration::ConfigurationHandler, coverage::CoverageStatus, passivate_grcov::CovdirJson};
use passivate_delegation::Actor;
use stdext::function_name;
use crate::views::{CoverageView, View};
use indexmap::IndexMap;

#[test]
pub fn show_coverage_hierarchy_fully_collapsed() {
    let (coverage_sender, coverage_receiver) = channel();

    let mut coverage_view = CoverageView::new(coverage_receiver, Tx::stub());

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let coverage_info = CovdirJson {
        children: Some(IndexMap::new()),
        coverage_percent: 88.0,
        lines_covered: 64,
        lines_missed: 16,
        lines_total: 80,
        name: "example.rs".to_string(),
    };

    coverage_sender.send(CoverageStatus::Done(Box::new(coverage_info)));

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn show_coverage_hierarchy_expand_children() {
    let (coverage_sender, coverage_receiver) = channel();

    let mut coverage_view = CoverageView::new(coverage_receiver, Tx::stub());

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let coverage_info = CovdirJson {
        children: Some(IndexMap::from([
            ("child1.rs".to_string(), CovdirJson {
                children: None,
                coverage_percent: 88.0,
                lines_covered: 64,
                lines_missed: 16,
                lines_total: 80,
                name: "child1.rs".to_string()
            }),
            ("child2.rs".to_string(), CovdirJson {
                children: Some(IndexMap::from([
                    ("nested1.rs".to_string(), CovdirJson {
                        children: None,
                        coverage_percent: 12.0,
                        lines_covered: 64,
                        lines_missed: 16,
                        lines_total: 80,
                        name: "nested1.rs".to_string()
                    }),
                    ("nested2.rs".to_string(), CovdirJson {
                        children: None,
                        coverage_percent: 24.0,
                        lines_covered: 64,
                        lines_missed: 16,
                        lines_total: 80,
                        name: "nested2.rs".to_string()
                    })
                ])),
                coverage_percent: 100.0,
                lines_covered: 64,
                lines_missed: 16,
                lines_total: 80,
                name: "child2.rs".to_string()
            })
        ])),
        coverage_percent: 69.0,
        lines_covered: 64,
        lines_missed: 16,
        lines_total: 80,
        name: "example.rs".to_string(),
    };

    coverage_sender.send(CoverageStatus::Done(Box::new(coverage_info)));

    harness.run();

    let top_level_header = harness.get_by_role(Role::Unknown);
    top_level_header.click();

    let top_level_header_id = top_level_header.id();

    harness.run();
    
    for header in harness.get_all_by_role(Role::Unknown) {
        if header.id() == top_level_header_id {
            continue;
        }

        header.click();
    }

    harness.run();
    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn enable_button_when_coverage_is_disabled_triggers_configuration_event() {
    let (test_run_tx, mut test_run_actor) = test_run_actor_fakes::stub();

    let configuration = ConfigurationHandler::new(Tx::from_actor(test_run_tx), Tx::stub());
    let (configuration_tx, mut configuration_actor) = Actor::new(configuration);

    let mut coverage_view = CoverageView::new(Rx::stub(), Tx::from_actor(configuration_tx));

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    let enable_button = harness.get_by_label("Enable");
    enable_button.click();

    harness.run();

    let configuration = configuration_actor.into_inner();
    let test_run_handler = test_run_actor.into_inner();

    assert!(test_run_handler.coverage_enabled());
    assert!(configuration.configuration().coverage_enabled);

    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

#[test]
pub fn show_error() {
    let (coverage_sender, coverage_receiver) = channel();
    let mut coverage_view = CoverageView::new(coverage_receiver, Tx::stub());

    let ui = |ui: &mut egui::Ui|{
        coverage_view.ui(ui);
    };

    coverage_sender.send(CoverageStatus::Error("Something went wrong with the coverage!".to_string()));
    
    let mut harness = Harness::new_ui(ui);
    harness.run();

    harness.fit_contents();
    harness.snapshot(&test_name(function_name!()));
}

fn test_name(function_name: &str) -> String {
    function_name.split("::").last().unwrap_or(function_name).to_string()
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

//     let (tests_sender, _tests_receiver) = channel();
//     let (coverage_sender, coverage_receiver) = channel();
//     let mut test_runner = ChangeEventHandler::new(Box::new(run_tests), Box::new(compute_coverage), tests_sender, coverage_sender);

//     test_runner.handle_event(ChangeEvent::File);

//     let (change_event_sender, _change_event_receiver) = channel();
//     let mut coverage_view = CoverageView::new(coverage_receiver, change_event_sender);

//     let ui = |ui: &mut egui::Ui|{
//         coverage_view.ui(ui);
//     };

//     let mut harness = Harness::new_ui(ui);

//     harness.run();

//     harness.fit_contents();
//     harness.snapshot("when_grcov_is_not_installed_error_is_reported");
// }
