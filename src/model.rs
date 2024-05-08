use std::{collections::HashMap, sync::Arc};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use crate::{error::{Error, ModelError}, SafeBTreeSet, SafeVec, Uuid};


#[derive(Debug, Clone, Serialize)]
pub struct EssayInfo {
    pub eid: Uuid,
    pub title: String,
    pub date: NaiveDateTime,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub brief: String,
}

impl EssayInfo {
    pub fn new(
        eid: Uuid,
        title: String,
        date: NaiveDateTime,
        categories: Vec<String>,
        tags: Vec<String>,
        brief: String,
    ) -> Self {
        Self {
            eid, title, date, categories, tags, brief
        }
    }
}

#[derive(Debug, Clone)]
pub struct Essay {
    essayinfo: EssayInfo,
    content: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Essay {
    pub fn new (
        essayinfo: EssayInfo, 
        content: &str, 
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            essayinfo,
            content: content.to_string(),
            created_at,
            updated_at
        }
    }
}

#[derive(Clone)]
pub struct EssayList {
    essaylist: SafeVec<Essay>,
}

impl EssayList {
    pub async fn new() -> Self {
        Self {
            essaylist: Arc::default()
        }
    }
    pub async fn create_essay(&self, essay: Essay) -> Result<(), Error> {
        let mut list = self.essaylist.lock().unwrap();
        list.push(essay);
        Ok(())
    }
    pub async fn get_content_map(&self) -> Result<HashMap<Uuid, String>, Error> {
        let list = self.essaylist.lock().unwrap();
        let essaymap = list.iter().map(|es| {
            let eid = es.essayinfo.eid.clone();
            let content = es.content.clone();
            (eid, content)
        }
        ).collect();
        Ok(essaymap)
    }
    pub async fn get_list(&self) -> Result<Vec<EssayInfo>, Error> {
        let list = self.essaylist.lock().unwrap();
        let mut res: Vec<EssayInfo> = list.iter().filter_map(|t| Some(t.essayinfo.clone()) ).collect();
        res.sort_by(|a, b| b.date.cmp(&a.date));
        Ok( res )
    }
}

pub struct TagMap {
    tagmap: HashMap<u32, String>,
}
pub struct CategoryMap {
    categorymap: HashMap<u32, String>,
}

impl TagMap {
    pub fn new() -> Self {
        Self {
            tagmap: HashMap::default()
        }
    }
    pub fn insert_tag(&mut self, id: u32, name: String) -> Result<(), ModelError> {
        match self.tagmap.insert(id, name) {
            None => Ok(()),
            Some(_) => Err(ModelError::UnexpectedData),
        }
    }
    pub fn get(&self, id: u32) -> Option<&String> {
        self.tagmap.get(&id)
    }
}

impl CategoryMap {
    pub fn new() -> Self {
        Self {
            categorymap: HashMap::default()
        }
    }
    pub fn insert_category(&mut self, id: u32, name: String) -> Result<(), ModelError> {
        match self.categorymap.insert(id, name) {
            None => Ok(()),
            Some(_) => Err(ModelError::UnexpectedData),
        }
    }
    pub fn get(&self, id: u32) -> Option<&String> {
        self.categorymap.get(&id)
    }
}

#[derive(Debug, Clone ,Serialize, sqlx::FromRow, PartialEq, PartialOrd, Eq)]
pub struct Message {
    uid : Uuid,
    content: String,
    created_at: DateTime<Utc>
}

impl Message {
    pub fn new(uid: &Uuid, content: &str, created_at: DateTime<Utc>) -> Self {
        Self {
            uid: uid.clone(),
            content: String::from(content),
            created_at,
        }
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.created_at.cmp(&other.created_at)
    }
}

#[derive(Debug, Clone)]
pub struct MessageList {
    msglist: SafeBTreeSet<Message>,
}

impl MessageList {
    pub fn new() -> Self {
        Self {
            msglist: Arc::default()
        }
    }
    pub async fn create_msg(&self, msg: Message) -> Result<(), Error> {
        let mut list = self.msglist.lock().unwrap();
        list.insert(msg);
        Ok(())
    }

    pub async fn from_vec(msgs: Vec<Message>) -> Result<Self, Error> {
        let res = Self::new();
        for msg in msgs {
            res.create_msg(msg).await?;
        }
        Ok(res)
    }

    pub async fn get_vec(&self) -> Result<Vec<Message>, Error> {
        let list = self.msglist.lock().unwrap();
        Ok(list.iter().cloned().collect())
    }
}