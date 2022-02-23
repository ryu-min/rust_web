use super::base::Base;
use super::traits::{get::Get, delete::Delete, edit::Edit};

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title : String) -> Done {
        let input_status : String = String::from("Done");
        let base : Base = Base::new(input_title, input_status);
        Done {super_struct : base}
    }
}

impl Get    for Done {}
impl Delete for Done {}
impl Edit   for Done {}
