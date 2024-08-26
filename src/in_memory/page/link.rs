use rkyv::{Archive, Deserialize, Serialize};

use crate::in_memory::page;

pub const LINK_LENGTH: usize = 12;

#[derive(
    Archive, Copy, Clone, Deserialize, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Link {
    pub page_id: page::Id,
    pub offset: u32,
    pub length: u32,
}

#[cfg(test)]
mod tests {
    use crate::in_memory::page::link::LINK_LENGTH;
    use crate::in_memory::page::Link;

    #[test]
    fn link_length_valid() {
        let link = Link {
            page_id: 1.into(),
            offset: 10,
            length: 20,
        };
        let bytes = rkyv::to_bytes::<_, 16>(&link).unwrap();

        assert_eq!(bytes.len(), LINK_LENGTH)
    }
}