<script>
  import { grid_width, grid_height } from "../stores/config_store.js";
  import { onMount } from "svelte";
  import { Canvas, Rect } from "fabric";

  let grid = [];
  let canvas;
  let overlay = null;
  const cell_width = 50;
  const cell_height = 50;

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
          strokeWidth: 0.1,
          stroke: "black",
          selectable: false,
        });
        if (i > grid.length - 1) {
          new_grid[i][j] = { value: -1, canvas_elem: rect };
        } else if (j > grid[i].length - 1) {
          new_grid[i][j] = { value: -1, canvas_elem: rect };
        } else {
          new_grid[i][j] = { value: grid[i][j], canvas_elem: rect };
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

  onMount(() => {
    canvas = new Canvas("c");
    canvas.backgroundColor = "#000000";
    update_grid();

    canvas.on("mouse:wheel", function (opt) {
      let delta = opt.e.deltaY;
      let zoom = canvas.getZoom();
      zoom *= 0.999 ** delta;
      if (zoom > 20) zoom = 20;
      if (zoom < 0.01) zoom = 0.01;
      canvas.zoomToPoint({ x: opt.e.offsetX, y: opt.e.offsetY }, zoom);
      canvas.renderAll();

      opt.e.preventDefault();
      opt.e.stopPropagation();
    });

    canvas.on("mouse:down", function (opt) {
      // @ts-ignore
      let e = opt.e;
      if (e.ctrlKey === true) {
        canvas.is_dragging = true;
        canvas.selection = false;
        // @ts-ignore
        canvas.lastPosX = e.clientX;
        // @ts-ignore
        canvas.lastPosY = e.clientY;
      }
    });

    canvas.on("mouse:move", function (opt) {
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

      if (opt.target) {
        let target_x = opt.target.left;
        let target_y = opt.target.top;
        let target_w = opt.target.width;
        let target_h = opt.target.height;
        let new_overlay = new Rect({
          left: target_x,
          top: target_y,
          fill: "rgba(255, 255, 255, 0.5)",
          width: target_w,
          height: target_h,
          selectable: false,
        });
        if (overlay) { canvas.remove(overlay); }
        overlay = new_overlay;
        canvas.add(overlay);
        canvas.renderAll();
      }
      
      else if (overlay) {
        canvas.remove(overlay);
        overlay = null;
        canvas.renderAll();
      }
          

    });

    canvas.on("mouse:up", function (opt) {
      canvas.setViewportTransform(canvas.viewportTransform);
      canvas.is_dragging = false;
      canvas.selection = true;
    });

    const unsubscribe_width = grid_width.subscribe(update_grid);
    const unsubscribe_height = grid_height.subscribe(update_grid);

    return () => {
      unsubscribe_width();
      unsubscribe_height();
      canvas.dispose();
    };
  });

</script>

<!-- Change the grid width and height -->

<canvas id="c" width="600" height="600" style="border: 1px solid black" />
