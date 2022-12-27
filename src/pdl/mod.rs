// pub struct Task {
//     url: String,
//     split_num: u8,
//     max_buffer_size: u64,
// }

// impl Task {
//     // default rule
//     pub fn new<S: AsRef<str>>(url: S, split_num: u8, max_buffer_size: u64) -> Self {
//         Task {
//             url: String::from(url.as_ref()),
//             split_num,
//             max_buffer_size,
//         }
//     }
// }

// use reqwest::{self, header::HeaderMap};

// pub enum Event {
//     OnError(reqwest::Error),
//     OnSuccess(),
// }

// struct TaskInfo {
//     content_length: u64,
//     content_disposition: String,
//     location: String,
// }

// fn parse_task(h: &HeaderMap) -> Option<TaskInfo> {
//     match h.get("Accept-Ranges") {
//         None => return None,
//         Some(val) => match val.to_str() {
//             Ok(val) => {
//                 if val != "bytes" {
//                     return None;
//                 }
//             }
//             Err(_) => {}
//         },
//     }
//     let mut task_info = TaskInfo {
//         content_length: 0,
//         content_disposition: String::new(),
//         location: String::new(),
//     };
//     match h.get("Content-Length") {
//         None => return None,
//         Some(val) => match val.to_str() {
//             Ok(val) => match val.parse::<u64>() {
//                 Ok(val) => task_info.content_length = val,
//                 Err(_) => return None,
//             },
//             Err(_) => return None,
//         },
//     }
//     match h.get("Content-Disposition") {
//         None => return None,
//         Some(val) => match val.to_str() {
//             Ok(val) => task_info.content_disposition = String::from(val),
//             Err(_) => return None,
//         },
//     }
//     match h.get("Location") {
//         None => return None,
//         Some(val) => match val.to_str() {
//             Ok(val) => task_info.location = String::from(val),
//             Err(_) => return None,
//         },
//     }
//     Some(task_info)
// }

// use tokio::{runtime::Runtime, task::JoinHandle};

// async fn download_segment(
//     cli: reqwest::Client,
//     url: &String,
//     begin: u64,
//     end: u64,
// ) -> Result<reqwest::Response, reqwest::Error> {
//     cli.get(url)
//         .header("Range", format!("bytes={}-{}", begin, end))
//         .send()
//         .await
// }

// use std::{sync::Arc, time::Duration};

// struct TaskState {
//     state: u8,
//     task_info: Option<TaskInfo>,
//     task_id: u64,
// }

// struct Bulk {}

// impl TaskState {
//     pub fn new() -> TaskState {
//         return TaskState {
//             state: 0,
//             task_info: None,
//             task_id: 0,
//         };
//     }

//     pub fn submit_task(&mut self, task_info: TaskInfo) {
//         self.task_info = Some(task_info);
//         self.state = 1;
//     }

//     pub fn get_state(&self) -> u8 {
//         return self.state;
//     }

//     pub fn get_next_bulk(&mut self) -> Option<Bulk> {
//         let info = self.task_info.as_ref().unwrap();
//         let id = self.task_id;
//         self.task_id = self.task_id + 1;
        
//     }
// }

// pub fn download(task: Task, event_handler: fn(Event), rt: &Runtime) -> JoinHandle<()> {
//     let arc = Arc::new(tokio::sync::Mutex::new(TaskState::new()));
//     let mut count: u8 = 0;
//     loop {
//         let st = arc.clone();
//         rt.spawn(async move {
//             loop {
//                 {
//                     let st = st.lock().await;
//                     if st.get_state() != 0 {
//                         break;
//                     }
//                 }
//                 tokio::time::sleep(Duration::from_millis(100)).await;
//             }
//             loop {}
//         });
//         count = count + 1;
//         if count >= task.split_num {
//             break;
//         }
//     }
//     rt.spawn(async move {
//         let st = arc.clone();
//         let cli = reqwest::Client::new();
//         match cli.head(task.url).send().await {
//             Ok(response) => match parse_task(response.headers()) {
//                 None => return,
//                 Some(task_info) => {
//                     let mut st = st.lock().await;
//                     st.submit_task(task_info);
//                 }
//             },
//             Err(err) => {
//                 event_handler(Event::OnError(err));
//                 return;
//             }
//         }
//     })
// }

// pub mod default {
//     pub const DEFAULT_SPLIT_NUM: u8 = 4; // default 4 connection
//     pub const DEFAULT_MAX_BUFFER_SIZE: u64 = 128 * 1024 * 1024 * 1024; // default max memory 128m

//     pub struct Task {}

//     impl Task {
//         pub fn new<S: AsRef<str>>(url: S) -> super::Task {
//             super::Task::new(url, DEFAULT_SPLIT_NUM, DEFAULT_MAX_BUFFER_SIZE)
//         }
//     }

//     pub fn download(task: super::Task) {
//         // super::download(task, pool::get());
//     }

//     mod pool {
//         use once_cell::sync::Lazy;
//         use tokio::runtime::Runtime;

//         static RT: Lazy<Runtime> = Lazy::new(|| {
//             let rt = Runtime::new().unwrap();
//             rt
//         });

//         pub(crate) fn get() -> &'static Lazy<Runtime> {
//             &RT
//         }
//     }
// }
mod tests;
mod task;