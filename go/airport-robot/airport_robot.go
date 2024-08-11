package airportrobot

import "fmt"

type Greeter interface {
	LanguageName() string
	Greet(name string) string
}

func SayHello(name string, greeter Greeter) string {
	return fmt.Sprintf("I can speak %s: %s", greeter.LanguageName(), greeter.Greet(name))
}

// Italian Greeter
type Italian struct{}

func (italianGreeter Italian) LanguageName() string {
	return "Italian"
}

func (italianGreeter Italian) Greet(name string) string {
	return fmt.Sprintf("Ciao %s!", name)
}

// Portuguese Greeter
type Portuguese struct{}

func (portugueseGreeter Portuguese) LanguageName() string {
	return "Portuguese"
}

func (portugueseGreeter Portuguese) Greet(name string) string {
	return fmt.Sprintf("Olá %s!", name)
}
