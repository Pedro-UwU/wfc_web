export class Tile {
  constructor(tile_id) {
    this.tile_id = tile_id;
    this.active = false;
    this.can_rotate = false;
    this.north = [];
    this.east = [];
    this.south = [];
    this.west = [];
  };

  set_active(active) {
    this.set_active = active;
  }

  set_can_rotate(can_rotate) {
    this.can_rotate = can_rotate;
  }

  add_category(direction, category) {
    if (direction === 'north' && this.north.length < 4 && !this.north.includes(category)) {
      this.north.push(category);
    }

    if (direction === 'east' && this.east.length < 4 && !this.east.includes(category)) {
      this.east.push(category);
    }

    if (direction === 'south' && this.south.length < 4 && !this.south.includes(category)) {
      this.south.push(category);
    }

    if (direction === 'west' && this.west.length < 4 && !this.west.includes(category)) {
      this.west.push(category);
    }
  }

  remove_category(direction, category) {
    if (direction === 'north' && this.north.includes(category)) {
      this.north.splice(this.north.indexOf(category), 1);
    }

    if (direction === 'east' && this.east.includes(category)) {
      this.east.splice(this.east.indexOf(category), 1);
    }

    if (direction === 'south' && this.south.includes(category)) {
      this.south.splice(this.south.indexOf(category), 1);
    }

    if (direction === 'west' && this.west.includes(category)) {
      this.west.splice(this.west.indexOf(category), 1);
    }
  }
}

