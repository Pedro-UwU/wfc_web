<script>
    import UploadDefault from "../assets/upload.png";
    import { tiles_width, tiles_height } from "../stores/config_store.js";
    import { image, tiles } from "../stores/image_store.js";

    let image_title = "No image selected";
    let image_url = "";
    let img;
    let selected = false;
    let image_input;
    let to_delete;

    // @ts-ignore
    const handleUpload = async (e) => {
        let file = e.target.files[0];
        image_url = URL.createObjectURL(file);
        image_title = file.name;
        selected = true;
        $tiles = [];

        // Get dimensions
        let img_w;
        let img_h;
        try {
            let dimensions = await getImageWidthAndHeight(image_url);
            img_w = dimensions.width;
            img_h = dimensions.height;
        } catch (err) {
            console.log(err);
            return;
        }

        tiles.set(getTileArray(img, img_w, img_h));
    };

    const getImageWidthAndHeight = (objectURL) => {
        return new Promise((resolve, reject) => {
            let img = new Image();
            img.onload = () => {
                resolve({ width: img.naturalWidth, height: img.naturalHeight });
            };
            img.onerror = (err) => {
                reject(err);
            };
            img.src = objectURL;
        });
    };

    const getTileArray = (img, img_w, img_h) => {
        // Initialize tiles
        let total_cols = img_w / $tiles_width;
        let total_rows = img_h / $tiles_height;
        let temp_tiles = [];

        for (let i = 0; i < total_cols; i++) {
            for (let j = 0; j < total_rows; j++) {
                let aux_canvas = document.createElement("canvas");
                let aux_context = aux_canvas.getContext("2d");
                aux_canvas.width = $tiles_width;
                aux_canvas.height = $tiles_height;
                aux_context.drawImage(
                    img,
                    i * $tiles_width,
                    j * $tiles_height,
                    $tiles_width,
                    $tiles_height,
                    0,
                    0,
                    $tiles_width,
                    $tiles_height,
                );
                let img_data = aux_canvas.toDataURL();
                temp_tiles.push(img_data);
            }
        }
        return [...new Set(temp_tiles)];
    };

    const getImageBase64 = () => {
        let canvas = document.createElement("canvas");
        let context = canvas.getContext("2d");
        canvas.height = img.naturalHeight;
        canvas.width = img.naturalWidth;
        context.drawImage(img, 0, 0, canvas.height, canvas.width);
        let base64String = canvas.toDataURL();
        console.log(base64String);
    };
</script>

<div class="wrapper">
    <button class="img-wrapper" on:click={image_input.click()}>
        <img
            bind:this={img}
            class="img-container"
            src={selected ? image_url : UploadDefault}
            alt="selected tile set"
            on:change={getImageBase64}
        />
    </button>
    <div class={selected ? "img-title" : "unselected img-title"}>
        {image_title}
    </div>

    <input
        bind:this={image_input}
        type="file"
        id="input-file"
        style="display: none"
        on:change={(e) => handleUpload(e)}
        accept="image/png, image/jpeg"
    />
</div>
<div style="display: flex; width: 100%; max-width: 100vw">
{#each $tiles as t}
    <img src={t} alt="tile" style="border: 0.5px solid gray; width: 50px; height: 50px"/>
{/each}
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
