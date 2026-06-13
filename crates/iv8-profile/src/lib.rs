pub mod source;
pub mod matrix;
pub mod projection;
pub mod defaults;
pub mod validation;
pub mod behavior_config;

pub use source::ProfileSource;
pub use matrix::ProfileMatrix;
pub use projection::EnvironmentProjection;
pub use validation::{validate, ValidationResult};
pub use behavior_config::BehaviorConfig;
