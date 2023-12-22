# Advent of Code 2023

¡Hola! Este año me he animado a hacer el Advent of Code, aunque he empezado bastante tarde y no lo voy a terminar a tiempo. He elegido usar **Rust** este año, ya que tengo pensado usarlo en un futuro cercano y quiero aprender a usarlo. Voy a ir poniendo mis soluciones y los problemas que he encontrado en cada día para que sirvan de guía para otra gente que quiera intentarlo. (Voy a escribir esto en inglés)

---

Hi! This year I've decided to try and solve Advent of Code, although I'm really late and I won't finish it on time. This year I've chosen **Rust**, since I've been thinking about using it in the future and I want to learn it. I'm going to leave my solutions and the problems I've found on each day for other people who wants to try and solve them.

---

## Day 1

### Part 1

For the first day, we have to parse a list of lines, get the first and last number of each line, put them together and then sum up all the numbers. For example, from the line `one2three4` we would obtain the number `24`.

This part was not so difficult, since all I had to do was chaining simple collection functions. The hardest part was getting used to Rust and its syntax and types. I had a little bit of trouble dealing with Results, but I handled it easily.

I also learnt about Rust tests and used them to check my code with the examples provided in the problem.

### Part 2

This part was a lot tricker than the first. In this part, numbers written in text such as `one` and `two` are also taken into account. So the line `one2three4` would be parsed as `14`.

My first approach involved iterating over a HashMap which contained every written number with its respective numeric value and replacing them in the line. This didn't work and produced random results, since the numbers would be evaluated in a random order.

Then I realized the actual problem: overlaps. There are cases such as `twone` (two and one) where the last and first letter of the numbers overlap, and the first number to be evaluated would remove a letter from the second one, thus making it impossible to parse.

The solution to this was not so intuitive and I got a little help to realize what needed to be done: instead of replacing the whole number (`one` with `1`), I left the first and last letters to prevent overlaps (`one` would become `o1e`). I also used a vector instead of a hash map so the numbers would be evaluated in order (although it shouldn't be a problem anymore).

In my opinion, this first day was a lot harder than last year, but it can be solved easily once you realize the problem with the overlaps.

