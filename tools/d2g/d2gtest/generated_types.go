// This source code file is AUTO-GENERATED by github.com/taskcluster/jsonschema2go

package d2gtest

import (
	"encoding/json"
)

type (
	// Static d2g input/output test cases. Contains pairs of Docker Worker payload
	// (inputs) and Generic Worker expected payload (outputs).
	D2GTestCases struct {

		// A suite of tests for a particular d2g feature or set of features
		TestSuite TestSuite `json:"testSuite"`
	}

	// A test case contains a static input Docker Worker task payload, and an
	// expected Generic Worker task payload output. The Docker Worker task payload
	// is converted by d2g to a Generic Worker task payload. The test is successful
	// if the generated Generic Worker task payload exactly matches the Generic
	// Worker task payload in the test case.
	TestCase struct {

		// Detailed information about what the test case tests
		Description string `json:"description"`

		// Additional properties allowed
		DockerWorkerTaskPayload json.RawMessage `json:"dockerWorkerTaskPayload"`

		// Additional properties allowed
		GenericWorkerTaskPayload json.RawMessage `json:"genericWorkerTaskPayload"`

		// Name for the test case
		Name string `json:"name"`
	}

	// A suite of tests for a particular d2g feature or set of features
	TestSuite struct {

		// Detailed information about what the test cases do and do not test
		Description string `json:"description"`

		// Name for the test suite
		Name string `json:"name"`

		// The test cases which this test suite contains
		Tests []TestCase `json:"tests"`
	}
)

// Returns json schema for the payload part of the task definition. Please
// note we use a go string and do not load an external file, since we want this
// to be *part of the compiled executable*. If this sat in another file that
// was loaded at runtime, it would not be burned into the build, which would be
// bad for the following two reasons:
//  1. we could no longer distribute a single binary file that didn't require
//     installation/extraction
//  2. the payload schema is specific to the version of the code, therefore
//     should be versioned directly with the code and *frozen on build*.
//
// Run `generic-worker show-payload-schema` to output this schema to standard
// out.
func JSONSchema() string {
	return `{
  "$id": "schemas/test_suites.json#",
  "$schema": "http://json-schema.org/draft-06/schema#",
  "additionalProperties": false,
  "description": "Static d2g input/output test cases. Contains pairs of Docker Worker payload\n(inputs) and Generic Worker expected payload (outputs).",
  "properties": {
    "testSuite": {
      "additionalProperties": false,
      "description": "A suite of tests for a particular d2g feature or set of features",
      "properties": {
        "description": {
          "description": "Detailed information about what the test cases do and do not test",
          "title": "Test Suite Description",
          "type": "string"
        },
        "name": {
          "description": "Name for the test suite",
          "title": "Test Suite Name",
          "type": "string"
        },
        "tests": {
          "description": "The test cases which this test suite contains",
          "items": {
            "additionalProperties": false,
            "description": "A test case contains a static input Docker Worker task payload, and an\nexpected Generic Worker task payload output. The Docker Worker task payload\nis converted by d2g to a Generic Worker task payload. The test is successful\nif the generated Generic Worker task payload exactly matches the Generic\nWorker task payload in the test case.",
            "properties": {
              "description": {
                "description": "Detailed information about what the test case tests",
                "title": "Test Case Description",
                "type": "string"
              },
              "dockerWorkerTaskPayload": {
                "type": "object"
              },
              "genericWorkerTaskPayload": {
                "type": "object"
              },
              "name": {
                "description": "Name for the test case",
                "title": "Test Case Name",
                "type": "string"
              }
            },
            "required": [
              "name",
              "description",
              "dockerWorkerTaskPayload",
              "genericWorkerTaskPayload"
            ],
            "title": "Test case",
            "type": "object"
          },
          "minItems": 1,
          "title": "Test cases",
          "type": "array"
        }
      },
      "required": [
        "name",
        "description",
        "tests"
      ],
      "title": "Test Suite",
      "type": "object"
    }
  },
  "required": [
    "testSuite"
  ],
  "title": "d2g test cases",
  "type": "object"
}`
}
