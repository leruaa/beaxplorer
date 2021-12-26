use std::{convert::TryFrom, fs::File, io::BufWriter, sync::Arc};

use db::{models::EpochModel, ConnectionManager, PgConnection, Pool, RunQueryDsl};
use flate2::{write::ZlibEncoder, Compression};
use lighthouse_types::MainnetEthSpec;
use rmp_serde::Serializer;
use serde::Serialize;
use types::{meta::EpochsMeta, views::EpochView};

use crate::types::spec_epoch_model::SpecEpochModel;

pub struct Indexer {}

impl Indexer {
    pub fn persist_epochs(&self, pool: &Arc<Pool<ConnectionManager<PgConnection>>>) {
        let db_connection = pool.get().expect("Error when getting connection");
        let epochs = db::queries::epochs::all()
            .load::<EpochModel>(&db_connection)
            .unwrap();

        for model in &epochs {
            let view = EpochView::try_from(SpecEpochModel::<MainnetEthSpec>::new(model)).unwrap();
            let mut f = BufWriter::new(
                File::create(format!(
                    "../web_static/public/data/epochs/{}.msg",
                    view.epoch
                ))
                .unwrap(),
            );
            view.serialize(&mut Serializer::new(&mut f)).unwrap();
        }

        // meta
        let mut f =
            BufWriter::new(File::create("../web_static/public/data/epochs/meta.msg").unwrap());
        let meta = EpochsMeta::new(epochs.len());
        meta.serialize(&mut Serializer::new(&mut f)).unwrap();

        /*
        let mut i = 1;
        for chunk in epochs.chunks(10) {
            let f = BufWriter::new(
                File::create(format!("../web_static/public/data/epochs/page/{}.msg", i)).unwrap(),
            );
            let mut enc = ZlibEncoder::new(f, Compression::default());
            chunk.serialize(&mut Serializer::new(&mut enc)).unwrap();
            i = i + 1;
        }
        */
    }
}