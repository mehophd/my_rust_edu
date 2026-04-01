trait BlockDevice {
    fn read_sector(&self, sector: u64, buf: &mut [u8]) -> Result<(), Error>;
    fn write_sector(&self, sector: u64, buf: &[u8]) -> Result<(), Error>;
    fn sector_size(&self) -> u32;
}

// Реализация для NVMe диска
impl BlockDevice for NvmeDisk {
    fn read_sector(&self, sector: u64, buf: &mut [u8]) -> Result<(), Error> {
        // NVMe-специфичная реализация
    }
    // ...
}

// Реализация для виртуального диска
impl BlockDevice for VirtualDisk {
    fn read_sector(&self, sector: u64, buf: &mut [u8]) -> Result<(), Error> {
        // RAM-диск реализация
    }
    // ...
}

// Общий код ядра работает с ЛЮБЫМ устройством через трейт
fn format_filesystem<T: BlockDevice>(device: &T) -> Result<(), Error> {
    let size = device.sector_size();
    // ... форматирование работает с любым устройством
}