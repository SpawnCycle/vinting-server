use entity::{category, image, product, tag};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ProductPostDto {
    pub name: String,
    pub description: String,

    /// List of category ids
    pub categories: Vec<i32>,
    /// List of image ids
    pub images: Vec<i32>,
    /// List of tag ids
    pub tags: Vec<i32>,
}

impl From<ProductPostDto> for product::ActiveModelEx {
    fn from(d: ProductPostDto) -> product::ActiveModelEx {
        // user id is set outside of this function, because we get it from auth
        let mut p = product::ActiveModel::builder()
            .set_name(d.name)
            .set_description(d.description);

        // TODO: Test this
        for c_id in d.categories {
            p = p.add_category(category::ActiveModel::builder().set_id(c_id));
        }

        // TODO: Test this too
        for i_id in d.images {
            p = p.add_image(image::ActiveModel::builder().set_id(i_id));
        }

        // TODO: More tests
        for t_id in d.tags {
            p = p.add_tag(tag::ActiveModel::builder().set_id(t_id));
        }

        p
    }
}
