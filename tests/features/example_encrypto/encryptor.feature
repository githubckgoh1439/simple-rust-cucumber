Feature: Encrypt messages and write them to a file.

  Scenario: Encrypt a simple gohck333- message.
    Given I have an encryptor initialized with input "gohck333!"
     Then I should see "gohck333!" in the encryptor's input field
     When I encrypt the encryptor's input
     Then testfile.txt exists
      And testfile.txt is not empty
     When I decrypt testfile.txt
     Then the decrypted result should be "gohck333!"