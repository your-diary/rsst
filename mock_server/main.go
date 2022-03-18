//usr/bin/env go run "$0" "$@"; exit

package main

import "io"
import "net/http"
import "fmt"
import "time"
import "os"

func main() {

	var file = "xml/rss_1.xml"
	var port = "9009"

	var f, err = os.Open(file)
	if err != nil {
		fmt.Printf("The file [ %s ] not found.\n", file)
		os.Exit(1)
	}
	defer f.Close()
	var fileContent, _ = io.ReadAll(f)
	fmt.Printf("Target File: [ %s ]\n", file)

	var server = &http.Server{
		Addr:         ":" + port,
		ReadTimeout:  5 * time.Second,
		WriteTimeout: 1 * time.Second,
	}

	http.HandleFunc("/", func(w http.ResponseWriter, req *http.Request) {
		w.Write(fileContent)
	})

	fmt.Printf("Started listening the port %s...\n\n", port)
	server.ListenAndServe()

}
