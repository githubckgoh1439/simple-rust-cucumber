Feature: Basic functionality
   # Comment
  @foo
  Scenario: foo
    Given a thing
    When nothing

  Scenario: bar
       # comment
    Given a thing
    When something goes wrong

  Scenario Outline: scenario with examples
    Given a number <num>
    Then twice that number should be <double>

    Examples:
      | num | double |
      |   2 |      4 |
      |   3 |      6 |

  Rule: A rule

    Scenario: a scenario inside a rule
      Given I am in inside a rule
      Then things are working
