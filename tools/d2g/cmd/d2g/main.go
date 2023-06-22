package main

import (
	"encoding/json"
	"fmt"
	"io"
	"log"
	"os"
	"strings"

	"github.com/mcuadros/go-defaults"
	"github.com/taskcluster/taskcluster/v53/tools/d2g"
	"github.com/taskcluster/taskcluster/v53/tools/d2g/dockerworker"
	"github.com/taskcluster/taskcluster/v53/tools/d2g/genericworker"
	"github.com/xeipuuv/gojsonschema"
)

func main() {
	log.SetFlags(0)
	log.SetPrefix("d2g: ")

	// Read the JSON input from standard input
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		log.Fatal("Failed to read input:", err)
	}

	// Validate the JSON input against the schema
	err = validateJSON(input, dockerworker.JSONSchema())
	if err != nil {
		log.Fatalf("Input validation failed: %v", err)
	}

	// Convert the validated JSON input
	dwPayload := new(dockerworker.DockerWorkerPayload)
	defaults.SetDefaults(dwPayload)
	err = json.Unmarshal(input, &dwPayload)
	if err != nil {
		log.Fatalf("Failed to convert input to a docker worker payload definition: %v", err)
	}

	gwPayload, err := d2g.Convert(dwPayload)
	if err != nil {
		log.Fatal(err)
	}

	formattedActualGWPayload, err := json.MarshalIndent(*gwPayload, "", "  ")
	if err != nil {
		log.Fatalf("Cannot convert Generic Worker payload %#v to JSON: %s", *gwPayload, err)
	}

	// Validate the JSON output against the schema
	err = validateJSON(formattedActualGWPayload, genericworker.JSONSchema())
	if err != nil {
		log.Fatalf("Output validation failed: %v", err)
	}

	fmt.Println(string(formattedActualGWPayload))
}

func validateJSON(input []byte, schema string) error {
	// Parse the JSON schema
	schemaLoader := gojsonschema.NewStringLoader(schema)
	documentLoader := gojsonschema.NewBytesLoader(input)

	// Perform the validation
	result, err := gojsonschema.Validate(schemaLoader, documentLoader)
	if err != nil {
		return err
	}

	// Check if the validation failed
	if !result.Valid() {
		// Collect validation errors
		var errors []string
		for _, desc := range result.Errors() {
			errors = append(errors, desc.String())
		}

		// Return the validation errors as an error message
		return fmt.Errorf("validation failed:\n%s", strings.Join(errors, "\n"))
	}

	return nil
}