use hashbrown::HashMap;

pub struct AdvancedLog{
    channels: HashMap<&'static str, Vec<String>>
}

/* 
pub static mut P: AdvancedLog = AdvancedLog{
    channels: 
};
*/