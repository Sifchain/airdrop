use utils::db::*;
use utils::memo::*;

// Used to process and clean memo's that have already been downloaded and have random new encoding issues

#[tokio::main]
async fn main() {
    let db = connect().await.unwrap();

    let records = sqlx::query!(
        r#"
        SELECT * 
        FROM txs 
    "#
    )
    .fetch_all(&db)
    .await
    .unwrap();

    println!("total records: {}", records.len());
    // println!("records: {:#?}", records);

    for record in records {
        // println!("record: {:#?}", record);
        match record.memo {
            Some(v) => {
                println!("v: {}", v);
                let memo = process_memo(&v);
                match sqlx::query!(
                    r#"
                        UPDATE txs
                        SET twitter_handle = $1, sif_address = $2
                        WHERE id = $3
                        RETURNING id  
                    "#,
                    memo.handle,
                    memo.address,
                    record.id,
                )
                .fetch_one(&db)
                .await
                {
                    Ok(_) => println!("Record updated"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            None => println!("None"),
        }
    }
}
