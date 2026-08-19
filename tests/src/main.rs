mod egg_baseline;

use epo_proc_macros::run_suite;

use crate::egg_baseline::EggSolver;

fn main() {
    run_suite!(EggSolver);
}
