<script lang="ts">
  import { onMount } from "svelte";
  
  interface Todo {
    id: number;
    description: string;
    done: boolean;
  }

  let todos: Todo[] = [];

  // Fetch todos from the backend
  async function fetchTodos() {
    try {
      const response = await fetch("http://localhost:8000/");
      if (!response.ok) throw new Error("Failed to fetch todos");
      todos = await response.json();
    } catch (error) {
      console.error("Error fetching todos:", error);
    }
  }

  // Create new todo
  async function createTodo(event: Event) {
    event.preventDefault();
    const form = event.target as HTMLFormElement;
    const formData = new FormData(form);
    
    await fetch("http://localhost:8000/create", {
      method: "POST",
      body: formData,
    });

    form.reset();
    await fetchTodos(); // Refresh list
  }

  // Update todo
  async function updateTodo(todo: Todo) {
    await fetch("http://localhost:8000/update", {
      method: "POST",
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      body: new URLSearchParams({
        id: todo.id.toString(),
        description: todo.description,
        done: todo.done.toString(),
      }),
    });

    await fetchTodos(); // Refresh list
  }

  // Delete todo
  async function deleteTodo(id: number) {
    await fetch(`http://localhost:8000/delete/${id}`, { method: "POST" });
    await fetchTodos(); // Refresh list
  }

  // Fetch todos when the component loads
  onMount(fetchTodos);
</script>

<div class="container mx-auto mt-16">
  <h1 class="h1 text-center">Todos</h1>

  <div class="max-w-screen-md mx-auto">
    <form on:submit={createTodo}>
      <input
        class="input p-4 my-8 w-full"
        name="description"
        type="text"
        placeholder="What needs to be done?"
        autocomplete="off"
        required
      />
      <button type="submit" class="btn variant-filled-primary">Add Todo</button>
    </form>

    <div class="space-y-4">
      {#each todos as todo}
        <div class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4">
          <input
            class="checkbox"
            type="checkbox"
            bind:checked={todo.done}
            on:change={() => updateTodo(todo)}
          />
          <input
            class="input"
            type="text"
            bind:value={todo.description}
            disabled={todo.done}
          />

          <div class="flex gap-2">
            <button class="btn variant-filled-secondary" on:click={() => updateTodo(todo)}>Update</button>
            <button class="btn variant-filled-primary" on:click={() => deleteTodo(todo.id)}>Delete</button>
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>
