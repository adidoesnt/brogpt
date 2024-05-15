import os
from flask import Flask, request
from dotenv import load_dotenv
from openai import OpenAI
from flask_cors import CORS

load_dotenv()

env = os.environ.get('ENV', 'dev')
host = os.environ.get('HOST', '127.0.0.1')
port = os.environ.get('PORT', 8080)
llama_api_key = os.environ.get('LLAMA_API_KEY', 'DUMMY-API-KEY')

llama = OpenAI(
    api_key = llama_api_key,
    base_url = "https://api.llama-api.com"
)
app = Flask(__name__)

CORS(app)

@app.route('/health', methods=['GET'])
def health():
    return 'server is running'

@app.route('/chat', methods=['POST'])
def reply():
    data = request.get_json()
    original_prompt: str = data.get('message')
    if original_prompt is None or original_prompt.strip() == '':
        return { "error": "Prompt is required" }, 400
    prompt = f"""Answer this prompt, but like a frat brother: {original_prompt}"""
    print(f"prompt: {prompt}")
    response = llama.chat.completions.create(model="llama-13b-chat", messages=[
            {
                "role": "system", 
                "content": prompt
            },
        ])
    reply = response.choices[0].message.content
    return { "reply": reply }

if __name__ == '__main__':
    debug = False
    if env == 'dev':
        debug = True
    app.run(host=host, port=port, debug=debug)
