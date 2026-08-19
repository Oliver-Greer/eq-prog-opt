mod egg_baseline;

use epo_proc_macros::run_suite;
use epo::context::ProblemContext;

use crate::egg_baseline::EggSolver;

fn main() {
    run_suite!(EggSolver);
}
