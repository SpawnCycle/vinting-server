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
