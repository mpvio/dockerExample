## Instructions
Open CMD in dockerExample folder:

docker-compose build
docker-compose run --rm set2

## Explanation
Set1 is a Python FastAPI endpoint listening for a string from Set2 (Rust code)
1. Set2 takes a string input from the user and sends it via request to Set1.
2. Set1's response is a json object containing the received text.
3. Set2 then outputs the response message.
