mod plugin; mod runner;

fn main() {
    let logger = plugin::LoggerPlugin::new("logger", 1, "sudo");
    let validator = plugin::ValidatorPlugin::new("validator", 2, 3);
    let run = runner::PluginRunner::new(logger, validator);

    match run.run_pipeline("Hello") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e)
    }

    match run.run_pipeline("Hi") {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e)
    }

    println!("{:?}", run.list_plugins());

    //sudo: Hello
    //Input too short
    //[("logger", 1), ("validator", 2)]
}  
