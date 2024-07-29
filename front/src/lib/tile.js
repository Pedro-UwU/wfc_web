/**
 * @class
 */
export class Tile {
  constructor(tile_id, sections) {
    this.tile_id = tile_id;
    this.active = false;
    this.can_rotate = false;
    this.sections = 1;
    this.weight = 1;
    this.north = Array(sections).fill(null);
    this.east = Array(sections).fill(null);
    this.south = Array(sections).fill(null);
    this.west = Array(sections).fill(null);
  };

  set_active(active) {
    this.set_active = active;
  }

  set_can_rotate(can_rotate) {
    this.can_rotate = can_rotate;
  }

  set_sections(sections) {
    this.sections = sections;
  }

  add_category(direction, section, category) {
    console.log("About to add in direction: ", direction);
    if (direction === 'north') {
      this.north[section] = category;
    }
    if (direction === 'east') {
      this.east[section] = category;
    }
    if (direction === 'south') {
      this.south[section] = category;
    }
    if (direction === 'west') {
      this.west[section] = category;
    }

  }

  remove_category(direction, section) {
    if (direction === 'north') {
      this.north[section] = null;
    }
    if (direction === 'east') {
      this.east[section] = null;
    }
    if (direction === 'south') {
      this.south[section] = null;
    }
    if (direction === 'west') {
      this.west[section] = null;
    }
  }

  get_category(direction, section) {
    if (direction === 'north') {
      return this.north[section]
    }
    if (direction === 'east') {
      return this.east[section]
    }
    if (direction === 'south') {
      return this.south[section]
    }
    if (direction === 'west') {
      return this.west[section]
    }
  }

  has_null_category() {
    return this.north.includes(null)
      || this.east.includes(null)
      || this.south.includes(null)
      || this.west.includes(null)
  }
  clone() {
    const newTile = new Tile(this.tile_id, this.sections);
    newTile.active = this.active;
    newTile.can_rotate = this.can_rotate;
    newTile.weight = this.weight;
    newTile.north = [...this.north];
    newTile.east = [...this.east];
    newTile.south = [...this.south];
    newTile.west = [...this.west];
    return newTile;
  }
}

