<!-- Dont look at this code. It is one of the ugliest code I have ever written. -->
<script>
  import { grid_width, grid_height } from "../stores/config_store.js";
  import { tiles, selected } from "../stores/image_store.js";
  import { onMount } from "svelte";
  import { Canvas, FabricImage, Rect } from "fabric";

  let CANVAS_WIDTH = 800;
  let CANVAS_HEIGHT = 600;

  let grid = [];
  let canvas;
  let overlay;
  let preview_image = null;
  const cell_width = 50;
  const cell_height = 50;
  let cell_x;
  let cell_y;

  const change_preview_image = () => {
    let img_elem = document.createElement("img");
    img_elem.src = $tiles[$selected];
    // If it contains "undefined", do nothing
    if (img_elem.src.includes("undefined")) {
      return;
    }

    preview_image = img_elem;
    // Change overlay image
    if (overlay) {
      overlay.setElement(preview_image);
    }
  };

  const update_grid = () => {
    const width = $grid_width;
    const height = $grid_height;
    const new_grid = Array(height)
      .fill()
      .map(() => Array(width).fill(0));
    for (let i = 0; i < height; i++) {
      for (let j = 0; j < width; j++) {
        let rect = new Rect({
          left: j * cell_width + (canvas.width - width * cell_width) / 2,
          top: i * cell_height + (canvas.height - height * cell_height) / 2,
          fill: "#313131",
          width: cell_width,
          height: cell_height,
          strokeWidth: 1,
          stroke: "#928374",
          selectable: false,
        });
        if (i > grid.length - 1) {
          new_grid[i][j] = { value: -1, canvas_elem: rect };
        } else if (j > grid[i].length - 1) {
          new_grid[i][j] = { value: -1, canvas_elem: rect };
        } else {
          new_grid[i][j] = { value: grid[i][j].value, canvas_elem: rect };
        }
      }
    }
    grid = new_grid;
    grid.forEach((row) => {
      row.forEach((cell) => {
        if (cell.value === -1) {
          canvas.add(cell.canvas_elem);
        }
      });
    });
  };

  const handle_zoom = (opt) => {
    let delta = opt.e.deltaY;
    let zoom = canvas.getZoom();
    zoom *= 0.999 ** delta;
    if (zoom > 20) zoom = 20;
    if (zoom < 0.1) zoom = 0.1;
    canvas.zoomToPoint({ x: opt.e.offsetX, y: opt.e.offsetY }, zoom);
    canvas.renderAll();

    opt.e.preventDefault();
    opt.e.stopPropagation();
  };

  const init_drag = (opt) => {
    // @ts-ignore
    let e = opt.e;
    if (e.ctrlKey === true || e.altKey == true) {
      canvas.is_dragging = true;
      canvas.selection = false;
      // @ts-ignore
      canvas.lastPosX = e.clientX;
      // @ts-ignore
      canvas.lastPosY = e.clientY;
    }
  };

  const drag_canvas = (opt) => {
    if (canvas.is_dragging) {
      // @ts-ignore
      let e = opt.e;
      // @ts-ignore
      let vpt = canvas.viewportTransform;
      // @ts-ignore
      vpt[4] += e.clientX - canvas.lastPosX;
      // @ts-ignore
      vpt[5] += e.clientY - canvas.lastPosY;
      canvas.requestRenderAll();
      // @ts-ignore
      canvas.lastPosX = e.clientX;
      // @ts-ignore
      canvas.lastPosY = e.clientY;
    }
    let target_x = canvas.getPointer(opt.e).x;
    let target_y = canvas.getPointer(opt.e).y;
    let min_x = (canvas.width - $grid_width * cell_width) / 2;
    let min_y = (canvas.height - $grid_height * cell_height) / 2;
    let max_x = min_x + $grid_width * cell_width;
    let max_y = min_y + $grid_height * cell_height;
    if (
      target_x < min_x ||
      target_x > max_x ||
      target_y < min_y ||
      target_y > max_y
    ) {
      if (overlay) {
        overlay.set({ left: 10000 });
        canvas.renderAll();
      }

      return;
    }
    if (!preview_image || preview_image.src === undefined) {
      return;
    }
    cell_x = Math.floor((target_x - min_x) / cell_width) * cell_width + min_x;
    cell_y = Math.floor((target_y - min_y) / cell_height) * cell_height + min_y;
    if (!overlay) {
      overlay = new FabricImage(preview_image, {
        left: cell_x,
        top: cell_y,
        width: cell_width,
        height: cell_height,
        scaleX: cell_width / preview_image.width,
        scaleY: cell_height / preview_image.height,
        opacity: 0.5,
        selectable: false,
        objectCaching: false,
        imageSmoothing: false,
      });
      overlay.set({ imageSmoothingEnabled: false });
      canvas.add(overlay);
      canvas.renderAll();
    } else {
      overlay.set({
        left: cell_x,
        top: cell_y,
        width: cell_width,
        height: cell_height,
        scaleX: cell_width / preview_image.width,
        scaleY: cell_height / preview_image.height,
        opacity: 0.5,
      });
      overlay.imageSmoothingEnabled = false;
      canvas.renderAll();
    }
  };

  const insert_tile = (opt) => {
    if (!preview_image || preview_image.src === undefined) {
      return;
    }
    let target_x = canvas.getPointer(opt.e).x;
    let target_y = canvas.getPointer(opt.e).y;
    let min_x = (canvas.width - $grid_width * cell_width) / 2;
    let min_y = (canvas.height - $grid_height * cell_height) / 2;
    let max_x = min_x + $grid_width * cell_width;
    let max_y = min_y + $grid_height * cell_height;
    if (
      target_x < min_x ||
      target_x > max_x ||
      target_y < min_y ||
      target_y > max_y
    ) {
      return;
    }
    let x = Math.floor((target_x - min_x) / cell_width);
    let y = Math.floor((target_y - min_y) / cell_height);

    let img_elem = new Image();
    img_elem.src = $tiles[$selected];

    if (grid[y][x].value === -1) {
      let tile_img = new FabricImage(img_elem, {
        left: x * cell_width + min_x,
        top: y * cell_height + min_y,
        width: cell_width,
        height: cell_height,
        scaleX: cell_width / img_elem.width,
        scaleY: cell_height / img_elem.height,
        selectable: false,
        imageSmoothing: false,
      });

      grid[y][x].canvas_elem = tile_img;
      canvas.add(grid[y][x].canvas_elem);
    } else {
      grid[y][x].canvas_elem.setElement(img_elem);
    }
    grid[y][x].value = $selected;

    canvas.renderAll();
  };

  onMount(() => {
    canvas = new Canvas("c");
    canvas.backgroundColor = "#928374";
    canvas.contextTop.imageSmoothingEnabled = false;
    canvas.contextContainer.imageSmoothingEnabled = false;
    canvas.imageSmoothingEnabled = false;

    // Calculate screen size
    let screen_width = window.innerWidth;
    let screen_height = window.innerHeight;

    // Make width be 80% of the screen width
    CANVAS_WIDTH = screen_width * 0.7;
    CANVAS_HEIGHT = screen_height * 0.7;
    canvas.setWidth(CANVAS_WIDTH);
    canvas.setHeight(CANVAS_HEIGHT);

    update_grid();

    canvas.on("mouse:wheel", function (opt) {
      handle_zoom(opt);
    });

    canvas.on("mouse:down", function (opt) {
      init_drag(opt);
      if (!canvas.is_dragging) {
        insert_tile(opt);
      }
    });

    canvas.on("mouse:move", function (opt) {
      drag_canvas(opt);
    });

    canvas.on("mouse:up", function (opt) {
      canvas.setViewportTransform(canvas.viewportTransform);
      canvas.is_dragging = false;
      canvas.selection = true;
    });

    const unsubscribe_width = grid_width.subscribe(update_grid);
    const unsubscribe_height = grid_height.subscribe(update_grid);
    const unsubscribe_selected = selected.subscribe(change_preview_image);

    return () => {
      unsubscribe_width();
      unsubscribe_height();
      unsubscribe_selected();
      canvas.dispose();
    };
  });
</script>

<!-- Change the grid width and height -->

<canvas id="c" width={CANVAS_WIDTH} height={CANVAS_HEIGHT} />
