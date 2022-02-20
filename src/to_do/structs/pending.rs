use super::base::Base;

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title : String) -> Pending {
        let input_status : String = String::from("Pending");
        let base : Base = Base::new(input_title, input_status);
        Pending {super_struct : base}
    }
}