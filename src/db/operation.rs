use std::collections::HashMap;

use crate::{error::Error, model::{CategoryMap, CreatedUpdatedAt, Essay, EssayInfo, EssayList, TagMap}};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::Row;
use super::connection::Database;

impl Database {
    pub async fn query_essaylist (
        & self,
    ) -> Result<EssayList, Error> {
        let tagmap = self.query_tag_map().await?;
        let categorymap = self.query_category_map().await?;
        let mut tag_relations: HashMap<String, Vec<u32>> = HashMap::new();
        let mut category_relations: HashMap<String, Vec<u32>> = HashMap::new();
        let essaylist = EssayList::new().await;
        
        let tag_essay = sqlx::query(
            r#"
SELECT * FROM essay_tag
            "#
        )
        .fetch_all(&self.pool)
        .await;
        
        let tag_essay = match tag_essay {
            Ok(data) => data,
            Err(e) => return Err(Error::from(e)),
        };

        for row in tag_essay {
            let eid : String = row.get("eid");
            let tid : u32  = row.get("tag_id");

            let entry = tag_relations.entry(eid.clone());
            entry.or_insert(Vec::new()).push(tid);
        }
        
        let category_essay = sqlx::query(
            r#"
            SELECT * FROM essay_category
            "#
        )
        .fetch_all(&self.pool)
        .await;
    
    let category_essay = match category_essay {
        Ok(data) => data,
        Err(e) => return Err(Error::from(e)),
    };

    for row in category_essay {
        let eid : String = row.get("eid");
        let cid : u32  = row.get("category_id");
        
        let entry = category_relations.entry(eid.clone());
        entry.or_insert(Vec::new()).push(cid);
    }
    
    let essays = sqlx::query(
        r#"
        SELECT * FROM essays   
            "#
        )
        .fetch_all(&self.pool)
        .await;
        
        let essays = match essays {
            Ok(data) => data,
            Err(e) => return Err(Error::from(e)),
        };
        
        for row in essays {
            let eid: String = row.get("eid");
            let title: String = row.get("title");
            let date: NaiveDateTime = row.get("date");
            let brief: String = row.get("brief");
            let content: String = row.get("content");
            let created_at : DateTime<Utc> = row.get("created_at");
            let updated_at : DateTime<Utc> = row.get("updated_at");

            let mut tags = Vec::new();
            let mut categories = Vec::new();
            for tid in tag_relations.get(&eid).expect("1") {
                let tag_name = tagmap.get(*tid).expect("2").clone();
                tags.push(tag_name);
            }
            for cid in category_relations.get(&eid).expect("3") {
                let category_name = categorymap.get(*cid).expect("4").clone();
                categories.push(category_name);
            }

            let essayinfo = EssayInfo::new(eid, title, date, categories, tags, brief);
            let created_updated_at = CreatedUpdatedAt::new(created_at, updated_at);
            let essay = Essay::new(essayinfo, &content, created_updated_at);
            essaylist.create_essay(essay).await.expect("5");
        }

        Ok(essaylist)
    }

    pub async fn query_tag_map(
        & self
    ) -> Result<TagMap, Error> {
        let mut tagmap = TagMap::new();
        let tags = sqlx::query(
            r#"
SELECT * FROM tag_set
        "#)
        .fetch_all(&self.pool)
        .await;
        let tags = match tags {
            Ok(data) => data,
            Err(e) => return Err(Error::from(e)),
        };
        for row in tags {
            tagmap.insert_tag(row.get("id"), row.get("tag_name"))?;
        }
        Ok(tagmap)
    }

    async fn query_category_map(
        &self
    ) -> Result<CategoryMap, Error> {
        let mut categorymap = CategoryMap::new();
        let categories = sqlx::query(
            r#"
SELECT * FROM category_set
        "#)
        .fetch_all(&self.pool)
        .await;
        let categories = match categories {
            Ok(data) => data,
            Err(e) => return Err(Error::from(e)),
        };
        for row in categories {
            categorymap.insert_category(row.get("id"), row.get("category_name"))?;
        }
        Ok(categorymap)
    }
}