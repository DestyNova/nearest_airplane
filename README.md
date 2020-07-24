# nearest_airplane

A solution to [dailyprogrammer challenge #360](https://www.reddit.com/r/dailyprogrammer/comments/8i5zc3/20180509_challenge_360_intermediate_find_the/) on Reddit, written in Rust as a learning exercise.

This took a couple of hours to complete, spread over a few days, which was a couple of hours longer than I thought.

My approach when solving this was pretty straightforward:

1. Grab a sample API response with `curl`, to avoid hitting the service pointlessly while working on the solution.
2. Write a test for the parser, asserting an expected number of results.
3. Write the parser (read the sample response into a string and deserialise with `serde_json`).
4. Run the tests again and discover that some of the result contain nulls almost everywhere.
5. Change any properties that might be null to an Option type in the struct.
6. Add one or two more assertions to the tests to give more confidence that the parser is behaving reasonably.
7. Add a test + code that parses the coordinate input format from stdin.
8. Look up "geodesic formula", discover the Haversine formula and steal an implementation from Rosetta Code. Change it to not require mutation of its arguments.
9. Add code to map over the flight states from the parsed Opensky API response, and get the distance between each flight and the input coordinates.
10. Remember that some of the flights might have missing lat/long coords, so produce a `None` in those cases and use `flat_map` instead of `map`.
11. Sort the resulting list of `(distance, state)` pairs and output the first one.
12. Search for "the" way to do a simple HTTP request and get JSON back in Rust. Add the `reqwest` library and copy/pasta to do the request.
13. Decide that 998 MB of dependencies and binaries is too much for a single request, and replace the `reqwest` crate with the simpler `attohttpc` crate, bringing the `target` directory down to about 325 MB after a clean and rebuild.
14. Profit!

I fought the Rust compiler a fair bit, but overall it was fun and the type system really helped. The Vim (neovim) plugins I installed for Rust seem to be very slow, and `rust-analyzer` timed out on several occasions. Might need to change some of it up, but I really liked some of the refactoring / code generation abilities provided by the [Conquer of Completion](https://github.com/neoclide/coc.nvim) plugin's "codeaction" feature.

Would like to try something like this in Go and see if it's a massive drag or not. In particular, the ability to do map/flatmap and collect into different types of container is really neat in Rust and might be boilerplatey in Go.
