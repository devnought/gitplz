use color_printer::ColorPrinter;
use command::{Command, WorkResult, WorkType};
use std::{collections::BTreeMap, sync::mpsc::Receiver};
use threadpool::ThreadPool;

const THREAD_SIGNAL: &str = "Could not signal main thread with WorkType::Work";

pub struct Dispatcher<'a> {
    queue: BTreeMap<usize, Option<Box<dyn WorkResult>>>,
    next_index: usize,
    command: Box<dyn Command>,
    pool: &'a ThreadPool,
    printer: ColorPrinter<'a>,
}

impl<'a> Dispatcher<'a> {
    pub fn new(pool: &'a ThreadPool, printer: ColorPrinter<'a>, command: Box<dyn Command>) -> Self {
        Self {
            queue: BTreeMap::new(),
            next_index: 0,
            pool,
            command,
            printer,
        }
    }

    pub fn run(&mut self, rx: &Receiver<WorkType>) {
        while let Ok(result) = rx.recv() {
            match result {
                WorkType::Repo { index, repo, tx } => {
                    let worker = self.command.box_clone();
                    self.pool.execute(move || {
                        let result = match worker.process(repo) {
                            Some(r) => WorkType::result(index, r),
                            None => WorkType::empty(index),
                        };

                        tx.send(result).expect(THREAD_SIGNAL)
                    })
                }
                WorkType::WorkEmpty { index } => {
                    if index == self.next_index {
                        self.process_queue();
                    } else {
                        self.queue.insert(index, None);
                    }
                }
                WorkType::Work { index, result } => {
                    if self.next_index != index {
                        self.queue.insert(index, Some(result));
                        continue;
                    }

                    result.print(&mut self.printer);

                    // If there are adjacent items in the queue, process them.
                    self.process_queue();
                }
            }
        }

        // Sanity check
        if !self.queue.is_empty() {
            panic!(
                "There are {} unprocessed items in the queue. \
                 Did you forget to send WorkEmpty::WorkEmpty messages?",
                self.queue.len()
            );
        }
    }

    fn process_queue(&mut self) {
        self.next_index += 1;

        while let Some(result) = self.queue.remove(&self.next_index) {
            if let Some(work_result) = result {
                work_result.print(&mut self.printer);
            }

            self.next_index += 1;
        }
    }
}
