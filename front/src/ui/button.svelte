<script>
  export let size = "medium"; // big - medium - small
  export let type = "accent"; // accent - primary - secondary
  export let width = "wide"; // limitless - wide - thin - content
  export let onClick = () => {}; 
  export let text = "BUTTON";
  export let disabled = false;
</script>

<button on:click={onClick} class="{size} {width} {type} {disabled ? 'disabled' : 'enabled'}" {disabled}>
  {text}
</button>

<style>
  button {
    all: unset;
    padding-inline: var(--margin-l);
    border-radius: var(--border-radius-s);
    font-size: var(--text-small);
    color: var(--text);
    text-align: center;
    text-justify: auto;
    filter: brightness(100%);
    transition: background 0.1s ease-in-out;
    transition: filter 0.1s ease-in-out;
  }

  button:hover {
    filter: brightness(110%);
  }

  .big {
    height: var(--button-big);
  }

  .medium {
    height: var(--button-medium);
  }

  .small {
    height: var(--button-small);
  }

  .accent {
    background: var(--accent);
  }

  .primary {
    background: var(--primary);
  }

  .secondary {
    background: var(--secondary);
  }

  .limitless {
    width: 100%;
  }

  .wide {
    width: calc(3 * var(--button-big));
  }

  .thin {
    width: calc(3 * var(--button-medium));
  }

  .disabled {
    background: var(--text-dim-50);
    color: var(--text-dim);
    cursor: not-allowed;
  }

  .enabled {
    cursor: pointer;
  }


</style>


