defmodule InternetCafe do
  def online(i, agent, max_concurrency) do
    Agent.update(agent, fn list -> [i | list] end)
    list = Agent.get(agent, fn list -> list end)
    if Enum.count(list) == max_concurrency do
      remaining = 1..25 |> Enum.to_list
      remaining -- list |> Enum.map(fn i -> IO.puts "Tourist #{i} waiting for turn" end)
    end
    IO.puts "Tourist #{i} is online"
    duration = Enum.random(20..30)
    :timer.sleep(duration * 100)
    IO.puts "Tourist #{i} is done, having spent #{duration} minutes online."
  end
end

max_concurrency = 8
{:ok, agent} = Agent.start_link fn -> [] end

tourists = 1..25 
|> Enum.to_list
|> Enum.shuffle
|> Task.async_stream(InternetCafe, :online, [agent, max_concurrency], max_concurrency: max_concurrency)
|> Enum.to_list

Agent.stop(agent)