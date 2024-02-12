use crate::*;

pub async fn run_line(line: &str, tl: &mut TodoList) {
    if let Ok((_, q)) = parser::query(line) {
        match run_query(q, tl).await {
            Ok(r) => {
                println!("{}", r);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

async fn run_query(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(_desc, _tags) => {
            let new_item = tl.push(_desc, _tags).await;
            Ok(QueryResult::Added(new_item))
        }
        Query::Done(idx) => {
            if let Some(_) = tl.done_with_index(idx).await {
                Ok(QueryResult::Done)
            } else {
                Err(QueryError("Index not found.".to_string()))
            }
        }
        Query::Search(params) => {
            let found_items = tl.search(params).await;
            Ok(QueryResult::Found(found_items))
        }
    }
}
