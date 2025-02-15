use super::CoverageStatus;


pub trait ComputeCoverage {
    fn compute_coverage() -> CoverageStatus;
}
