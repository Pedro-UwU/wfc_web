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
      this.north[section] = null;
    }
    if (direction === 'south') {
      this.north[section] = null;
    }
    if (direction === 'west') {
      this.north[section] = null;
    }
  }
}

