<script>
// import Button from "./ui/button.svelte";
import Header from "./components/header.svelte";
    import TileSelector from "./components/tile_selector.svelte";
    import TileSplicer from "./components/tile_splicer.svelte";
</script>

<main>
  <h1>Wave Function Collapse</h1>
  <TileSplicer/>
  <Header/>
  <div class="tiles">
    <div class="tile-selector-wrapper">
      <TileSelector/>
    </div>
  </div>
</main>

<style>
  main {
    display: flex;
    flex-flow: column;
    margin: var(--margin-l);
    max-height: calc(100vh - 2 * var(--margin-l));
    min-height: calc(100vh - 2 * var(--margin-l));
  }
  .tiles {
    margin-top: var(--margin-l);
    flex: 1 1 auto;
    display: flex;
    width: 100%;
    height: 0;
  }

  .tile-selector-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }
</style>
