#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum TaskState {
    TaskPending,
    TaskHeading,
    TaskHeaded,
    TaskDownloading,
    TaskError,
}

pub struct Task {
    url: String,
    parallel_num: u64,
    max_buffer_size: u64,
    task_header: Option<TaskHeader>,
    bulk_id: u64,
    task_state: TaskState,
}

impl Task {
    fn get_state(&self) -> TaskState {
        self.task_state
    }

    pub fn new(url: String, parallel_num: u64, max_buffer_size: u64) -> Self {
        return Task {
            url,
            parallel_num,
            max_buffer_size,
            task_header: None,
            bulk_id: 0,
            task_state: TaskState::TaskPending,
        };
    }

    async fn fetch_header(&mut self) {
        self.task_state = TaskState::TaskHeading;
        let cli = reqwest::Client::new();
        match cli.head(&self.url).send().await {
            Ok(response) => {
                let mut task_header = TaskHeader::from_header_map(response.headers());
                if task_header.content_length == 0 {
                    task_header.accept_ranges = false;
                }
                if task_header.location == "" {
                    task_header.location = self.url.clone();
                }
                self.task_state = TaskState::TaskHeaded;
            }
            Err(_) => self.task_state = TaskState::TaskError,
        }
    }
}

struct TaskHeader {
    accept_ranges: bool,
    content_length: u64,
    content_disposition: String,
    location: String,
}

impl TaskHeader {
    fn from_header_map(header_map: &HeaderMap) -> Self {
        let mut header = TaskHeader {
            accept_ranges: false,
            content_length: 0,
            content_disposition: String::from(""),
            location: String::from(""),
        };
        if let Some(val) = header_map.get("Accept-Ranges") {
            if let Ok(val) = val.to_str() {
                if val == "bytes" {
                    header.accept_ranges = true;
                }
            }
        }

        if let Some(val) = header_map.get("Content-Length") {
            if let Ok(val) = val.to_str() {
                if let Ok(val) = val.parse::<u64>() {
                    header.content_length = val;
                }
            }
        }
        if let Some(val) = header_map.get("Content-Disposition") {
            if let Ok(val) = val.to_str() {
                header.content_disposition = String::from(val);
            }
        }
        if let Some(val) = header_map.get("Location") {
            if let Ok(val) = val.to_str() {
                header.location = String::from(val);
            }
        }
        header
    }
}

use std::{sync::Arc, time::Duration};

use reqwest::header::HeaderMap;
use tokio::{runtime::Runtime, sync::Mutex, time::sleep};

pub fn download(task: Task, runtime: &Runtime) {
    let parallel_num = task.parallel_num;
    let task = Arc::new(Mutex::new(task));
    let count: u64 = 0;
    loop {
        let task = task.clone();
        runtime.spawn(async move {
            loop {
                let task = task.lock().await;
                if task.get_state() > TaskState::TaskHeading {
                    break;
                }
                sleep(Duration::from_millis(100)).await;
            }
            let task = task.lock().await;
        });
        if count >= parallel_num {
            break;
        }
    }

    runtime.spawn(async move {
        let mut task = task.lock().await;
        task.fetch_header().await;
    });
}
