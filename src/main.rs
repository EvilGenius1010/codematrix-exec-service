use std::{path::{Path, PathBuf}, process::Command, time::Duration};
use rlimit::setrlimit;
use tempfile::TempDir;


#[derive(Debug)]
struct NonNegativeU64{
    value:u64
} //is this private?

impl NonNegativeU64{
    pub fn new(value:u64)->Result<NonNegativeU64,String>{
        if value >= 0{
            Ok(NonNegativeU64{value:value})
        }
        else{
            Err("Size is negative.".to_string())
        }
    }
}


#[derive(Debug)]
struct ResourceLimits{ //specifies resource limits.
    cpu_time:Duration,
    memory_mb:Result<NonNegativeU64,String>,
    stack_size_mb:Result<NonNegativeU64,String>
}

struct CppExecutor{
    compiler_paths:PathBuf, //mutable file path
    resource_limits:ResourceLimits
}

impl CppExecutor{
    pub fn new()->Self{ //ensure access to struct by only this method. 
        
         CppExecutor{
            compiler_paths:PathBuf::from("/usr/bin/g++"),
            resource_limits:ResourceLimits{
                cpu_time:Duration::from_secs(1),
                memory_mb:NonNegativeU64::new(256),//change these later
                stack_size_mb:NonNegativeU64::new(64)//change these later
         }
    }
}

   async fn compile(&self,source_file_path:&Path,output_path:&Path){
    const COMPILERFLAGS:[&str;4]=["O2","-Wall","-fstack-protector","-static-libstdc++"];
    let status = Command::new(&self.compiler_paths)
        .args(&[
        source_file_path.to_str().unwrap(),//behavior here ie does it panic or what?
        "-o",
        output_path.to_str().unwrap(),
        &COMPILERFLAGS[0],
        &COMPILERFLAGS[1],
        &COMPILERFLAGS[2],
        &COMPILERFLAGS[3], //dirty way to do it.
    ])
    .status();


        //create a tempfile
        //spawn a new process
        //compile cpp code using flags.
    }



    async fn execute(&self,binary_file_path:&Path){
        let output = Command::new(binary_file_path)
        .current_dir(binary_file_path.parent().unwrap())
        .output();

        
    }


    pub async fn run_code(&self,code:&str){
        let temp_dir = TempDir::new();
        // let source_file_path = temp_dir


    }
}


#[tokio::main] //meaning of this?
async fn main(){
    println!("hefsllo!")
}