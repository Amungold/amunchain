pub struct EnumRegistry {
    pub entries: heapless::Vec<EnumEntry, 64>,
}

pub struct EnumEntry {
    pub name: heapless::String<32>,
    pub variant_count: u8,
    pub version: u32,
}

impl EnumRegistry {
    pub fn new() -> Self {
        Self {
            entries: heapless::Vec::new(),
        }
    }

    pub fn register(&mut self, name: &str, variant_count: u8, version: u32) -> Result<(), &'static str> {
        let mut n = heapless::String::new();
        n.push_str(name).map_err(|_| "name overflow")?;
        self.entries
            .push(EnumEntry {
                name: n,
                variant_count,
                version,
            })
            .map_err(|_| "registry full")
    }

    pub fn validate(&self, name: &str, variant: u8, version: u32) -> bool {
        self.entries.iter().any(|e| {
            e.name.as_str() == name && e.variant_count > variant && e.version <= version
        })
    }
}
