mod department;

pub use department::*;

// Explicit re-exports for clarity
pub use department::{Department, CreateDepartment, DepartmentList, DepartmentListBase, UpdateDepartment};