


use intan_experiment::experiments::experiment::Experiment;

fn main() {
    // create a new experiment
    let mut experiment = Experiment::new();

    experiment.display_host_options();

    experiment.display_device_options();
}