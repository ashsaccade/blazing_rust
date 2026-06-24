// Задание:
// 1) нужна функция, которая выполнит перевод виртуального адреса в физический.
// pub fn translate(&self, va: u8, is_write: bool) -> Result
// va - 6-битный виртуальный адрес.
// is_write - флаг операции
fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    PageNotPresent,     // если флаг P = 0
    WriteProtected,     // при попытке записи, если флаг W = 0
    PrivilegeViolation, // если CPL=Ring3, а флаг U = 0
    InvalidAddress,     // если переданный адрес > 63 (выходит за 6 бит)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    Ring0, // kernel-space (ядро)
    Ring3, // user-space (пользовательский код)
}

#[allow(dead_code)]
pub struct ToyMMU {
    page_table: [u8; 8],
    current_privilege: PrivilegeLevel,
}

impl ToyMMU {
    pub fn new(page_table: [u8; 8], privilege: PrivilegeLevel) -> Self {
        Self {
            page_table,
            current_privilege: privilege,
        }
    }

    /// Метод перевода Виртуального адреса (VA) в Физический адрес (PA)
    ///
    /// # Аргументы
    /// * `va` - 6-битный виртуальный адрес
    /// * `is_write` - флаг, указывающий, является ли операция записью (true) или чтением (false)
    pub fn translate(&self, virtual_addr: u8, is_write: bool) -> Result<u8, MemoryError> {
        // TODO: Реализовать логику трансляции и проверок флагов

        const PAGE_SIZE: u8 = 8; // 8 byte
        // маска, чтобы получить младшие три бита
        const OFFSET_MASK: u8 = PAGE_SIZE - 1; // 0b000_111
        const MAX_VA: u8 = 0b111_111; // 63

        // старшие 3 бита — номер физической страницы, а младшие 3 бита — флаги
        const PRESENT: u8 = 0b001;
        const WRITABLE: u8 = 0b010;
        const USER: u8 = 0b100;

        // 1. Проверить адрес
        if virtual_addr > MAX_VA {
            return Err(MemoryError::InvalidAddress);
        }

        // 2. Получить номер виртуальной страницы
        let virtual_page_number = virtual_addr >> 3; // сдвигаем адрес вправо, чтобы отбросить offset

        // 3. Получить смещение внутри страницы
        let offset = virtual_addr & OFFSET_MASK; // достаем младшие три бита адреса

        // 4. Найти запись в таблице страниц
        // Читаем запись в page_table (состоит из 8 записей)
        let page_table = self.page_table[virtual_page_number as usize]; // as - индекс массива в rust в usize

        // 5. Проверить флаг Present (присутствует ли страница в памяти: P = 1 - страница есть)
        if page_table & PRESENT == 0 { // оставляем только младший бит
            return Err(MemoryError::PageNotPresent);
        }

        // 6. Проверить права доступа (запись). W = 0 - страница только на чтение
        if is_write && page_table & WRITABLE == 0 {
            return Err(MemoryError::WriteProtected);
        }

        // 6. Проверить права доступа (user/kernel)
        if self.current_privilege == PrivilegeLevel::Ring3 && page_table & USER == 0 {
            return Err(MemoryError::PrivilegeViolation);
        }

        // 7. Получить номер физической страницы
        let physical_frame_number = page_table >> 3;

        // 8. Собрать физический адрес
        // physical address = physical page number + offset
        let pa = (physical_frame_number << 3) | offset;

        Ok(pa)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Индекс 0: Физическая стр 2, Флаги: U=1, W=1, P=1 (0b010_111 = 23) -> Доступно всем для всего
    // Индекс 1: Физическая стр 5, Флаги: U=0, W=1, P=1 (0b101_011 = 43) -> Только для Ядра (Ring0)
    // Индекс 2: Физическая стр 1, Флаги: U=1, W=0, P=1 (0b001_101 = 13) -> Только для чтения
    // Индекс 3: Физическая стр 0, Флаги: U=0, W=0, P=0 (0b000_000 = 0)  -> Страница отсутствует
    fn setup_test_table() -> [u8; 8] {
        [23, 43, 13, 0, 0, 0, 0, 0]
    }

    #[test]
    fn test_successful_translation() {
        let mmu = ToyMMU::new(setup_test_table(), PrivilegeLevel::Ring3);
        // VA = 3 (0b000_011): Стр 0, Смещение 3.
        // Физическая стр в таблице = 2 (0b010). Физический адрес должен быть: 0b010_011 = 19
        assert_eq!(mmu.translate(3, false), Ok(19));
    }

    #[test]
    fn test_invalid_address() {
        let mmu = ToyMMU::new(setup_test_table(), PrivilegeLevel::Ring3);
        assert_eq!(mmu.translate(64, false), Err(MemoryError::InvalidAddress));
        assert_eq!(mmu.translate(100, false), Err(MemoryError::InvalidAddress));
    }

    #[test]
    fn test_page_not_present() {
        let mmu = ToyMMU::new(setup_test_table(), PrivilegeLevel::Ring3);
        // VA = 24 (0b011_000): Стр 3, Смещение 0. У стр 3 флаг P = 0
        assert_eq!(mmu.translate(24, false), Err(MemoryError::PageNotPresent));
    }

    #[test]
    fn test_write_protection() {
        let mmu = ToyMMU::new(setup_test_table(), PrivilegeLevel::Ring3);
        // VA = 16 (0b010_000): Стр 2, Смещение 0. У стр 2 флаг W = 0 (Только чтение)
        assert!(mmu.translate(16, false).is_ok()); // Читать можно
        assert_eq!(mmu.translate(16, true), Err(MemoryError::WriteProtected)); // Писать нельзя
    }

    #[test]
    fn test_privilege_violation_in_ring3() {
        let mmu = ToyMMU::new(setup_test_table(), PrivilegeLevel::Ring3);
        // VA = 8 (0b001_000): Стр 1, Смещение 0. У стр 1 флаг U = 0 (Ядро)
        assert_eq!(
            mmu.translate(8, false),
            Err(MemoryError::PrivilegeViolation)
        );
    }

    #[test]
    fn test_kernel_can_access_everything() {
        let mmu = ToyMMU::new(setup_test_table(), PrivilegeLevel::Ring0);
        // В Ring0 ядро должно успешно читать страницу, защищенную флагом U=0
        // VA = 11 (0b001_011): Стр 1, Смещение 3.
        // Физическая стр в таблице = 5 (0b101). Физический адрес: 0b101_011 = 43
        assert_eq!(mmu.translate(11, false), Ok(43));
    }
}
