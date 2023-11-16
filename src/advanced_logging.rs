use hashbrown::HashMap;
use lazy_static::lazy_static;
use core::panic;
use std::cell::Cell;

/*
I dont even know what happenning in here anymore
Need to find a way to make AdvancedLog and UninitLogger
both plug into the LOG variable without using if / switch
statements.

Also, LOG should be the only global mutable variable in 
this project. All others should be refactored to not be.
LOG just needs to be due to its nature
*/

pub trait Logger{
    fn push_new_channel(&mut self, channel: &'static str);
    fn write_to_channel(&mut self, channel: &'static str, data: String) -> Option<()>;
    fn flush_all(&mut self);
}


pub struct UninitLogger{

}

impl Logger for UninitLogger{
    fn push_new_channel(&mut self, channel: &'static str) {
        panic!("Attempted to push new log channel before logger init");
    }
    fn write_to_channel(&mut self, channel: &'static str, data: String) -> Option<()> {
        panic!("Attempted write to log channel before logger init");
    }
    fn flush_all(&mut self){
        panic!("Attempted flush log channels before logger init");
    }  
}

pub struct AdvancedLog{
    // maybe this shouldn't store String
    channels: HashMap<&'static str, Vec<String>>,
    channel_list: Vec<&'static str>
}

impl Logger for AdvancedLog{
    fn push_new_channel(&mut self, channel: &'static str){
        self.channel_list.push(channel);
        self.channels.insert(channel, Vec::new());
    }
    fn write_to_channel(&mut self, channel: &'static str, data: String) -> Option<()> {
        let channel = self.channels.get_mut(channel)?;
        channel.push(data);
        Some(())
    }
    fn flush_all(&mut self) {
        for channel in &self.channel_list{
            self.channels
                .get_mut(channel)
                .expect("Channel in log list but not in hashmap")
                .clear()
        }
    }
}

pub enum LogLogic{
    Uninit,
    Init
}

// this helps avoid pain
static mut LOG: LogHolder = LogHolder{ log: &mut UninitLogger{} };

pub fn init(){
    unsafe{
        /* 
        LOG.log = &'static mut AdvancedLog{
            channel_list: Vec::new(),
            channels: HashMap::new()
        }
        */
    }
}

pub fn new_log_channel(channel: &'static str){
    println!("Spawning new log");
    unsafe{
        //LOG.push_new_channel(channel);
    }
}

pub fn push_to_log(channel: &'static str, msg: String){
    unsafe{
       // LOG.write_to_channel(channel, msg);
    }
}

pub fn flush_logs(){
    unsafe{
       // LOG.flush_all();
    }
}