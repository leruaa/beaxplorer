use crate::{
    path::{FromPath, ToPath},
    persistable::{ResolvablePersistable, SelfPersistable},
};

pub trait Meta {
    fn count(&self) -> usize;
}

impl<T: Meta + SelfPersistable + ToPath<Id = ()>> ResolvablePersistable for T {
    fn save(&self, base_path: &str) -> Result<(), String> {
        let full_path = T::to_path(base_path, &());
        self.persist(&full_path)
    }
}

pub trait WithMeta
where
    Self: Sized,
{
    type MetaType: FromPath<Id = ()>;

    fn meta(base_dir: &str) -> Result<Self::MetaType, String> {
        Self::MetaType::from_path(base_dir, &())
    }
}
