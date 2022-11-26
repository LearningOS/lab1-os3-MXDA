//! Types related to task management

use super::TaskContext;
pub use crate::config::MAX_SYSCALL_NUM;
pub use alloc::vec::Vec;
#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub first_running_time: usize,
    //pub syscall_times: Vec<u32>,
    // LAB1: Add whatever you need about the Task.
}
#[derive(Clone)]
pub struct TaskSysCallTimes {
    pub syscall_times: Vec<u32>,
}
#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
