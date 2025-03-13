use warp::Filter;
use crate::blockchain::Block;

#[tokio::main]
async fn main() {
    let genesis_block = Block::new(0, "0".to_string());

    let routes = warp::path!("blocks").map(move || warp::reply::json(&genesis_block));
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
