use std::{fs::File, io::BufWriter, sync::Arc};

use db::{models::EpochModel, ConnectionManager, PgConnection, Pool, RunQueryDsl};
use rmp_serde::Serializer;
use serde::Serialize;

pub struct Indexer {}

impl Indexer {
    pub fn persist_epochs(&self, pool: &Arc<Pool<ConnectionManager<PgConnection>>>) {
        let db_connection = pool.get().expect("Error when getting connection");
        let epochs = db::queries::epochs::all()
            .load::<EpochModel>(&db_connection)
            .unwrap();

        for e in epochs {
            let mut f = BufWriter::new(
                File::create(format!("../web_static/public/data/epochs/{}.msg", e.epoch)).unwrap(),
            );
            e.serialize(&mut Serializer::new(&mut f)).unwrap();
        }
    }
}
