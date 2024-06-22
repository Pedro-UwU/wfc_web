<script>
  import ControlPane from "./control_pane.svelte";
  import GridCanvas from "./grid_canvas.svelte";
  import TileSelector from "./tile_selector.svelte";
</script>

<div class="wrapper">
  <div class="l-side">
    <div class="canvas-wrapper">
      <GridCanvas />
    </div>
    <TileSelector />
  </div>
  <div class="r-side">
    <ControlPane />
  </div>
</div>

<style>
  .wrapper {
    background: var(--bg);
    display: flex;
    width: 100%;
    max-height: 90vh;
    margin-top: 3vh;
    margin-bottom: 3vh;
    margin-inline: 1vw;
    border-radius: 0.5rem;
    box-shadow: 0px 10px 9px -3px rgba(0, 0, 0, 0.4);
  }

  .l-side {
    justify-content: center;
    flex-flow: column;
    padding-left: 1rem;
    max-width: 70vw;
    min-width: 70vw;
  }

  .r-side {
    min-width: 10%;
    flex: 1;
    padding-block: 1rem;
    padding-inline: 1rem; 
  }

  .canvas-wrapper {
    align-self: start;
    border-radius: 0.5rem;
    overflow: hidden;
    margin-block: 1rem;
  }

</style>
