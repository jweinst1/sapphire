
/// The `SaphObject` serves as the main type object in the 
/// Sapphire language. It contains many enum variants to handle
/// different data types.
#[derive(Debug)]
pub enum SaphObject {
	Null,
	Number(f32)
}

impl SaphObject {

	pub fn to_f32(&self) -> f32 {
		match self {
			SaphObject::Null => 0.0,
			SaphObject::Number(num) => *num
		}
	}

	pub fn to_bool(&self) -> bool {
		match self {
			SaphObject::Null => false,
			SaphObject::Number(num) => (*num as i32) != 0		
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			SaphObject::Null => String::from("Null"),
			SaphObject::Number(num) => num.to_string()
		}
	}
}
