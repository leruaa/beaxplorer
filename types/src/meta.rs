use crate::path::FromPath;

pub trait Meta {
    fn count(&self) -> usize;
}

pub trait WithMeta
where
    Self: Sized,
{
    type MetaType: FromPath<Id = (), Model = Self::MetaType>;

    fn meta(base_dir: &str) -> Result<Self::MetaType, String> {
        Self::MetaType::from_path(base_dir, &())
    }
}
