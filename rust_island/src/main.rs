
struct SurvivalKit {
    coconut: i32,
    energy: i32,
    water: f64,
    has_knife: bool,
    inventory: Vec<&'static str>,
}

struct Shelter {
    size: i32,
    durability: i32,
    material: String,
}

impl Shelter {
    fn new(size_value: i32, durability_value: i32, material_value: String) -> Shelter {
        Shelter {size: size_value, durability: durability_value, material: material_value}
    }
    fn print(&self) {
        println!("size: {}, durability: {}, material: {}", self.size, self.durability, self.material);    
    }

}

fn drink_water (kit: &mut SurvivalKit) {
    if kit.water >= 0.5 {
        kit.water -= 0.5;
        kit.energy += 25;
        println!("drinking water...");
    }
    else {
        println!("not enough water!");
    }

    println!("Water: {:.1} liters", kit.water);
    println!("Energy: {}", kit.energy);
}

fn main() {
    
    let name: &str = "Helene";
    let mut kit: SurvivalKit = SurvivalKit {
        coconut: 2,
        energy: 100,       
        water: 5.0,
        has_knife: true,
        inventory: Vec::new(),
    };
    let abri: Shelter = Shelter::new(10, 50, "Bois");

    println!("Name: {}", name);
    println!("Coconuts: {}", kit.coconut);
    println!("Water: {:.1} liters", kit.water);
    println!("Has knife: {}", kit.has_knife);
    println!("Energy: {}", kit.energy);

    drink_water(&mut kit);

    kit.inventory.push("Stick1");
    kit.inventory.push("Stick2");
    kit.inventory.push("Stone1");
    kit.inventory.push("Stone2");

    for item in &kit.inventory {
        println!("- {}", item);
    }

    abri.print();

}
