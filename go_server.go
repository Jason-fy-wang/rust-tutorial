package main

import (
	"log"
	"net/http"
)

func  main() {
	fs := http.FileServer(http.Dir("./target/criterion"))

	http.Handle("/",fs)
	log.Println("serving on 8080")
	log.Fatal(http.ListenAndServe(":8080",nil))
}


