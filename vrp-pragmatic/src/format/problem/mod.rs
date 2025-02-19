//! Specifies logic to read problem and routing matrix from json input.
//!

mod model;
pub use self::model::*;

mod reader;
pub use self::reader::create_approx_matrices;
pub use self::reader::PragmaticProblem;

pub(crate) fn get_job_tasks(job: &Job) -> impl Iterator<Item = &JobTask> {
    job.pickups.iter().chain(job.deliveries.iter()).chain(job.services.iter()).chain(job.replacements.iter()).flatten()
}
