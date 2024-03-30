use std::collections::HashMap;

use crate::{db::connection::Database, model::EssayList, Uuid, error::Error};

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub essaylist: EssayList,
    pub essaymap: HashMap<Uuid, String>,
}

impl AppState {
    pub async fn new(
        db: Database,
    ) -> Result<Self, Error> {
        let essaylist = db.query_essaylist().await?;
        let essaymap = essaylist.get_content_map().await?;
        Ok(
            Self {
                db,
                essaylist,
                essaymap,
            }
        )
    }
}