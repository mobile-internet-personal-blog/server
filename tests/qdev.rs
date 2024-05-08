#![allow(unused)]

use anyhow::Result;
use axum::Json;
use serde_json::json;

#[tokio::test]
async fn qdev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8216")?;

    // hc.do_get("/").await?.print().await?;

    // let req_basicinfo = hc.do_get(
    //     "/api/basicinfo",
    // );

    // req_basicinfo.await?.print().await?;
    
    // let req_essayinfolist = hc.do_get(
    //     "/api/essayinfolist",
    // );
    
    // req_essayinfolist.await?.print().await?;

    // let req_queryessaycontent = hc.do_get(
    //     "/api/queryessaycontent?eid=6b4357a0-a2fe-483f-9196-c0c9bca9dbd7"
    // );

    // req_queryessaycontent.await?.print().await?;

    // let req_login = hc.do_post(
    //     "/api/login", 
    //     json!({
    //         "code": "3829e8bb786f3817e818",
    //         "third_party_provider": "Github"
    //     })
    // );
    
    // req_login.await?.print().await?;

    let req_create_remark = hc.do_post(
        "/api/createremark", 
        json!({
            "eid": "4d93cdf6-0993-4477-8be3-04e4d5b3ef2e",
            "open_id" : "97720243",
            "third_party_provider": "Github",
            "content" : "test"
        })
    );

    let req_create_chat = hc.do_post(
        "/api/createchatmsg", 
        json!({
            "open_id" : "97720243",
            "third_party_provider": "Github",
            "content" : "test"
        })
    );

    req_create_chat.await?.print().await?;
    req_create_remark.await?.print().await?;


    let req_get_remarklist = hc.do_get(
        "/api/remarklist?eid=4d93cdf6-0993-4477-8be3-04e4d5b3ef2e",
    );

    let req_get_chatmsglist = hc.do_get(
        "/api/chatmsglist"
    );

    req_get_chatmsglist.await?.print().await?;
    req_get_remarklist.await?.print().await?;
    

    Ok(())
}