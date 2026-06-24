#![deny(unsafe_code)]

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self},
};

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

pub type Task = fn(i64);

struct Shared {
    // <> -  в rust это дженерик типы (как в го [T])
    state: Mutex<State>,
    // Condvar: (аналог sync.Cond в гошке) "событие для активации: очередь не пустая ИЛИ начался shutdown"
    has_work: Condvar,
}

struct State {
    // Если использовать Vec<i64> и pop(), задачи будут выполняться в порядке LIFO.
    // В этом задании FIFO-порядок не требуется.
    // Важно только, чтобы каждая задача была выполнена ровно один раз.
    queue: VecDeque<i64>, // очередь с задачами (настоящая FIFO очередь)
    shutting_down: bool,
}

pub struct ThreadPool {
    // ARC - указатель для разделяемого владения между потоками.
    // Хранит значение в куче и считает, сколько владельцев сейчас на него ссылаются.
    // Когда последний владелец исчезает, значение автоматически освобождается.
    shared: Arc<Shared>,
    workers: Vec<thread::JoinHandle<()>>,
    // заккоментила, потому что ругается компилятор: field `task` is never read
    // task: Task,
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    pub fn new(worker_count: usize, task: Task) -> Self {
        // todo!("create shared queue, spawn workers, and return ThreadPool")
        if worker_count == 0 {
            panic!("worker count cannot be less than 0")
        }

        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                queue: VecDeque::new(),
                shutting_down: false,
            }),
            has_work: Condvar::new(),
        });

        let mut workers = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            // создаем ссылку для каждого потока
            let shared = Arc::clone(&shared); // ARC увеличивает счетчик ссылок

            // создание потока (task копируем в каждый worker во время создания потока)
            // воркер хранит копию
            let handle = thread::spawn(move || {
                worker_processing(shared, task);
            });

            workers.push(handle);
        }

        return ThreadPool {
            shared,
            workers,
            // task,
        };
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        //todo!("push a task argument into the queue and notify one worker")

        // добавление задачи
        let mut state = self.shared.state.lock(). // берем мьютекс на стейт
            expect("mutex poisoned"); // текст ошибки, который будет показан при панике
        if state.shutting_down {
            panic!("cannot execute task after shutdown started");
        }

        state.queue.push_back(num);

        let condvar = &self.shared.has_work;
        condvar.notify_one(); // notify_one будит одного воркера
    }

    /// Finish all queued work and stop all workers.
    /// запрещает дальнейшую работу, будит всех воркеров и делает join каждого worker-потока
    /// задачи должны быть выполнены до выхода воркеров.
    /// все отправленные задачи должны быть выполнены один раз.
    pub fn shutdown(self) {
        // self - забираем владение над всем ThreadPool
        // todo!("set shutdown flag, notify all workers, and join them")

        // Деструктуризация структуры (pattern matching для struct).
        // Чтобы shared, workers, task стали локальными переменными
        let ThreadPool {
            shared,
            workers,
            // task: _, // или _task - опускаем переменную, мы ее не используем
        } = self;

        {
            let mut state = shared.state.lock().expect("mutex poisoned");
            state.shutting_down = true;
            // после выхода из } переменная state уничтожается и mutex разлочивается
        }
        // mutex освбожден, чтобы workers смогли отработать, а не дожидаться mutex
        // вместо {} можно использовать drop(state);
        // А NLL может помочь???

        // Будим всех: кто спит на пустой очереди, увидит shutting_down == true.
        // Кто проснется и увидит задачи — сначала выполнит их.
        shared.has_work.notify_all();

        for worker in workers {
            // join - "присоединяет каждого воркера к потоку", чтобы корректно завершить пул
            worker.join().expect("worker thread panicked");
        }
    }
}

fn worker_processing(shared: Arc<Shared>, task: Task) {
    loop {
        let maybe_num = {
            let mut state = shared.state.lock().expect("mutex poisoned");

            while state.queue.is_empty() && !state.shutting_down {
                // wait атомарно запускает mutex и засыпает.
                // Mutex одновременно может держать только один воркер.
                state = shared.has_work. // получаем convdvar
                    wait(state).// держим mutex и имеем доступ к state
                    expect("mutex poisoned"); // текст ошибки, который будет показан при панике
            }

            // если задача есть - взять ее, если задач нет и не shutting_down - ждать на condvar
            if let Some(num) = state.queue.pop_front() {
                Some(num)
            } else {
                None
            }
        };

        match maybe_num {
            // task вызываем уже без mutex, чтобы не блокировать всю очередь надолго
            Some(num) => task(num),
            None => break,
        };
    }
}
