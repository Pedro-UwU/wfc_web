<script>
  import { categories } from "../stores/categories_store.js";
  import Button from "../ui/button.svelte";

  export let show_modal; // It is false
  let dialog;
  let name_input;

  $: if (dialog && show_modal) {
    dialog.showModal();
  } else if (dialog && !show_modal && dialog.hasAttribute("open")) {
    dialog.close();
  }

  const add_category = (name) => {
    const cats = $categories;
    if (!cats.includes(name)) {
      cats.push(name);
    }
    categories.set(cats);
    console.log($categories);
  };

  const delete_category = (name) => {
    const cats = $categories;
    const index = cats.indexOf(name);
    if (index > -1) {
      cats.splice(index, 1);
      categories.set(cats);
    }

  }
</script>

{#if show_modal}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
  <dialog
    bind:this={dialog}
    on:close={() => (show_modal = false)}
    on:click|self={() => dialog.close()}
  >
    <h2>Categories</h2>
    {#if $categories.length === 0}
      <div class="no-categories">
        <h2>No categories</h2>
      </div>
    {:else}
      <div class="category-list">
        {#each $categories as cat}
        <div class="category-row">
          <p>{cat}</p>
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <div on:click={() => delete_category(cat)} class="category-row-action" title="Delete">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"
              ><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path
                d="M135.2 17.7L128 32 32 32C14.3 32 0 46.3 0 64S14.3 96 32 96l384 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-96 0-7.2-14.3C307.4 6.8 296.3 0 284.2 0L163.8 0c-12.1 0-23.2 6.8-28.6 17.7zM416 128L32 128 53.2 467c1.6 25.3 22.6 45 47.9 45l245.8 0c25.3 0 46.3-19.7 47.9-45L416 128z"
              /></svg
            >
          </div>
        </div>
        {/each}
      </div>
    {/if}
    <div class="category-add">
      <input
        bind:this={name_input}
        type="text"
        placeholder="Enter new category name"
      />
      <Button
        size="big"
        type="accent"
        text="ADD CATEGORY"
        width="content"
        onClick={() => {
          if (!name_input.value) {
            return;
          }
          add_category(name_input.value);
          name_input.value = "";
        }}
      />
    </div>
  </dialog>
{/if}

<style>
  .no-categories {
    width: calc(0.75 * var(--modal-width));
    max-height: calc(0.75 * var(--modal-width));
    height: calc(0.75 * var(--modal-width));
    border-radius: var(--border-radius-l);
    display: flex;
    justify-content: center;
    align-items: center;
    background: var(--background);
    color: var(--text-dim-50);
  }

  .category-list {
    width: calc(0.75 * var(--modal-width));
    max-height: calc(0.75 * var(--modal-width));
    height: calc(0.75 * var(--modal-width));
    overflow-y: auto;
    border-radius: var(--border-radius-l);
    background: var(--background);
  }

  .category-row {
    width: calc(100% - 2 * var(--margin-m));
    background: var(--secondary);
    border-radius: var(--border-radius-s);
    margin: var(--margin-m);
    display: flex;
    align-items: center;
  }

  .category-row p {
    flex: 1;
  }

  .category-row-action {
    height: var(--text-medium);
    width: var(--text-medium);
    display: flex;
    justify-content: center;
    align-items: center;
    margin-inline: var(--margin-m);
    cursor: pointer;
  }

  .category-row-action:hover {
    filter: brightness(1.1);
  }

  .category-row-action {
    fill: var(--text);
  }

  .category-add {
    margin-top: var(--margin-l);
    display: flex;
    width: 100%;
    justify-content: start;
  }

  .category-add input {
    flex: 1;
    margin-right: var(--margin-m);
    max-width: 100% !important;
  }

  dialog {
    max-width: var(--modal-width);
    border-radius: var(--border-radius-l);
    background: var(--background-dim);
    border: 0;
    color: var(--text);
    display: flex;
    flex-flow: column;
    align-items: center;
  }

  dialog::backdrop {
    background: rgba(0, 0, 0, 0.75);
  }

  dialog[open] {
    animation: zoom 0.3s ease-in-out;
  }

  @keyframes zoom {
    from {
      transform: scale(0.95);
    }
    to {
      transform: scale(1);
    }
  }

  dialog[open]::backdrop {
    animation: fade 0.3s ease-in-out;
  }

  @keyframes fade {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }
</style>
