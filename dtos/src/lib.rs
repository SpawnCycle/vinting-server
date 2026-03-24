use std::thread;

// utility stuff for serde
pub mod email_string;
pub mod limited_string;

pub mod category;
pub mod image;
pub mod order;
pub mod product;
pub mod tag;
pub mod user;

// file structure: <model>/<method>.rs

/// # Panics
///
/// panics if the configuration is incorrect
#[must_use]
pub fn get_argon_params() -> argon2::Params {
    // SAFETY: Unless you have more cores than u32::MAX, this should never truncate
    #[allow(clippy::cast_possible_truncation)]
    let thread_num = (thread::available_parallelism().map_or(2, Into::into) as u32)
        .max(argon2::Params::MIN_T_COST);
    // More memory seems to make the hashing more costly in a linear fashion
    argon2::Params::new(
        1024,           // Memory cost
        thread_num * 2, // Number of iterations
        thread_num / 2, // Degree of parallelism
        None,           // Output size
    )
    .expect("Argon2 configuration should be withing the allowed limits")
}

#[macro_export]
macro_rules! from_models {
    // `$entity` has to be an already imported module
    ($entity:ident, $type:ty, $vname:ident, $def:block) => {
        impl From<$entity::Model> for $type {
            fn from($vname: $entity::Model) -> Self {
                $def
            }
        }

        impl From<$entity::ModelEx> for $type {
            fn from($vname: $entity::ModelEx) -> Self {
                $def
            }
        }
    };
}
