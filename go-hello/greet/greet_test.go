package greet

import "testing"

func TestHelloPreservesInputCase(t *testing.T) {
	got := Hello("world")
	want := "Hello, world!"
	if got != want {
		t.Fatalf("Hello(%q) = %q, want %q", "world", got, want)
	}
}
