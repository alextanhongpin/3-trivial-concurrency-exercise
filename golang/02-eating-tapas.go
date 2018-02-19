package main

import (
	"log"
	"math/rand"
	"sync"
	"time"
)

// http://whipperstacker.com/2015/10/05/3-trivial-concurrency-exercises-for-the-confused-newbie-gopher/
// Solution 2: Eating Tapas
func main() {

	makeDishes := func(menus []string) []string {
		var dishes []string
		for i := 0; i < len(menus); i++ {
			numDish := 5 + rand.Intn(5)
			for j := 0; j < numDish; j++ {
				dishes = append(dishes, menus[i])
			}
		}
		return shuffle(dishes)
	}

	eat := func(person, dish string, isDining chan interface{}, isAvailable chan interface{}, wg *sync.WaitGroup) {
		duration := 2 + rand.Intn(2)
		log.Printf("%s is enjoying some %s\n", person, dish)
		time.Sleep(time.Duration(duration) * time.Second)
		wg.Done()
		<-isDining
		<-isAvailable
	}

	dine := func(done chan interface{}, dishes, people []string, available map[string]chan interface{}, wg *sync.WaitGroup) {
		log.Println("Bon appetit!")
		isDining := make(chan interface{}, len(people))
		for _, dish := range dishes {
			isDining <- true
			go func(dish string) {
				for person, isAvailable := range available {
					select {
					case <-done:
						return
					case isAvailable <- true:
						eat(person, dish, isDining, isAvailable, wg)
						return
					default:
					}
				}
			}(dish)
		}
	}

	rand.Seed(1)

	people := []string{"Alice", "Bob", "Charlie", "Dave"}
	menus := []string{"chorizo", "chopitos", "pimientos de padron", "croquetas", "patatas bravas"}

	available := make(map[string]chan interface{})

	for _, p := range people {
		available[p] = make(chan interface{}, 1)
	}

	dishes := makeDishes(menus)
	done := make(chan interface{})
	defer close(done)

	var wg sync.WaitGroup
	wg.Add(len(dishes))
	dine(done, dishes, people, available, &wg)
	wg.Wait()

	log.Println("That was delicious!")
}

func shuffle(src []string) []string {
	out := make([]string, len(src))
	perm := rand.Perm(len(src))
	for i, v := range perm {
		out[v] = src[i]
	}
	return out
}
