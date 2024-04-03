use std::{collections::HashMap, fs, time::UNIX_EPOCH};

use crate::{config::Config, db::connection::Database, error::Error, model::EssayList, Uuid};

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub essaylist: EssayList,
    pub essaymap: HashMap<Uuid, String>,
    pub config: Config,
}

impl AppState {
    pub async fn new(
    ) -> Result<Self, Error> {
        let db = Database::new().await;
        let essaylist = db.query_essaylist().await?;
        let essaymap = essaylist.get_content_map().await?;
        Ok(
            Self {
                config: Config::default(),
                db,
                essaylist,
                essaymap,
            }
        )
    }
}

/// 生成一个 uuid
pub fn get_uuid() -> Uuid {
    uuid::Uuid::new_v4().to_string()
}


/// 递归得到 `dir` 文件夹下所有后缀为 `suffix` 的文件路径。
pub fn get_entries(dir: &str, suffix: &str) -> Vec<String> {
    match fs::read_dir(dir) {
        Ok(entries) => {
            let mut res: Vec<String> = Vec::new();
            for i in entries {
                let path = i.unwrap().path();
                if path.is_dir() {
                    res.append(&mut get_entries(path.to_str().unwrap(), suffix));
                } else if path.extension().unwrap() == suffix {
                    res.push(path.to_str().unwrap().to_string());
                }
            }
            res
        },
        Err(_) => Vec::new(),
    }
}

/// 得到该路径的文件的最后一次修改时间
pub fn get_modified_time(path: &str) -> Result<f64, Error> {
    let metadata = fs::metadata(&path)?; // Error 需要处理
    let modified_time = metadata.modified()?.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    Ok(modified_time)
}