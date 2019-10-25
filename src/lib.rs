pub use rand;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Leaves {
  pub count: f64,
}

#[wasm_bindgen]
impl Leaves {
  pub fn new() -> Self {
    Leaves { count: 0.0 }
  }
}

#[wasm_bindgen]
pub struct Trees {
  pub count: i32,
  pub baby_count: i32,
  ratio: i32,
}

#[wasm_bindgen]
impl Trees {
  pub fn new() -> Self {
    Trees {
      count: 1,
      baby_count: 0,
      ratio: 40,
    }
  }
  fn add_tree(&mut self) {
    self.baby_count = self.baby_count - 1;
    self.count = self.count + 1;
  }
  fn remove_baby(&mut self) {
    self.baby_count = self.baby_count - 1;
  }
  pub fn create_leaves(&self, leaves: &mut Leaves, seconds: f64) {
    let ratio = self.ratio as f64 / 60.0 * seconds;
    println!("ratio {}", ratio);
    leaves.count = leaves.count + (ratio * self.count as f64);
  }
  pub fn plant_tree(&mut self, fruits: &mut Fruits) -> BabyTree {
    fruits.count = fruits.count - 1;
    self.baby_count = self.baby_count + 1;
    BabyTree::new()
  }
}

#[wasm_bindgen]
pub struct BabyTree {
  ratio: f64,
  pub growth: f64,
}

#[wasm_bindgen]
pub enum GrowthActions {
  Growing = 2,
  Grown = 1,
  Died = 0,
}

#[wasm_bindgen]
impl BabyTree {
  pub fn new() -> Self {
    BabyTree {
      ratio: 0.25,
      growth: 0.0,
    }
  }
  pub fn grow(&mut self, seconds: f64, trees: &mut Trees) -> GrowthActions {
    let ratio = self.ratio / 60.0 * seconds;
    self.growth = self.growth + ratio;

    if self.growth >= 100.0 {
      if rand::random() {
        trees.add_tree();
        return GrowthActions::Grown;
      }
      trees.remove_baby();
      return GrowthActions::Died;
    }

    GrowthActions::Growing
  }
}

#[wasm_bindgen]
pub struct Fruits {
  pub count: i32,
  pub leaves_cost: i32,
}

#[wasm_bindgen]
impl Fruits {
  pub fn new() -> Self {
    Fruits {
      count: 0,
      leaves_cost: 10,
    }
  }
  pub fn add_fruit(&mut self, leaves: &mut Leaves) {
    if leaves.count > self.leaves_cost as f64 {
      self.count = self.count + 1;
      leaves.count = leaves.count - self.leaves_cost as f64;
    }
  }
}
