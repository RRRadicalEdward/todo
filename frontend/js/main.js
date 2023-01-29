const task_input = document.querySelector('input');
const add_btn = document.querySelector('.add-task-button');
const todos_list = document.querySelector('.todos-list');
const delete_all_btn = document.querySelector('.delete-all-btn');


let todos = [];
class Todo {
    constructor(uuid, title) {
        this._uuid = uuid;
        this._title = title;
    }

    get title()
    {
        return this._title
    }

    get uuid()
    {
        return this._uuid;
    }
}

window.addEventListener('DOMContentLoaded', loadAllTodos);

function loadAllTodos()
{
    let url = 'http://127.0.0.1:8000/list';
    $.getJSON(url, function (data, status) {
        if(status) {
            data.forEach((todo) => {
                todos.push(new Todo(todo.uuid, todo.title));
            });
            showAllTodos();
        }
    });
}

function showAllTodos() {
    todos_list.innerHTML = '';
    todos.forEach((todo) => {
        todos_list.innerHTML += `
            <li class="todo-item" data-id="${todo.uuid}">
                <p class="task-body">
                    ${todo.title}
                </p>
                <div class="todo-actions">
                    <button class="btn btn-error" onclick="deleteTodo('${todo.uuid}');">
                        <i class="bx bx-trash bx-sm"></i>
                    </button>
                </div>
            </li>
        `;
    });
}

function deleteTodo(uuid) {
    deleteTodoImpl(uuid);
}

function deleteTodoImpl(uuid) {
    let url = 'http://127.0.0.1:8000/delete'
    let todo = JSON.stringify(todos.find((todo) => todo.uuid === uuid).uuid);

    $.post(url, todo, function (data, status) {
        if(status) {
            todos = todos.filter(todo => todo.uuid !== uuid);
            showAllTodos();
        }
    });
}

add_btn.addEventListener('click', () => {
    if (task_input.value !== '') {
        addToDo(task_input);
        task_input.value = '';
    }
});

function addToDo(task_input) {
    if(task_input.value === '') {
        return;
    }

    let title = task_input.value;
    let todo = JSON.stringify({
        "title": title,
    });
    let url = 'http://127.0.0.1:8000/create'
    $.post(url, todo, function (data, status) {
        if(status) {
            todos.push(new Todo(data, title));
            showAllTodos();
        }
    });
}

delete_all_btn.addEventListener('click', clearAllTodos);

function clearAllTodos() {
    if(todos.length > 0) {
        todos.forEach((todo) => {
            deleteTodoImpl(todo.uuid)
        });
    }
}