
//this project is called minigrep. The duty of minigrep is to allow us search for string withing a file. it is a simple example of command line programmes
use std::env; //importing the environment from the std library
use std::fs;  // to create our file system 
use std::process; // this  helps us to exit the program with out paniking
use std::error::Error;
fn main() {
    //the first step is to create a command line argument. We want the user to pass in a string and a filename. so we import our environment module
  let args: Vec<String> = env::args().collect(); //we create a variable called args, where the type is a vector of strings. next we call the args method on the environment module and the collect.
                                                 //the args() method will give us an iterator, and the collect method will turn it into a collection which is a vector of strings.
      //println!("{:?}", args);

//next we create two variables nameely, query and a filename
let config = Config::new(&args).unwrap_or_else(|err |{
    println!("Problem parsing arguments: {}", err);
    process::exit(1)
});

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);
//next we create a new file at the root of our project. just create a file!!!!!. then inside the file, write a poem
//the we move to call the std library into scope to create our file.

 if let Err(e)= run(config){
    println!("Application error: {}", e);
    process::exit(1);
 }

}

fn run(config:Config)-> Result<(), Box<dyn Error>>{// insted of the .expect, we add a question mark
    let contents = fs::read_to_string(config.filename)?;

                      println!("With text:\n{}", contents);

                      Ok(())

}

struct Config{
    query: String,
    filename: String,
}
//next we create an implementation for our config struct
impl Config{
    fn new(args: &[String]) -> Result<Config, &str> { //to properly handle our errors, let us have our return tyoe as the result which has two variants
          //lets add a check
          if args.len() <3{
            return Err("Not enough arguments");
          }

        let query = args[1].clone(); //we created a variable named query, and we pass a reference to the vector
        let filename = args[2].clone(); //we cloned because we dont want to take ownership
    
          Ok( Config {query, filename} )//returning a tuple
}
}

