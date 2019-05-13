
/// The `SaphObject` serves as the main type object in the 
/// Sapphire language. It contains many enum variants to handle
/// different data types.
#[derive(Debug)]
pub enum SaphObject {
	Null,
	Int(i32)
}

impl SaphObject {
	/// Can be used to convert a `SaphObject` into a signed
	/// 32-bit integer.
	/// # Examples
	/// The following example prints the `i32` value of a
	/// SaphObject.
	/// ```rust
	/// use sapphire::object::SaphObject;
	/// let obj = SaphObject::Int(3);
	/// println!("{}", obj.to_i32());
	/// ```
	pub fn to_i32(&self) -> i32 {
		match self {
			SaphObject::Null => 0,
			SaphObject::Int(num) => *num
		}
	}

	pub fn to_bool(&self) -> bool {
		match self {
			SaphObject::Null => false,
			SaphObject::Int(num) => *num == 1		
		}
	}

	pub fn to_string(&self) -> String {
		match self {
			SaphObject::Null => String::from("Null"),
			SaphObject::Int(num) => num.to_string()
		}
	}
}
