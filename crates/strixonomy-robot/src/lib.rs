//! Thin wrappers around the [ROBOT](https://github.com/ontodev/robot) CLI.

mod error;
mod runner;

pub use error::{Result, RobotError};
pub use runner::{
    detect_robot, robot_convert, robot_merge, robot_report, robot_validate, run_robot, RobotOutput,
};
