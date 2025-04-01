use nix::unistd::{fork, ForkResult, Pid};
use std::marker::PhantomData;


pub struct Parent;
pub struct Child;

pub struct Fork<State = Parent> {
    pid: Option<Pid>,
    _state: PhantomData<State>,
}



//representation of parent process after successful fork
impl Fork<Parent>{
    //create a new fork manager
    pub fn new()->Self{
        match unsafe {fork()}{
            Ok(ForkResult::Parent { child, .. }) => Fork {
                pid: Some(child),
                _state: PhantomData,
            },
            Ok(ForkResult::Child) =>{
                panic!("Fork failed, called parent fork on child process ");
            },
            Err(e) => panic!("Fork failed: {}", e),
        }

    }

    pub fn into_child(self)->Fork<Child>{
        match self.pid{
            Some(pid) => {
                Fork {
                    pid: Some(pid),
                    _state: PhantomData,
                }
            },
            None => panic!("Fork failed, called parent fork on child process "),
        }
    }
}