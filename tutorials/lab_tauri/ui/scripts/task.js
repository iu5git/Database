const { invoke } = window.__TAURI__.tauri;

export function createTask(description, isDone, dueTime, createdBy) {
    const task = invoke('create_task', { description, done: isDone, dueTime, createdBy });
    return task;
}

export function getUserTasks(userId) {
    const tasks = invoke('get_user_tasks', { userId });
    return tasks;
}

export function addNewTask(task) {
    const task_id = invoke('add_task', { task });
    return task_id;
}

export function setTaskDone(taskId, done) {
    return invoke('set_task_done', { taskId, done });
}

export function removeTask(taskId) {
    return invoke('remove_task', { taskId });
}