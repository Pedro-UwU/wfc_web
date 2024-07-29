<script>
  import Button from "../ui/button.svelte";
  import { build } from "../lib/socketio.js";
    import { tiles_params } from "../stores/tiles_store";
  export let show_modal; // It is false
  let dialog;
  let result_width;
  let result_height;
  let error_msg;

  const send_request = () => {
    console.log(result_width, result_height);
  };

  $: if (dialog && show_modal) {
    dialog.showModal();
  } else if (dialog && !show_modal && dialog.hasAttribute("open")) {
    dialog.close();
  }

  const handle_build = () => {
    let tiles = $tiles_params;
    let result = build(result_width, result_height, tiles);
    if (result !== "") {
      error_msg.innerText = result
      error_msg.classList.remove('invisible');
      console.log(error_msg.classList);
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
    <h2>Generate</h2>
    <form on:submit|preventDefault={() => send_request()}>
      <div class="field">
        <label for="width">Result Width:</label>
        <input type="number" id="width" bind:value={result_width} required />
      </div>
      <div class="field">
        <label for="height">Result Height:</label>
        <input type="number" id="height" bind:value={result_height} required />
      </div>
      <small class="invisible" bind:this={error_msg}></small>
      <div class="buttons">
        <Button
          size="big"
          type="secondary"
          onClick={() => dialog.close()}
          text="CANCEL"
        />
        <Button onClick={() => handle_build()} size="big" type="accent" text="GENERATE" />
      </div>
    </form>
  </dialog>{/if}

<style>
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

  .buttons {
    margin-top: var(--margin-l);
  }

  .field {
    display: flex;
    justify-content: space-between;
    margin-block: var(--margin-l);
  }

  .invisible {
    display: none;
  }

  small {
    color: var(--error);
  }
</style>
