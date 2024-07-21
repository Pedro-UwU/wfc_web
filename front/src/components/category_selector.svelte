<script>
  export let title = "Title";
  let opened = false;
</script>

<div class="title">
  <div class="expand-btn">
    <svg class={opened ? 'rotated' : ''} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"
      ><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path
        d="M137.4 374.6c12.5 12.5 32.8 12.5 45.3 0l128-128c9.2-9.2 11.9-22.9 6.9-34.9s-16.6-19.8-29.6-19.8L32 192c-12.9 0-24.6 7.8-29.6 19.8s-2.2 25.7 6.9 34.9l128 128z"
      /></svg
    >
  </div>
  <p>{title}</p>
  {#if opened}
  <p class="help-title">(Click to remove)</p>
  {/if}
</div>
<div class="content {opened ? '' : 'no-h'}">
</div>

<style>
  .title {
    display: flex;
    
  }
  .expand-btn {
    width: var(--text-medium);
    height: var(--text-medium);
    fill: var(--text-dim);
    cursor: pointer;
  }

  .expand-btn svg {
    transform: rotate(0deg);
    transition: transform 0.3s ease-in;
  }

  .rotated {
    transform: rotate(180deg);
  }

  .no-h {
    height: 0px;
  }
</style>
