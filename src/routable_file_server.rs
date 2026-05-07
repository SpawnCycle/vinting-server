use std::path::PathBuf;

use rocket::{fs::NamedFile, get};

use crate::responder::Responder;

/// React router does internal routing,
/// which is fine when the user is simply navigating the website,
/// however, when the user refreshes the page it will 404,
/// since the path most likely won't be found.
/// This is where this function comes in.
/// Regardless of the requested file, this will return the root index.html,
/// so react-router can do client-side routing
///
/// # Errors
///
/// This function returns an io error if there is no index file in the web directory
#[get("/<_file..>", rank = 999)]
pub async fn get_root_regardless(_file: PathBuf) -> Result<NamedFile, Responder> {
    NamedFile::open("./web/index.html")
        .await
        .map_err(Responder::server_error)
}
