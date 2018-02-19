defmodule Person do
  def eat(meal, person) do
    IO.puts "#{person} is enjoying some #{meal}"
    :timer.sleep(Enum.random(10..30) * 100)
    person
  end

  def dine(dish, agent) do
    person = Agent.get(agent, fn [head | _] -> head end)
    Agent.update(agent, fn list -> list -- [person] end)

    pid = Task.async(fn -> eat(dish, person) end) 
    person = Task.await(pid, 100_000)

    Agent.update(agent, fn list -> list ++ [person] end)
  end
end

dishes = ["chorizo", "chopitos", "pimientos de padron", "croquetas", "patatas bravas"]
|> Enum.map(fn menu -> 
    1..Enum.random(5..10)
    |> Enum.to_list
    |> Enum.map(fn _ -> menu end)
   end)
|> Enum.reduce([], fn(r, l) -> l ++ r end)
|> Enum.shuffle

{:ok, agent} = Agent.start_link(fn -> ["Alice", "Bob", "Car", "Didy"] end)

max_concurrency = 4
dishes
|> Task.async_stream(Person, :dine, [agent], max_concurrency: max_concurrency)
|> Enum.to_list

Agent.stop(agent)