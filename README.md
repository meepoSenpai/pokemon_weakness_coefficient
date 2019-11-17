# Pokémon Weakness Coefficient

This is a Rust Library meant to be used in an upcoming webapplication that calculates what weaknesses your team has.
Currently it doesn't to much but the following steps are planned:

- Statically (or lazily) load all Pokémon Types and their weaknesses into the library (types and their weakness-factor are given in `src/static/weaknesses.csv`. Types themselves are stored in `src/static/types.csv`)
- Statically (or lazily) load all Pokémon, their Types into the library (distant future outlook: also load if they have stronger Def. or Sp. Def.)
- Calculate what types of attacks a team is weakest against, given a set of up to 12 types contained in your team
- Calculate what types of attacks a team is weakest against, given a set of up to 6 Pokémon

This is just some pet project to get me passionate about programming again, and to get me a bit distracted from C++ when I'm really fed up with how my Bachelors-Thesis is going.
