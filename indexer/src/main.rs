use dotenv::dotenv;

pub mod epoch_retriever;
pub mod types;

fn main() {
    dotenv().ok();
}
