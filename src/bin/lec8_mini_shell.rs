// Минимальная оболочка на rust.
// В общем виде, оболочка это shell:
// 1) читает команду
// 2) запускает child process
// 3) настраивает stdin/stdout процесс
// 4) ждет завершения

// Программа blazing_shell:
//1. печатает prompt;
//2. читает строку из stdin;
//3. разбивает строку на команду и аргументы;
//4. запускает команду;
//5. ждёт завершения дочернего процесса;
//6. завершает работу по команде exit;
//

use std::io::{self, ErrorKind, Write};
use std::process::Command;

// Result<()> - результат операции, которая при успехе ничего полезного не возвращает,
// а при ошибке возвращает ошибку.
// () - это unit type, "ничего"
//
fn main() -> io::Result<()> {
    // используем бесконечный цикл, потому что shell должен постоянно показывать prompt, читать команду etc.
    loop {
        print!("blazzzing_shell> "); // использую print *(не println)
        io::stdout().flush()?;

        // это пустая строка, КУДА будет прочитан пользовательский ввод
        let mut line = String::new();
        // читаем одну строку из stdin
        // &mut line - передаем изменяемую ссылку на строку, чтобы функция могла записать туда данные
        let bytes_read = io::stdin().read_line(&mut line)?;

        // ctrl+D
        if bytes_read == 0 {
            break;
        }

        // сплитим строку по пробельным символам
        let mut line_parts = line.split_whitespace();

        // первая часть строки - это имя команды,
        let Some(command) = line_parts.next() else {
            // если команды нет - идем на след.итерацию
            continue;
        };

        if command == "exit" {
            break;
        };

        // собираем части строки в вектор аргументов
        let args: Vec<&str> = line_parts.collect();

        // создаем процессу с именем команды + применяем аргументы.
        // spawn - дочерний процесс
        match Command::new(command).args(args).spawn() {
            Ok(mut child) => match child.wait() {
                Ok(status) => {
                    // если процесс завершился с ненулевым статусом — выведем exit status;
                    if !status.success() {
                        eprintln!("{status}");
                    }
                }
                Err(err) => {
                    eprintln!("failed to wait for child process: {err}")
                }
            },

            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    // если команда не найдена
                    eprintln!("unknown_command");
                }
                _ => {
                    eprintln!("failed to start command: {err}");
                }
            },
        }
    }
    Ok(())
}
