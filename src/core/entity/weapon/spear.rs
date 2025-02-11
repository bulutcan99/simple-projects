use std::any::Any;
use super::weapon::{Rarity, Weapon};

#[derive(Debug, Clone)]
pub struct Spear {
	name: String,
	rarity: Rarity,
	price: u8,
	weight: u8,
	attack_damage: f32,
	attack_speed: f32,
	durability: u8,
	sharpness: u8,
	range: u8,
	required_level: u32,
}

impl Spear {
	pub fn new(
		name: String,
		rarity: Rarity,
		price: u8,
		weight: u8,
		attack_damage: f32,
		attack_speed: f32,
		sharpness: u8,
		range: u8,
		required_level: u32,
	) -> Self {
		Self {
			name,
			rarity,
			price,
			weight,
			attack_damage,
			attack_speed,
			durability: 100_u8,
			sharpness,
			range,
			required_level,
		}
	}
}

impl Weapon for Spear {
	fn get_name(&self) -> &str {
		&self.name
	}

	fn get_rarity(&self) -> Rarity {
		self.rarity.clone()
	}

	fn get_price(&self) -> u8 {
		self.price
	}

	fn get_weight(&self) -> u8 {
		self.weight
	}

	fn get_attack_speed(&self) -> f32 {
		self.attack_speed
	}

	fn get_attack_damage(&self) -> f32 {
		self.attack_damage * self.rarity.get_value() * (self.sharpness as f32)
	}


	fn get_durability(&self) -> u8 {
		self.durability
	}

	fn set_durability(&mut self, durability: u8) {
		self.durability -= durability
	}

	fn get_range(&self) -> u8 {
		self.range
	}

	fn clone_box(&self) -> Box<dyn Weapon> {
		Box::new(self.clone())
	}

	fn get_required_level(&self) -> u32 {
		self.required_level
	}

	fn as_any(&self) -> &dyn Any {
		self
	}
}
