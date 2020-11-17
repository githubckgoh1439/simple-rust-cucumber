# simple-rust-cucumber


An implementation of the Cucumber testing framework for Rust, by refer [![Documentation](https://docs.rs/cucumber_rust/badge.svg)](https://docs.rs/cucumber_rust)

### Usage

Create a directory called `tests/` in the project root and create a test target base on preference. In this example I will name it `cucumber.rs`.

Add this to your `Cargo.toml`:

```toml
[[test]]
name = "cucumber"
harness = false # Allows Cucumber to print output instead of libtest

[dev-dependencies]
cucumber = { package = "cucumber_rust", version = "^0.7.0" }
```

Create a directory called `tests/features/` and put all feature files in it named something like `example.feature`. It might look like:

```gherkin
Feature: Example feature

  Scenario: An example scenario
    Given I am trying out Cucumber
    When I consider what I am doing
    Then I am interested in ATDD
    And we can implement rules with regex

```

Those examples can be found under directory called `tests/`

I can then run those Cucumber tests by running these commands:

```
cargo test --test cucumber_example01
```
```
cargo test --test cucumber_example02
```
```
cargo test --test cucumber_encryptor
```
