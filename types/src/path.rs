use serde::de::DeserializeOwned;

pub trait ToPath<Id> {
    fn prefix() -> String;

    fn to_path(base_dir: &str, id: &Id) -> String;
}

impl<T, Id> ToPath<Id> for Option<T>
where
    T: ToPath<Id>,
{
    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &Id) -> String {
        T::to_path(base_dir, id)
    }
}

impl<T, Id> ToPath<Id> for Vec<T>
where
    T: ToPath<Id>,
{
    fn prefix() -> String {
        T::prefix()
    }

    fn to_path(base_dir: &str, id: &Id) -> String {
        T::to_path(base_dir, id)
    }
}

pub trait FromPath<Id, M> {
    fn from_path(base_dir: &str, id: &Id) -> M;
}

impl<Id, M> FromPath<Id, M> for M
where
    M: ToPath<Id> + DeserializeOwned,
{
    fn from_path(base_dir: &str, id: &Id) -> Self {
        let path = Self::to_path(base_dir, id);
        let file = std::fs::File::open(path).unwrap();
        rmp_serde::from_read::<_, M>(file).unwrap()
    }
}
