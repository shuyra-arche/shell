pub struct Port {
    id: u64,
}

impl Port {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }

    pub(crate) fn id(self) -> u64 {
        self.id
    }
}
