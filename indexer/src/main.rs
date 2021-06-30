use dotenv::dotenv;

pub mod types;
pub mod epoch_retriever;

fn main() {
    dotenv().ok();
}
