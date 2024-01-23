
pub mod data;
pub mod decision;
pub mod loops;
pub mod functions;
pub mod primitive_types;
pub mod composite_types;
pub mod struct_type;
pub mod enum_type;

pub use data::default_function as data;
pub use decision::default_function as decision;
pub use loops::default_function as loops;
pub use functions::default_function as functions;
pub use primitive_types::default_function as primitive_types;
pub use composite_types::default_function as composite_types;
pub use struct_type::default_function as struct_type;
pub use enum_type::default_function as enum_type;