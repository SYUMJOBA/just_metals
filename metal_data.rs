pub enum MaterialClassId {
    Stone(StoneMaterial),
    Wood(WoodMaterial),
    Gem(GemMaterial),
    Metal(MetalMaterial)
}

pub struct MaterialClass {
    pub name: String,
    pub strength: f64,
    pub burn_spread: f64,
    pub burn_speed: f64
}

impl MaterialClass {
    pub fn from(material_id: MaterialClassId) -> Self {
        match material_id {
            MaterialClassId::Stone(_) => MaterialClass {
                name: "Stone".into(),
                strength: 23.5,
                burn_speed: 0.0,
                burn_spread: 0.0
            },
            MaterialClassId::Wood(_) => MaterialClass {
                name: "Wood".into(),
                strength: 12.0,
                burn_speed: 2.0,
                burn_spread: 4.0
            },
            MaterialClassId::Gem(_) => MaterialClass {
                name: "Gem".into(),
                strength: 56.40,
                burn_speed: 0.0,
                burn_spread: 0.0,
                
            },
            MaterialClassId::Metal(_) => MaterialClass {
                name: "Metal".into(),
                strength: 34.6,
                burn_speed: 0.0,
                burn_spread: 0.0
            }
        }
    }
}

pub enum WoodMaterialId {

}

pub struct WoodMaterial {

}

pub enum StoneMaterialId {

}

pub struct StoneMaterial {
    
}

pub enum GemMaterialId {

}

pub struct GemMaterial {

}

pub enum MetalMaterialId {

}

pub struct MetalMaterial {
    pub electric_conductivity: f64
}

pub enum FireClassId {
    RedFlame(RedFire),
    SoulFlame(SoulFire),
    ElectricFlame(ElectricFire)
}

pub struct FireClass {
    pub name: String,
    pub burn_strength: f64,
    pub expansion: f64,
    pub incazzaggine: f64
}

pub struct RedFire {
    pub source: Option<WoodMaterialId>
}

pub struct SoulFire {
    pub source: Option<GemMaterialId>
}

pub struct ElectricFire {
    pub source: Option<MetalMaterialId>
}

impl FireClass {
    pub fn from(fireclass_id: FireClassId) -> Self {
        match fireclass_id {
            FireClassId::RedFlame(_) => FireClass {
                name: "RedFlame".into(),
                burn_strength: 10.0,
                expansion: 5.0,
                incazzaggine: 2.0
            },
            FireClassId::SoulFlame(_) => FireClass {
                name: "SoulFlame".into(),
                burn_strength: 32.0,
                expansion: 0.0,
                incazzaggine: 10.0
            },
            FireClassId::ElectricFlame(_) => FireClass {
                name: "ElectricFlame".into(),
                burn_strength: 50.0,
                expansion: 20.0,
                incazzaggine: 3.0
            }
        }
    }
}

pub struct Arrow {
    
}

pub struct Bow {

}

