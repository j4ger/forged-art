use super::card::Card;

pub(crate) type PlayerID = usize;

#[derive(Debug, Clone, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[archive(check_bytes)]
pub(crate) struct Player {
    pub(crate) uuid: String,
    pub(crate) id: PlayerID,
    pub(crate) name: String,
    pub(crate) connected: bool,
    // TODO: statistics
}

