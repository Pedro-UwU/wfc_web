<script>
  import UploadDefault from "../assets/upload.png";
  let image = UploadDefault;
  let image_title = "No image selected";
  let selected = false;
  let image_input;
</script>

<div class="wrapper">
  <button class="img-wrapper" on:click={image_input.click()}>
    <img class="img-container" src={image} alt="selected tile set" />
  </button>
  <div class={selected ? "img-title" : "unselected img-title"}>
    {image_title}
  </div>

  <input bind:this={image_input} type="file" id="input-file" style="display: none" />
</div>

<style>

.wrapper {
  display: flex;
  width: 100%;
  justify-content: start;
  align-items: center;
}

.img-title {
  margin-left: 1em;
  flex: 1;
}

.unselected {
  color: #31313154;
}

.img-wrapper {
  width: 15%;
  overflow: hidden;
}

img {
  width: 100%;
}
</style>
