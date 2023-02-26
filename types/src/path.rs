pub trait ToPath<Id> {
    fn to_path(base_dir: &str, id: Id) -> String;
}
