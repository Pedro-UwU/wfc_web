<script>
    import { onMount } from "svelte";
    import { image_width, image_height } from "../stores/image_store";
    import { grid, grid_width, grid_height, tiles_width, tiles_height, result_width, result_height } from "../stores/config_store";
    import { get_divisors } from "../lib/utils.js";
    import { create_graph } from "../lib/create_graph.js";
    import { send_graph } from "../lib/api.js";

    let possible_tiles_width = [];
    let possible_tiles_height = [];
    let select_width;
    let select_height;

    onMount(() => {
        let unsuscribed_img_w = image_width.subscribe((val) => {
            possible_tiles_width = get_divisors(val).reverse();
            tiles_width.set(possible_tiles_width[0]);
        });
        let unsuscribed_img_h = image_height.subscribe((val) => {
            possible_tiles_height = get_divisors(val).reverse();
            tiles_height.set(possible_tiles_height[0]);
        });

        return () => {
            unsuscribed_img_w();
            unsuscribed_img_h();
        };
    });

    const updateWidth = (e) => {
        console.log("Updating Tiles Width");
        tiles_width.set(e.target.value);
    };

    const updateHeight = (e) => {
        console.log("Updating Tiles Height");
        tiles_height.set(e.target.value);
    };

    const handleSend = (_) => {
        let values = $grid.map((arr) => {
            let row = arr.map((val) => val.value);
            return row;
        });
        let graph = create_graph(values);
        // console.log(create_graph(values));
        send_graph($result_width, $result_height, graph)
    };

    const handleSave = (_) => {
        let dataStr = JSON.stringify(get_save_json());

        const blob = new Blob([dataStr], { type: "application/json" }); 
        const url = URL.createObjectURL(blob);

        const a = document.createElement("a");
        a.href = url;
        a.download = "data.json";
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url); 
    };

    const get_save_json = () => {
        let values = $grid.map((arr) => {
            let row = arr.map((val) => val.value);
            return row;
        });
        let dataToSave = {
            gw: $grid_width,
            gh: $grid_height,
            tw: $tiles_width,
            th: $tiles_height,
            grid: values
        }
        return dataToSave;
    };
</script>

<div class="options-wrapper">
    <div class="option-select">
        <label for="tiles-width">Tiles Width:</label>
        <select
            bind:this={select_width}
            name="tiles-width"
            on:change={(e) => updateWidth(e)}
            id="tiles-width"
        >
            {#each possible_tiles_width as p}
                <option value={p}>{p}</option>
            {/each}
        </select>
    </div>
    <div class="option-select">
        <label for="tiles-width">Tiles Height:</label>
        <select
            bind:this={select_height}
            name="tiles-height"
            on:change={(e) => updateHeight(e)}
            id="tiles-height"
        >
            {#each possible_tiles_height as p}
                <option value={p}>{p}</option>
            {/each}
        </select>
    </div>

    <div class="send-button">
        <button on:click={(e) => handleSend(e)}>Send</button>
    </div>

    <div class="save-button">
        <button on:click={(e) => handleSave(e)}>Save</button>
    </div>
</div>
