# So that task await doesn't timeout
max_duration = 100_000

defmodule Person do
  def get_ready(name) do
    IO.puts "#{name} started getting ready"
    delay = (60 + Enum.random(0..30))
    :timer.sleep(delay * 100)
    IO.puts "#{name} spent #{delay} seconds getting ready"
    name
  end

  def put_shoes(name) do
    IO.puts "#{name} started putting on shoes"
    delay = (35 + Enum.random(0..10))
    :timer.sleep(delay * 100)
    IO.puts "#{name} spent #{delay} seconds putting on shoes"
    :ok
  end
end

defmodule Alarm do
  def arm_alarm do
    IO.puts "Arming alarm."
    :timer.sleep(60 * 100)
    IO.puts "Alarm is armed."
  end
end


IO.puts "Let's go for a walk!"

put_shoes_pid = ["Alice", "Bob"]
|> Enum.map(fn p -> Task.async(fn -> Person.get_ready(p) end) end)
|> Enum.map(fn(task) -> Task.await(task, max_duration) end)
|> Enum.map(fn p -> Task.async(fn -> Person.put_shoes(p) end) end)

alarm_pid = Task.async(fn -> Alarm.arm_alarm end)

[put_shoes_pid, alarm_pid]
|> Enum.map(fn 
  pids when is_list(pids) -> pids
    |> Enum.map(fn pid -> Task.await(pid, max_duration) end)
    |> Enum.reduce(fn(_, _) -> :ok end)
    |> (fn _ -> IO.puts "Exiting and locking the door." end).()
  pid -> Task.await(pid, max_duration) 
end)