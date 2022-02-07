use serde::Serialize;

pub struct ModelWithId<M: Serialize + Send> {
    pub id: u64,
    pub model: M,
}
