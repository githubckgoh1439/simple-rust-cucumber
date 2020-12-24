
use async_trait::async_trait;
use std::{convert::Infallible};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MyWorldCalculator {
    // You can use this struct for mutable context in scenarios.
    input_1: i32,
    input_2: i32,
    operation: String,
}

impl MyWorldCalculator {
   
    fn calculate(&mut self, flag: String)-> i32{
 
        // println!("operator use : {:?}", flag);
        if flag.eq("+"){
           let rs = self.input_1 + self.input_2;
           return rs;
        }

        let rs = self.input_1 - self.input_2;
        return rs;
    } 


}

#[async_trait(?Send)]
impl cucumber::World for MyWorldCalculator {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            input_1: 0,
            input_2: 0,
            operation: "".to_string(),
        })
    }
}

mod example_steps_calculator {
    // use cucumber::{t, Steps, World};
    use cucumber::{t, Steps};

    pub fn steps() -> Steps<crate::MyWorldCalculator> {
        let mut builder: Steps<crate::MyWorldCalculator> = Steps::new();

        builder
            .given(
                "a clear calculator",
                |world: crate::MyWorldCalculator, _step| {
                    world
                },
            )
            .when_regex_async(
                "I begin use operator of (.*)",
                t!(|mut world, operator, _step| {
                    world.operation = operator[1].to_owned();
                    world
                }),
            )
            .then_regex_async(
                r#"^I input value 1 "([\w\s!]+)"$"#,
                t!(|mut world, val_input1, _step| {
                    world.input_1 = val_input1[1].to_owned().parse::<i32>().unwrap();
                    world
                }),
            )
            .then_regex_async(
                r#"^I input value 2 "([\w\s!]+)"$"#,
                t!(|mut world, val_input2, _step| {

                    if let Err(e) = val_input2[1].to_owned().parse::<i32>(){
                        println!("Failed conversion to i32: {}", e);
                        world.input_2 = 0;
                    }else{
                        println!("OK conversion to i32.....");
                        world.input_2 = val_input2[1].to_owned().parse::<i32>().unwrap();
                    }    

                    world
                }),
            )
            .then_regex_async(
                r#"^the result is "([\w\s!]+)"$"#,
                t!(|mut world, output, _step| {
                    let opr = &world.operation;
                    let mut nx_world = world.to_owned();
                    let rs_output = nx_world.calculate(opr.to_owned());
                    assert_eq!(rs_output.to_owned(), output[1].to_owned().parse::<i32>().unwrap());
                    world
                }),
            );
          
    
        builder
    }
}

fn main() {
    // Do any setup you need to do before running the Cucumber runner.
    // e.g. setup_some_db_thing()?;

    let runner = cucumber::Cucumber::<MyWorldCalculator>::new()
        .features(&["./tests/features/example_calculator/"])
        .steps(example_steps_calculator::steps());
       
    // You may choose any executor you like (Tokio, async-std, etc)
    // You may even have an async main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(runner.run());
}

