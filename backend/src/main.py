import os
from flask import Flask, request
from dotenv import load_dotenv
from llamaapi import LlamaAPI

load_dotenv()

env = os.environ.get('ENV', 'dev')
host = os.environ.get('HOST', '127.0.0.1')
port = os.environ.get('PORT', 8080)
llama_api_key = os.environ.get('LLAMA_API_KEY', 'DUMMY-API-KEY')

llama = LlamaAPI(llama_api_key) 
app = Flask(__name__)

@app.route('/health', methods=['GET'])
def health():
    return 'server is running'

@app.route('/reply', methods=['POST'])
def reply():
    data = request.get_json()
    original_prompt: str = data.get('prompt')
    original_prompt = original_prompt.strip()
    prompt = f"""Answer this prompt, but like a frat brother: {original_prompt}"""
    if prompt is None or prompt == '':
        return { "error": "Prompt is required" }, 400
    print(f"prompt: {prompt}")
    api_request_json = {
        "model": "llama3-70b",
        "messages": [
            {
                "role": "system", 
                "content": prompt
            },
        ]
    }
    response = llama.run(api_request_json)
    response_json = response.json()
    summary = response_json['choices'][0]['message']['content']
    return { "summary": summary }

if __name__ == '__main__':
    debug = False
    if env == 'dev':
        debug = True
    app.run(host=host, port=port, debug=debug)
