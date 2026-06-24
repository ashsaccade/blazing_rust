// Можно использовать:
// std::fs::File
// std::io::{Read, Write}
//
// Нельзя использовать:
// std::io::BufReader
// std::io::BufWriter
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
    vec,
};

pub const BUFFER_SIZE: usize = 64 * 1024; // 64 * 1024 bytes = 65536 bytes = 64 KiB

// -----------------------------------------------------------------------------
// MyBufReader
// -----------------------------------------------------------------------------

#[allow(dead_code)]
pub struct MyBufReader {
    file: File,    // храним сам файл
    buf: Vec<u8>,  // внтутренний буффер, куда прочитаем большой кусок файла (дженерник тип)
    pos: usize,    // текущая позиция внутри buf, с которой будем отдавать даные покусочно
    filled: usize, // сколько байт в buf заполнено реальными данными
}

// Как должен работать MyBufReader
// read_byte() должен вернуть один байт пользователю.
// Но внутри он не должен каждый раз читать один байт из файла.
// Правильная идея:
// -> Если во внутреннем буфере ещё есть данные — вернуть следующий байт из буфера.
// -> Если буфер пуст — прочитать из файла большой блок, например 64 KiB.
// -> Если файл закончился — вернуть Ok(None).
//
// блок реализации для структуры MyBufReader + ассоциированные функции структуры MyBufReader
impl MyBufReader {
    // функция, которая относятся к этому типу (MyBufReader). Открывает файл по пути.
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        // знак вопроса ? перед ; - это оператор раннего возврата ошибки (вместо if err != nil)
        let file = File::open(path)?;

        Ok(Self {
            file,
            buf: vec![0; BUFFER_SIZE],
            pos: 0,
            filled: 0,
        })
    }

    // * метод read_byte структуры MyBufReader (потому что есть &mut self)
    // функция чтения одного байта через внутренний буффер
    pub fn read_byte(&mut self) -> io::Result<Option<u8>> {
        // Если буффер закончился - читаем из файла большой блок
        if self.pos >= self.filled {
            self.filled = self.file.read(&mut self.buf)?; // кладем в буфер даннные из файла
            self.pos = 0; // новые данные лежат начиная с индекса 0

            // read вернул 0 byte - значит файл закончился
            if self.filled == 0 {
                return Ok(None);
            }
        }

        // если буффер еще есть -> берем байт, который отдадим наружу
        let byte = self.buf[self.pos];
        self.pos += 1;

        Ok(Some(byte))
    }
}

// -----------------------------------------------------------------------------
// MyBufWriter
// -----------------------------------------------------------------------------

pub struct MyBufWriter {
    file: File, // файл, куда будем писать данные
    // внтутренний буффер, куда прочитаем большой кусок пользовательских данных, но еще не записали в файл
    buf: Vec<u8>,
}

// Как должен работать MyBufWriter
// write_buffered() должен принять данные от пользователя.
// Но он не должен сразу писать каждый маленький кусок в файл.
// Правильная идея:
//  -> Скопировать данные во внутренний буфер.
//  -> Если буфер заполнился — записать его в файл одним большим блоком.
//   - flush() должен записать в файл всё, что осталось во внутреннем буфере.
//   - close() должен вызвать flush().
impl MyBufWriter {
    // открываем файл для записи
    pub fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::create(path)?;

        Ok(Self {
            file,
            buf: Vec::with_capacity(BUFFER_SIZE), // аллоцируем место под буффер
        })
    }

    // data: &[u8] - ссылка на слайс байтов. data не владеет данными, а
    // только смотрит на существующий участок участок памяти
    pub fn write_buffered(&mut self, data: &[u8]) -> io::Result<()> {
        // позиция внутри пользовательского среза data (сколько бойт мы скопировали во внутренний буффер)
        let mut offset = 0; // храним сам индекс

        // while - как цикл for len(data) > 0 в golang. Обрабатываем все байты из data
        while offset < data.len() {
            let buffer_free_space = BUFFER_SIZE - self.buf.len(); // получаем сколько места осталось в буфере

            // если буфер уже заполнен — записываем его в файл
            if buffer_free_space == 0 {
                self.flush()?;
                continue;
            }

            // сколько байт осталось обработать в пользовательском data
            let remaining_data = data.len() - offset;

            // сколько байт из data мы можем положить в буфер
            // надо выбрать между "осталось свободного места в буффере" и "осталось необработанных байт в data"
            let bytes_to_copy = buffer_free_space.min(remaining_data);

            // копируем все элементы слайса в вектор (делаем срез)
            self.buf
                // читаем из исходного data "кусками"
                // .. - синтаксис диапазона (от offset до offset + bytes_to_copy (не включает правую границу))
                .extend_from_slice(&data[offset..offset + bytes_to_copy]);

            // увеличиваем offset, потому что bytes_to_copy мы уже скопировали
            offset += bytes_to_copy;

            // если мы заполнили внутренний буффер - сразу пишем в файл и флашим!
            if self.buf.len() == BUFFER_SIZE {
                self.flush()?;
            }
        }
        Ok(())
    }

    //эта функция должна записать в файл все данные, которые сейчас лежат во внутреннем буфере
    pub fn flush(&mut self) -> io::Result<()> {
        // если буффер не пустой, запишем содержимое буфера в файл
        if !self.buf.is_empty() {
            self.file.write_all(&self.buf)?; // write_all - запишем все байты из self.buf
            // очищаем буффер
            self.buf.clear();
        }

        // сбрасываем буфер самого File
        self.file.flush()?;

        Ok(())
    }

    pub fn close(mut self) -> io::Result<()> {
        self.flush()
    }
}

impl Drop for MyBufWriter {
    fn drop(&mut self) {
        // Ошибку из Drop вернуть нельзя.
        // Поэтому в реальном коде лучше явно вызывать close() или flush().
        let _ = self.flush();
    }
}

//______________________________________________________________________________
//
// -----------------------------------------------------------------------------
// Медленная версия
// -----------------------------------------------------------------------------

pub fn copy_slow(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut input = File::open(input)?;
    let mut output = File::create(output)?;

    let mut copied = 0;
    let mut byte = [0u8; 1];

    loop {
        let n = input.read(&mut byte)?;
        if n == 0 {
            break;
        }

        output.write_all(&byte[..n])?;
        copied += n as u64;
    }

    output.flush()?;

    Ok(copied)
}

// -----------------------------------------------------------------------------
// Быстрая версия
// -----------------------------------------------------------------------------
// copy_fast специально тоже использует побайтный API.
// Разница должна быть не в коде копирования, а в реализации MyBufReader и MyBufWriter
// эту функцию не нужно менять, она должна работать с любыми реализациями MyBufReader и MyBufWriter,
// которые вы сделаете
pub fn copy_fast(input: impl AsRef<Path>, output: impl AsRef<Path>) -> io::Result<u64> {
    let mut reader = MyBufReader::open(input)?;
    let mut writer = MyBufWriter::create(output)?;

    let mut copied = 0;

    while let Some(byte) = reader.read_byte()? {
        writer.write_buffered(&[byte])?;
        copied += 1;
    }

    writer.close()?;

    Ok(copied)
}

pub const RECORD_SIZE: usize = 10;

pub fn make_record(index: usize) -> [u8; RECORD_SIZE] {
    let mut record = [0u8; RECORD_SIZE];

    (0..RECORD_SIZE).for_each(|i| {
        record[i] = ((index + i) % 251) as u8;
    });

    record
}

pub fn generate_input_file(path: impl AsRef<Path>, records: usize) -> io::Result<()> {
    let mut file = File::create(path)?;

    for i in 0..records {
        let record = make_record(i);
        file.write_all(&record)?;
    }

    file.flush()?;

    Ok(())
}
