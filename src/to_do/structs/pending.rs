use super::base::Base;

use super::traits::{create::Create, edit::Edit, get::Get, delete::Delete};

pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title : &str) -> Pending {
        let base : Base = Base::new(input_title, "pending");
        Pending {super_struct : base}
    }
}

impl Create for Pending {}
impl Delete for Pending {}
impl Get    for Pending {}
impl Edit   for Pending {}