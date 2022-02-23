use super::base::Base;

use super::traits::{create::Create, edit::Edit, get::Get, delete::Delete};

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

impl Create for Pending {}
impl Delete for Pending {}
impl Get    for Pending {}
impl Edit   for Pending {}