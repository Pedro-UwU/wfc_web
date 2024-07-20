const get_divisors = (n) => {
  let divisors = [];
  for (let i = 1; i <= n; i++) {
    if (n % i === 0) {
      divisors.push(i);
    }
  }
  return divisors;
}

const get_image_width_and_height = (objectURL) => {
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

const get_tile_array = (img, img_w, img_h, tiles_w, tiles_h) => {
  if (!img.complete) {
    console.error("Image not fully loaded");
    return [];
  }

  let total_cols = Math.floor(img_w / tiles_w);
  let total_rows = Math.floor(img_h / tiles_h);
  let tiles = [];

  for (let i = 0; i < total_cols; i++) {
    for (let j = 0; j < total_rows; j++) {
      let canvas = document.createElement("canvas");
      canvas.width = tiles_w;
      canvas.height = tiles_h;
      let context = canvas.getContext('2d');

      context.drawImage(
        img, 
        i * tiles_w, 
        j * tiles_h, 
        tiles_w, 
        tiles_h, 
        0, 
        0, 
        tiles_w, 
        tiles_h
      );

      let img_data = canvas.toDataURL();
      tiles.push(img_data);
    }
  }

  return [...new Set(tiles)];
};


export {
  get_divisors,
  get_image_width_and_height,
  get_tile_array
}
