<script>
  import UploadDefault from "../assets/upload.png";
  let image_title = "No image selected";
  let image_url = "";
  let selected = false;
  let image_input;

  let handleUpload = (e) => {
    let file = e.target.files[0];
    image_url = URL.createObjectURL(file);
    image_title = file.name;
    selected = true
  }


  </script>

<div class="wrapper">
  <button class="img-wrapper" on:click={image_input.click()}>
    <img class="img-container" src={selected ? image_url : UploadDefault} alt="selected tile set" />
  </button>
  <div class={selected ? "img-title" : "unselected img-title"}>
    {image_title}
  </div>

  <input bind:this={image_input} type="file" id="input-file" style="display: none" on:change={e => handleUpload(e)}  accept="image/png, image/jpeg"/>
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
  all: unset;
  width: 15%;
  overflow: hidden;
  aspect-ratio: 1/1;
  display: flex;
  justify-items: center;
  align-items: center;
}

img {
  height: 100%;
}
</style>
