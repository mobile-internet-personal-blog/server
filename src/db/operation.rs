use std::collections::HashMap;

use crate::{api::login_api::ThirdPartyProvider, error::{Error, Result}, model::{CategoryMap, Essay, EssayInfo, EssayList, Message, MessageList, TagMap}, Uuid};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::Row;
use super::connection::Database;


impl Database {
    /// 得到所有文章的 eid
    pub async fn query_alleid (
        &self,
    ) -> Result<Vec<Uuid>> {
        let eid_list = sqlx::query(
            r#"
SELECT eid FROM essays
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        let mut eids = Vec::new();
        for eid in eid_list {
            let eid = eid.get("eid");
            eids.push(eid);
        }
        Ok(eids)
    }

    /// 得到文章列表
    pub async fn query_essaylist (
        &self,
    ) -> Result<EssayList> {
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
            let essay = Essay::new(essayinfo, &content, created_at, updated_at);
            essaylist.create_essay(essay).await.expect("5");
        }

        Ok(essaylist)
    }

    /// 得到 Tag Map
    pub async fn query_tag_map(
        & self
    ) -> Result<TagMap> {
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

    /// 得到 Category Map
    async fn query_category_map(
        &self
    ) -> Result<CategoryMap> {
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

    /// 新增一个用户
    pub async fn create_user (
        &self,
        open_id: &str,
        third_party_provider: ThirdPartyProvider,
        access_token: &str
    ) -> Result<()> {
        let third_party_provider = match third_party_provider {
            ThirdPartyProvider::Github => "Github",
        };
        sqlx::query(
            r#"
INSERT INTO users (open_id, third_party_provider, access_token)
VALUES (?, ?, ?)
            "#
        )
        .bind(open_id)
        .bind(third_party_provider)
        .bind(access_token)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 更新用户信息
    pub async fn update_user (
        &self,
        uid: &Uuid,
        access_token: &str
    ) -> Result<()> {
        sqlx::query(
            r#"
UPDATE users 
SET access_token = ?, updated_at = current_timestamp()
WHERE uid = ?
            "#
        )
        .bind(access_token)
        .bind(uid)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 查询用户的 uid
    pub async fn query_uid (
        &self,
        open_id: &str,
        third_party_provider: &ThirdPartyProvider,
    ) -> Result<Uuid> {
        let third_party_provider = match third_party_provider {
            ThirdPartyProvider::Github => "Github",
        };
        let uid: Uuid = sqlx::query_scalar(
            r#"
SELECT uid FROM users WHERE open_id = ? AND third_party_provider = ?
            "#
        )
        .bind(open_id)
        .bind(third_party_provider)
        .fetch_one(&self.pool)
        .await?;

        Ok(uid)
    }

    /// 新增一条评论
    pub async fn create_remark (
        &self,
        eid: &Uuid,
        uid: &Uuid,
        content: &str
    ) -> Result<()> {
        sqlx::query(
            r#"
INSERT INTO remarks (eid, uid, content)
VALUES (?, ?, ?)
            "#
        )
        .bind(eid)
        .bind(uid)
        .bind(content)
        .execute(&self.pool)
        .await?;
    
        Ok(())
    }

    /// 新增一条留言
    pub async fn create_message(
        &self,
        uid: &Uuid,
        content: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
INSERT INTO chat_messages (uid, content)
VALUES (?, ?)
            "#
        )
        .bind(uid)
        .bind(content)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 得到评论列表
    pub async fn query_remark_list (
        &self,
        eid: &Uuid
    ) -> Result<MessageList> {
        let msgs: Vec<Message> = sqlx::query_as::<_, Message> (
            r#"
SELECT uid, content, created_at from remarks WHERE eid = ?
            "#
        )
        .bind(eid)
        .fetch_all(&self.pool)
        .await?;
        let msgs = MessageList::from_vec(msgs).await?;
        Ok(msgs)
    }
    /// 得到所有文章的评论列表
    pub async fn query_all_remark_lists (
        &self,
    ) -> Result<HashMap<Uuid, MessageList>> {
        let mut remark_lists = HashMap::new();
        for eid in self.query_alleid().await? {
            remark_lists.insert(eid.clone(), self.query_remark_list(&eid).await?);
        }
        Ok(remark_lists)
    }

    /// 得到留言列表
    pub async fn query_message_list (
        &self
    ) -> Result<MessageList> {
        let msgs: Vec<Message> = sqlx::query_as::<_, Message> (
            r#"
SELECT uid, content, created_at from chat_messages
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        let msgs = MessageList::from_vec(msgs).await?;
        Ok(msgs)
    }

}