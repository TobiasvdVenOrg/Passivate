use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use galvanic_assert::matchers::*;
use galvanic_assert::*;
use passivate_configuration::configuration::Configuration;
use passivate_configuration::configuration_manager::ConfigurationManager;
use passivate_core::change_events::ChangeEvent;
use passivate_core::test_run_model::{SingleTest, SingleTestStatus, TestRun, TestRunEvent};
use passivate_delegation::Tx;
use passivate_hyp_names::hyp_id::HypId;
use passivate_hyp_names::test_name;
use passivate_testing::path_resolution::test_data_path;
use rstest::*;

use crate::views::{DetailsView, TestRunView, View};

#[test]
pub fn show_a_passing_test()
{
    let failing_test = example_hyp("example_crate::example_test", SingleTestStatus::Passed);

    show_test(&test_name!(), failing_test);
}

#[test]
pub fn show_a_failing_test()
{
    let failing_test = example_hyp("example_crate::example_test", SingleTestStatus::Failed);

    show_test(&test_name!(), failing_test);
}

#[test]
pub fn show_a_failing_test_with_output()
{
    let failing_test = SingleTest::new(
        HypId::new("example_crate", "example_test").unwrap(),
        SingleTestStatus::Failed,
        vec!["this is some error output".to_string(), "you messed up".to_string()]
    );

    show_test(&test_name!(), failing_test);
}

#[test]
pub fn selecting_a_test_shows_it_in_details_view()
{
    let (test_run_tx, test_run_rx) = Tx::new();
    let (details_tx, details_rx) = Tx::new();

    let configuration = ConfigurationManager::new(Configuration::default(), Tx::stub());

    let mut details_view = DetailsView::new(details_rx, Tx::stub(), configuration);
    let mut test_run_view = TestRunView::new(test_run_rx, details_tx);

    let mut test_run_ui = Harness::new_ui(|ui: &mut egui::Ui| {
        test_run_view.ui(ui);
    });

    let mut details_ui = Harness::new_ui(|ui: &mut egui::Ui| {
        details_view.ui(ui);
    });

    let mut test_run = TestRun::default();
    test_run.tests.add(example_hyp("tests::example_test", SingleTestStatus::Failed));
    test_run_tx.send(test_run);

    test_run_ui.run();

    let test_entry = test_run_ui.get_by_label("example_test");
    test_entry.click();

    test_run_ui.run();
    details_ui.run();

    details_ui.fit_contents();
    details_ui.snapshot(&test_name!());
}

#[test]
pub fn when_a_test_is_selected_and_then_changes_status_the_details_view_also_updates()
{
    let (test_run_tx, test_run_rx) = Tx::new();
    let (details_tx, details_rx) = Tx::new();
    let configuration = ConfigurationManager::new(Configuration::default(), Tx::stub());
    let mut details_view = DetailsView::new(details_rx, Tx::stub(), configuration);
    let mut test_run_view = TestRunView::new(test_run_rx, details_tx);

    let mut test_run_ui = Harness::new_ui(|ui: &mut egui::Ui| {
        test_run_view.ui(ui);
    });

    let mut details_ui = Harness::new_ui(|ui: &mut egui::Ui| {
        details_view.ui(ui);
    });

    let mut test_run = TestRun::default();
    test_run.update(TestRunEvent::TestFinished(example_hyp("tests::example_test", SingleTestStatus::Failed)));
    test_run_tx.send(test_run.clone());

    test_run_ui.run();

    let test_entry = test_run_ui.get_by_label("example_test");
    test_entry.click();

    test_run_ui.run();
    details_ui.run();

    test_run.update(TestRunEvent::TestFinished(example_hyp("tests::example_test", SingleTestStatus::Passed)));
    test_run_tx.send(test_run);

    test_run_ui.run();
    details_ui.run();

    details_ui.fit_contents();
    details_ui.snapshot(&test_name!());
}

#[test]
pub fn show_snapshot_associated_with_test_rgb()
{
    let test_with_snapshot = example_hyp("tests::example_snapshot_rgb", SingleTestStatus::Failed);

    show_test(&test_name!(), test_with_snapshot);
}

#[test]
pub fn show_snapshot_associated_with_test_rgba()
{
    let test_with_snapshot = example_hyp("tests::example_snapshot_rgba", SingleTestStatus::Failed);

    show_test(&test_name!(), test_with_snapshot);
}

#[test]
pub fn show_current_and_new_snapshots_associated_with_test()
{
    let test_with_changed_snapshot = example_hyp("tests::example_snapshot_changed", SingleTestStatus::Failed);

    show_test(&test_name!(), test_with_changed_snapshot);
}

#[test]
pub fn show_only_new_snapshot_associated_with_test_when_there_is_no_current_snapshot()
{
    let test_first_run = example_hyp("tests::example_snapshot_only_new", SingleTestStatus::Failed);

    show_test(&test_name!(), test_first_run);
}

#[test]
pub fn show_only_one_snapshot_when_both_current_and_new_are_present_but_identical()
{
    let test_run_identical_snapshot = example_hyp("tests::example_snapshot_identical", SingleTestStatus::Failed);

    show_test(&test_name!(), test_run_identical_snapshot);
}

#[rstest]
#[case::current_and_new("tests::example_snapshot_changed")]
#[case::only_new("tests::example_snapshot_only_new")]
pub fn approving_new_snapshot_emits_event_to_run_test_with_update_snapshots_enabled(#[case] hyp: &str)
{
    let snapshot_test = example_hyp(hyp, SingleTestStatus::Failed);

    let (details_tx, details_rx) = Tx::new();
    let (test_run_tx, test_run_rx) = Tx::new();
    let configuration = get_configuration_with_example_snapshots_path();

    let mut details_view = DetailsView::new(details_rx, test_run_tx, configuration);

    let ui = |ui: &mut egui::Ui| {
        details_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    details_tx.send(Some(snapshot_test));
    harness.run();

    let approve = harness.get_by_label("Approve");
    approve.click();
    harness.run();

    let approval_run = test_run_rx.last().unwrap();

    assert_that!(
        &approval_run,
        has_structure!(ChangeEvent::SingleHyp {
            id: eq(HypId::new("example_crate", hyp).unwrap()),
            update_snapshots: eq(true)
        })
    );
}

fn show_test(test_name: &str, single_test: SingleTest)
{
    let (tx, rx) = Tx::new();
    let configuration = get_configuration_with_example_snapshots_path();

    let mut details_view = DetailsView::new(rx, Tx::stub(), configuration);

    let ui = |ui: &mut egui::Ui| {
        details_view.ui(ui);
    };

    let mut harness = Harness::new_ui(ui);

    tx.send(Some(single_test));

    harness.run();
    harness.fit_contents();
    harness.snapshot(test_name);
}

fn get_configuration_with_example_snapshots_path() -> ConfigurationManager
{
    ConfigurationManager::new(
        Configuration {
            snapshots_path: Some(test_data_path().join("example_snapshots").to_string()),
            ..Configuration::default()
        },
        Tx::stub()
    )
}

fn example_hyp(name: &str, status: SingleTestStatus) -> SingleTest
{
    let id = HypId::new("example_crate", name).unwrap();
    SingleTest::new(id, status, vec![])
}
