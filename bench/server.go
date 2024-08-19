package main

import (
	"log"
	"net/http"
	"os"
    "path/filepath"
)

func main() {
	wd, err := os.Getwd()
    if err != nil {
        log.Fatal(err)
    }

    // Set the directory to serve (absolute path to the "bench" directory)
    benchDir := filepath.Join(wd, "bench")
    fs := http.FileServer(http.Dir(benchDir))

	// Handle all requests by serving a file of the same name
	http.Handle("/", fs)

	// Define the port to listen on
	port := "8080"
	log.Printf("Listening on http://localhost:%s/", port)

	// Start the server
	err = http.ListenAndServe(":"+port, nil)
	if err != nil {
		log.Fatal(err)
	}
}