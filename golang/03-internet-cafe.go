package main

import (
	"log"
	"math/rand"
	"sync"
	"time"
)

func main() {
	rand.Seed(1)

	delay := func() int {
		duration := 5 + rand.Intn(10)
		time.Sleep(time.Duration(duration) * time.Second)
		return duration
	}

	genTourists := func(num int) <-chan interface{} {
		touristStream := make(chan interface{}, num)

		go func() {
			for i := 0; i < num; i++ {
				touristStream <- i + 1
			}
			close(touristStream)
		}()

		return touristStream
	}

	clearQueue := func(
		queueStream <-chan interface{},
		wg *sync.WaitGroup,
		i interface{},
	) {
		log.Printf("Tourist %d is online.\n", i)
		duration := delay()
		<-queueStream
		wg.Done()
		log.Printf("Tourist %d is done, having spent %d seconds online.\n", i, duration)
	}

	queue := func(
		done chan interface{},
		queueStream chan interface{},
		wg *sync.WaitGroup,
		i interface{},
	) {
		select {
		case <-done:
			return
		case queueStream <- i:
			clearQueue(queueStream, wg, i)
		}
	}

	openCafe := func(
		done chan interface{},
		touristStream <-chan interface{},
		wg *sync.WaitGroup,
		maxComputers int,
	) {
		queueStream := make(chan interface{}, maxComputers)

		for i := range touristStream {
			go func(i interface{}) {
				select {
				case <-done:
					return
				case queueStream <- i:
					clearQueue(queueStream, wg, i)
				default:
					log.Printf("Tourist %d is waiting for turn.\n", i)
					queue(done, queueStream, wg, i)
				}
			}(i)
		}
	}

	// Initialization
	numTourists := 25
	maxComputers := 8

	done := make(chan interface{})
	defer close(done)

	var wg sync.WaitGroup
	wg.Add(numTourists)
	openCafe(done, genTourists(numTourists), &wg, maxComputers)
	wg.Wait()

	log.Println("The place is empty, let's close up and go to the beach!")
}
