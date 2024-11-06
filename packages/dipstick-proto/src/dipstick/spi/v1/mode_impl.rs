use super::SpiMode;

impl SpiMode {
    pub const fn cpol(self) -> bool {
        matches!(self, SpiMode::SpiMode2 | SpiMode::SpiMode3)
    }

    pub const fn cpha(self) -> bool {
        matches!(self, SpiMode::SpiMode1 | SpiMode::SpiMode3)
    }

    pub const fn from_cpol_cpha(cpol: bool, cpha: bool) -> Self {
        match (cpol, cpha) {
            (false, false) => SpiMode::SpiMode0,
            (false, true) => SpiMode::SpiMode1,
            (true, false) => SpiMode::SpiMode2,
            (true, true) => SpiMode::SpiMode3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpol_cpha_consistent() {
        for &mode in &[
            SpiMode::SpiMode0,
            SpiMode::SpiMode1,
            SpiMode::SpiMode2,
            SpiMode::SpiMode3,
        ] {
            assert_eq!(mode, SpiMode::from_cpol_cpha(mode.cpol(), mode.cpha()));
        }
    }
}
