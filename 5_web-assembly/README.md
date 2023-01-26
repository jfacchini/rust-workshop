# Implementing TodoMVC

This folder contains the source files for the [TodoMVC](https://todomvc.com/) example, you
can see a running example following [this link](https://todomvc.com/examples/react/#/).

To start the project locally, please run the following command:

```sh
trunk serve
```

This command is watching all the files in the project and will recompile whenever a file is saved.

## Extract components

- TodoList: Component that receives a list of todo and renders a list of TodoDetails Component
- TodoDetails: Component that display a given todo to render. A Todo can be either Viewed, Completed, Editing
- NewTodoInput: Component that display an input that is used to create a new Todo

## Load a list of static todos from a state

You could use a state struct like the following one. It will be simpler to pass it down
to a child component.

```rust
struct TodoListState {
    todos: Vec<Todos>
}
```

## Add a new Todo

Implement a callback to a `onkeypress` event triggered by the `<input />` element.
This callback should emit a `NewInputEntered` event that should be listened to by the parent element.
The parent element listens to the event using a Callback and update the `TodoListState` 
