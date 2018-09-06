use super::{AttributeKind, DataKind};

#[derive(Debug, Clone, Hash)]
pub struct GLAttribute {
    kind: AttributeKind,
    count: usize,
    item_count: usize,
    item_kind: DataKind,
    location: usize,
}

impl GLAttribute {
    #[inline]
    pub fn new(kind: AttributeKind, count: usize, location: usize) -> Self {
        let (item_count, item_kind) = kind.item_data();

        GLAttribute {
            kind,
            count,
            item_count,
            item_kind,
            location,
        }
    }

    #[inline(always)]
    pub fn kind(&self) -> AttributeKind {
        self.kind
    }
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline(always)]
    pub fn item_count(&self) -> usize {
        self.item_count
    }
    #[inline(always)]
    pub fn item_kind(&self) -> DataKind {
        self.item_kind
    }
    #[inline(always)]
    pub fn location(&self) -> usize {
        self.location
    }
}
