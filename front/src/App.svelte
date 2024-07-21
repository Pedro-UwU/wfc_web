<script>
  import CategoriesSync from "./components/categories_sync.svelte";

  // import Button from "./ui/button.svelte";
  import Header from "./components/header.svelte";
  import TileCard from "./components/tile_card.svelte";
  import TileSelector from "./components/tile_selector.svelte";
  import TileSplicer from "./components/tile_splicer.svelte";
</script>

<main>
  <h1>Wave Function Collapse</h1>
  <TileSplicer />
  <Header />
  <div class="tiles">
    <div class="tile-selector-wrapper">
      <TileSelector />
    </div>
    <div class="tile-card-wrapper">
      <TileCard />
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

  h1 {
    margin-bottom: 0px;
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

  .tile-card-wrapper {
    min-width: calc(8 * var(--text-giant));
    margin-left: var(--margin-l);
    height: 100%;
  }
</style>
