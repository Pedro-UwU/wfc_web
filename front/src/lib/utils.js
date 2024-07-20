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


export {
  get_divisors,
  get_image_width_and_height
}
