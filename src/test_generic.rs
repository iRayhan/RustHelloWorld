pub struct GenericStruct<T, E>
where
    T: AllowedGrade,
    E: AllowedError,
{
    pub(crate) grade: T,
    pub(crate) error: E,
}

trait AllowedGrade {
    fn to_int(&self) -> i32;
}
impl AllowedGrade for String {
    fn to_int(&self) -> i32 {
        -1
    }
}
impl AllowedGrade for &str {
    fn to_int(&self) -> i32 {
        -1
    }
}
impl AllowedGrade for i32 {
    fn to_int(&self) -> i32 {
        *self
    }
}
impl AllowedGrade for f32 {
    fn to_int(&self) -> i32 {
        *self as i32
    }
}

trait AllowedError {
    fn from_bool(value: bool) -> Self;
}
impl AllowedError for bool {
    fn from_bool(value: bool) -> Self {
        value
    }
}

pub fn get_grade<T, E>(mut grade: GenericStruct<T, E>) -> Option<String>
where
    T: std::fmt::Display + AllowedGrade + std::cmp::PartialOrd,
    E: std::fmt::Display + AllowedError,
{
    match std::any::type_name::<T>() {
        "alloc::string::String" | "&str" => {
            grade.error = E::from_bool(false);
            Some(format!(
                "String grade, error: {} {}",
                grade.grade, grade.error
            ))
        }
        "i32" => {
            if grade.grade.to_int() > 100 {
                grade.error = E::from_bool(true);
                return Some(format!(
                    "Integer grade, error: {} {}",
                    grade.grade, grade.error
                ));
            }
            Some(format!(
                "Integer grade, error: {} {}",
                grade.grade, grade.error
            ))
        }
        "f32" => {
            if grade.grade.to_int() > 100 {
                grade.error = E::from_bool(true);
                return Some(format!(
                    "Float grade, error: {} {}",
                    grade.grade, grade.error
                ));
            }
            Some(format!(
                "Float grade, error: {} {}",
                grade.grade, grade.error
            ))
        }
        _ => None,
    }
}
