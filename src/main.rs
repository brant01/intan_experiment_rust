
use crate::experiment::experiment::Experiment;

fn main() {
    // create a new experiment
    let mut experiment = Experiment::new();

    experiment.display_host_options();
}