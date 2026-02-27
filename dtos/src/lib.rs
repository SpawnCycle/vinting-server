// utility stuff for serde
pub mod email_string;
pub mod limited_string;

// TODO: remove the allow(unused)

#[allow(unused)]
pub mod category;
#[allow(unused)]
pub mod image;
#[allow(unused)]
pub mod order;
#[allow(unused)]
pub mod product;
#[allow(unused)]
pub mod tag;
#[allow(unused)]
pub mod user;

// TODO: dtos
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
