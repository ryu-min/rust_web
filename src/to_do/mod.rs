pub(crate) mod structs;

use structs::done::Done;
use structs::pending::Pending;
//use crate::to_do::ItemType::{Done, Pending};

pub enum ItemType {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(item_type : String, item_title: String)
    -> Result<ItemType, &'static str>
{
    if item_type == "pending" {
        let pending_item = Pending::new(item_title);
        Ok(ItemType::Pending(pending_item))
    }
    else if item_type == "done" {
        let done_item = Done::new(item_title);
        Ok(ItemType::Done(done_item))
    }
    else {
        Err("this is not accepted")
    }
}