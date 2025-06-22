from fastapi import FastAPI, Request

app = FastAPI()

@app.post("/process")
async def process_string(request: Request):
    input_string = await request.body()
    decoded_string = input_string.decode('utf-8')
    print(f"Received: {decoded_string}")
    return {"message": f"Processed: {decoded_string}"}