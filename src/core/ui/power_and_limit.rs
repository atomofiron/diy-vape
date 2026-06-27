use strum::EnumIs;

#[derive(Clone, PartialEq, EnumIs)]
pub enum Edit {
    None,
    Power,
    Limit,
}
