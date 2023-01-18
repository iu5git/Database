import { getUserTasks, removeTask, setTaskDone } from "./task.js";

const tasksList = document.getElementById('tasksList');

export function setElementVisible(element, visible) {
    element.hidden = !visible;
}

export async function showTasksList(userId) {
    const tasks = await getUserTasks(userId);
    tasksList.replaceChildren();

    async function checkboxChanged(e) {
        const taskId = parseInt(e.target.name);

        await setTaskDone(taskId, e.target.checked);
        await showTasksList(userId);
    }

    async function deleteClicked(e) {
        const taskId = parseInt(e.target.name);

        await removeTask(taskId);
        await showTasksList(userId);
    }

    for (const task of tasks) {
        const taskNode = document.createElement('div');

        const checkboxNode = document.createElement('input');
        checkboxNode.type = 'checkbox';
        checkboxNode.name = task.id;
        checkboxNode.checked = task.done;
        checkboxNode.onchange = checkboxChanged;

        const descriptionNode = document.createElement('b');
        descriptionNode.innerHTML = `"${task.description}"`;

        const dueTimeNode = document.createElement('div');
        dueTimeNode.innerHTML = `Due time: ${task.due_time}`;

        const deleteButton = document.createElement('button');
        deleteButton.textContent = 'Delete';
        deleteButton.name = task.id;
        deleteButton.onclick = deleteClicked;

        taskNode.insertBefore(deleteButton, taskNode.lastChild);
        taskNode.insertBefore(dueTimeNode, deleteButton);
        taskNode.insertBefore(descriptionNode, dueTimeNode);
        taskNode.insertBefore(checkboxNode, descriptionNode);

        tasksList.insertBefore(taskNode, tasksList.lastChild);
    }
}