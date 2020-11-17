use async_trait::async_trait;
use std::{cell::RefCell, convert::Infallible};
use cucumber::{t, Steps};

pub struct MyWorld02 {
    // You can use this struct for mutable context in scenarios.
    foo: String,
    bar: usize,
    some_value: RefCell<u8>,
}

impl MyWorld02 {
    async fn test_async_fn(&mut self) {
        *self.some_value.borrow_mut() = 123u8;
        self.bar = 123;
    }
}

#[async_trait(?Send)]
impl cucumber::World for MyWorld02 {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            foo: "wat".into(),
            bar: 0,
            some_value: RefCell::new(0),
        })
    }
}

mod example_steps02 {
    use cucumber::{t, Steps, World};

    pub fn steps() -> Steps<crate::MyWorld02> {
        let mut builder: Steps<crate::MyWorld02> = Steps::new();

        builder
            .given_async(
                "a thing",
                t!(|mut world, _step| {
                    world.foo = "elho".into();
                    world.test_async_fn().await;
                    world
                }),
            )
            .when_regex_async(
                "something goes (.*)",
                t!(|world, _matches, _step| crate::MyWorld02::new().await.unwrap()),
            )
            .given(
                "I am trying out Cucumber",
                |mut world: crate::MyWorld02, _step| {
                    world.foo = "Some string".to_string();
                    world
                },
            )
            .when("I consider what I am doing", |mut world, _step| {
                let new_string = format!("{}.", &world.foo);
                world.foo = new_string;
                world
            })
            .then("I am interested in ATDD", |world, _step| {
                assert_eq!(world.foo, "Some string.");
                world
            })
            .then_regex(
                r"^we can (.*) rules with regex$",
                |world, matches, _step| {
                    // And access them as an array
                    assert_eq!(matches[1], "implement");
                    world
                },
            )
            .given_regex(r"a number (\d+)", |mut world, matches, _step| {
                world.foo = matches[1].to_owned();
                world
            })
            .then_regex(
                r"twice that number should be (\d+)",
                |world, matches, _step| {
                    let to_check = world.foo.parse::<i32>().unwrap();
                    let expected = matches[1].parse::<i32>().unwrap();
                    assert_eq!(to_check * 2, expected);
                    world
                },
            );

        builder
    }
}

fn main() {
    // Do any setup you need to do before running the Cucumber runner.
    // e.g. setup_some_db_thing()?;

    let runner = cucumber::Cucumber::<MyWorld02>::new()
        .features(&["./tests/features/example_02/"])
        .steps(example_steps02::steps());
       

    // You may choose any executor you like (Tokio, async-std, etc)
    // You may even have an async main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(runner.run());
}

