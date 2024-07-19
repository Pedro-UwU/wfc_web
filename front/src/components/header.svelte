<script>
  import Button from "../ui/button.svelte";

  let tileset_name = "Tileset Name";
  let tileset_width = 800;
  let tileset_height = 800;
  let tiles_width = 16;
  let tiles_height = 16;
</script>

<div class="header">
  <div class="img-uploader">
    <!-- plus symbol -->
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
      <path
        d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 144L48 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l144 0 0 144c0 17.7 14.3 32 32 32s32-14.3 32-32l0-144 144 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-144 0 0-144z"
      />
    </svg>
  </div>
  <div class="tileset-info">
    <h2>{tileset_name}</h2>
    <div class="info-list">
      <li>Tileset Width: {tileset_width}</li>
      <li>Tileset Height: {tileset_height}</li>
      <li>Tile Width: {tiles_width}</li>
      <li>Tile Height: {tiles_height}</li>
    </div>
  </div>
  <div class="btn-col">
    <div class="btn-container">
      <Button text="SEND" type="accent" onClick={() => console.log("Hola")} width="wide" size="big"/>
    </div>
  </div>
</div>

<style>
  .header {
    background: var(--background-dim);
    height: calc(4 * var(--text-giant));
    display: flex;
    align-items: center;
    border-radius: var(--border-radius-m);
  }

  .img-uploader {
    background: var(--secondary-shade);
    height: calc(4*var(--text-giant) - 2*(var(--margin-l)));
    width: calc(4*var(--text-giant) - 2*(var(--margin-l)));
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: var(--border-radius-m);
    margin: var(--margin-l);
  }

  .tileset-info {
    flex: 1;
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .info-list {
    display: flex;
    flex-flow: column;
    height: 100%;
    margin-bottom: var(--margin-l);
    justify-content: space-around;
    color: var(--text-dim);
  }

  .btn-col {
    height: 100%;
    display: flex;
    flex-flow: column;
    justify-content: end;
  }

  .btn-container {
    margin-inline: var(--margin-l);
    margin-bottom: var(--margin-l);
    justify-content: end;
  }

  svg {
    width: 50%;
    height: 50%;
    fill: var(--primary);
  }

  li {
    list-style-type: none;
  }

</style>
