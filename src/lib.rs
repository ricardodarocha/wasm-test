pub mod parser;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_name = alert)]
    fn alert_usize(a: usize);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Ainda estou implementando!");
    alert_usize(5);
}

pub struct OwnerID {
    id: usize,
}

#[wasm_bindgen]
pub struct Car {
    pub number: usize,
    pub color: usize, // color in hex code
    owner: OwnerID,
}

#[wasm_bindgen]
impl Car {
    pub fn new() -> Self {
        Car {
            number: 0,
            color: 0,
            owner: OwnerID { id: 0 },
        }
    }
    pub fn duplicate(&self) -> Self {
        Self {
            number: self.number + 1,
            color: self.color,
            owner: OwnerID { id: 0 },
        }
    }

    pub fn change_number(&mut self, number: usize) {
        self.number = number;
    }

    pub fn get_id(&self) -> usize {
        self.owner.id
    }
}

 impl Default for Car {
     fn default() -> Self {
         Self::new()
     }
 }


#[wasm_bindgen]
pub fn color(a: Car, color: usize) -> Car {
    Car {
        number: a.number,
        color,
        owner: OwnerID { id: 0 },
    }
}

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[wasm_bindgen]
pub fn mul(a: usize, b: usize) -> usize {
    a * b
}

#[no_mangle]
pub extern "C" fn addc(a: usize, b: usize) -> usize {
   a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
