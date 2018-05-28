use std::{collections::BTreeMap, sync::mpsc::Receiver};

use printer::Printer;
use process::Processor;
use worktype::{WorkResult, WorkType};

pub struct Dispatcher<'a> {
    queue: BTreeMap<usize, Option<WorkResult>>,
    next_index: usize,
    processor: &'a Processor<'a>,
    printer: &'a Printer<'a>,
}

impl<'a> Dispatcher<'a> {
    pub fn new(processor: &'a Processor, printer: &'a Printer) -> Self {
        Self {
            processor,
            printer,
            queue: BTreeMap::new(),
            next_index: 0,
        }
    }

    pub fn run(&mut self, rx: &Receiver<WorkType>) {
        while let Ok(result) = rx.recv() {
            match result {
                WorkType::Repo { index, repo, tx } => self.processor.repo(tx, index, repo),
                WorkType::WorkEmpty { index } => {
                    if index == self.next_index {
                        self.process_queue();
                    } else {
                        self.queue.insert(index, None);
                    }
                }
                WorkType::Work { index, message } => {
                    if self.next_index != index {
                        self.queue.insert(index, Some(message));
                        continue;
                    }

                    self.printer.handle(&message);

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
            if let Some(message) = result {
                self.printer.handle(&message);
            }

            self.next_index += 1;
        }
    }
}
