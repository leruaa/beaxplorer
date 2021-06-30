use ::types::Epoch;

pub mod types;


fn main() {
    import_epoch(Epoch::new(10))
}

fn import_epoch(epoch: Epoch) {
    println!("{:?}", epoch)
}