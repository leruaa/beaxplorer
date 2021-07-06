use dotenv::dotenv;

pub mod epoch_retriever;
pub mod types;
pub mod errors;

fn main() {
    dotenv().ok();
}
