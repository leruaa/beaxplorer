use crate::path::FromPath;

pub trait Meta {
    fn count(&self) -> usize;
}

pub trait WithMeta
where
    Self: Sized,
{
    type MetaType: FromPath<Self::MetaType, ()>;

    fn meta(base_dir: &str) -> Self::MetaType {
        Self::MetaType::from_path(base_dir, &())
    }
}
