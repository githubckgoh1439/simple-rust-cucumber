Feature: Calculator for Testing

  Scenario: Calculate two positive numbers
    Given a clear calculator
    When I begin use operator of "-"
    Then I input value 1 "177"
    And I input value 2 "4"
    Then the result is "173"

  # Scenario: Adding three numbers
  #   Given a clear calculator
  #   When I begin adding
  #   And I input 2
  #   And I input -7
  #   And I input 20
  #   Then the result is 15

  Scenario: Calculate numbers resulting in a negative if any
    Given a clear calculator
    When I begin use operator of "-"
    Then I input value 1 "3"
    And I input value 2 "2"
    Then the result is "1"
