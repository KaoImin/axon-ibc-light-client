use ibc::core::ics02_client::height::Height;

#[derive(Debug, Clone, Copy)]
pub struct AxonHeight(u64);

impl AxonHeight {
    pub fn new(h: u64) -> Self {
        Self(h)
    }

    pub fn prev(&self, h: u64) -> Self {
        if self.0 > h {
            Self(self.0 - h)
        } else {
            panic!("height can not be negative")
        }
    }
}

impl From<u64> for AxonHeight {
    fn from(h: u64) -> Self {
        Self(h)
    }
}

impl From<AxonHeight> for Height {
    fn from(h: AxonHeight) -> Self {
        Height::new(0, h.0 - 1).unwrap()
    }
}
