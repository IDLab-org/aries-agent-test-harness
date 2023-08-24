
Feature: Mobile flows and features

    Scenario: Mediation

    Scenario: Connection
        Given "2" agents
            | name | role   |
            | Acme | issuer |
            | Bob  | holder |
        When "Acme" generates a connection invitation
        And "Bob" receives the connection invitation
        And "Bob" sends a connection request to "Acme"
        And "Acme" receives the connection request
        And "Acme" sends a connection response to "Bob"
        And "Bob" receives the connection response
        And "Bob" sends <message> to "Acme"
        Then "Acme" and "Bob" have a connection

    Scenario: Connection re-use
        Given "2" agents

    Scenario: Messaging
        Given "2" agents

    Scenario: Issue-credential
        Given "2" agents
        | name | role   |
        | Acme | issuer |
        | Bob  | holder |
        And "Acme" and "Bob" have an existing connection
        When "Acme" offers a credential
        And "Bob" requests the credential
        And "Acme" issues the credential
        And "Bob" acknowledges the credential issue
        Then "Bob" has the credential issued

    Scenario: Issue-credential v2 - AnonCred
        Given "2" agents

    Scenario: Issue-credential v2 - Json-ld
        Given "2" agents

    Scenario: Verify-credential
        Given "2" agents

    Scenario: Verify-credential zkp
        Given "2" agents

    Scenario: Verify-credential v2 - AnonCred
        Given "2" agents

    Scenario: Verify-credential v2 - Json-ld
        Given "2" agents

    Scenario: Revocation
        Given "2" agents

    Scenario: OOB
        Given "2" agents

    Scenario: OOB w/ cred attach
        Given "2" agents

    Scenario: OOB w/ proof attach
        Given "2" agents