package main

import (
	"context"
	"log"
	"math/rand"
	"sync"
	"time"
)

func main() {
	rand.Seed(1)
	people := []string{"Alice", "Bob"}

	log.Println("Let's go for a walk")
	var wg sync.WaitGroup

	wg.Add(2)
	for _, name := range people {
		go gettingReady(&wg, name)
	}
	wg.Wait()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	alarmStream := makeAlarm(ctx)

	wg.Add(2)
	for _, name := range people {
		go puttingOnShoes(&wg, name)
	}
	wg.Wait()

	log.Println("Exiting and locking the door.")
	alarmStream <- true
}

func delayInSeconds(min, max int) int {
	delta := max - min
	duration := min + rand.Intn(delta)
	time.Sleep(time.Duration(duration) * time.Second)
	return duration
}

func gettingReady(wg *sync.WaitGroup, name string) {
	log.Printf("%s started getting ready\n", name)
	duration := delayInSeconds(6, 9)
	log.Printf("%s spent %v seconds getting ready\n", name, duration)
	wg.Done()
}

func puttingOnShoes(wg *sync.WaitGroup, name string) {
	log.Printf("%s started putting on shoes\n", name)
	duration := delayInSeconds(3, 4)
	log.Printf("%s spend %d seconds putting on shoes\n", name, duration)
	wg.Done()
}

func makeAlarm(ctx context.Context) chan interface{} {
	alarmStream := make(chan interface{}, 1)
	heartbeatInterval := 2 * time.Second
	log.Println("Arming alarm")
	alarmStream <- true

	go func() {
		for {
			select {
			case <-ctx.Done():
				log.Println("Alarm is armed.")
				<-alarmStream
				return
			case <-time.After(heartbeatInterval):
				log.Println("Alarm is counting down...")
			}
		}
	}()

	return alarmStream
}
