use super::Task;

pub enum Message {
  NewTask(Task),
  Terminate,
}
