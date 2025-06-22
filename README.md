## Instructions
Open CMD in dockerExample folder:

1. docker-compose build
2. docker-compose run --rm side-rust

## Explanation
side-python is a Python FastAPI endpoint listening for a string from side-rust (Rust)
1. side-rust takes a string input from the user and sends it via request to side-python.
2. side-python's response is a json object containing the received text.
3. side-rust then outputs the response message.
4. run side-rust detached to access the stdin from the terminal
