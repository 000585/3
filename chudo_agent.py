import os
import subprocess
import threading
import time
import grpc
from flask import Flask, render_template, jsonify, request

try:
    import node_pb2
    import node_pb2_grpc
    GRPC_AVAILABLE = True
except ImportError:
    GRPC_AVAILABLE = False

app = Flask(__name__)
NODE_PROCESS = None
NODE_LOGS = []

class BlockchainClient:
    def __init__(self, port=50054):
        self.target = f"127.0.0.1:{port}"
    
    def get_status(self):
        if not GRPC_AVAILABLE: return {"error": "gRPC modules missing"}
        try:
            with grpc.insecure_channel(self.target) as channel:
                stub = node_pb2_grpc.NodeServiceStub(channel)
                response = stub.GetStatus(node_pb2.Empty(), timeout=1)
                return {
                    "running": response.running,
                    "height": response.block_height,
                    "peers": response.peer_count,
                    "hash": response.last_block_hash
                }
        except Exception as e:
            return {"error": "Offline", "details": str(e)}

grpc_client = BlockchainClient()

class CHUDOAssistant:
    def answer(self, question):
        q = question.lower().strip()
        if any(x in q for x in ["статус", "высота", "блока", "height", "сколько"]):
            data = grpc_client.get_status()
            if "error" in data: 
                return "❌ Нода не отвечает. Убедитесь, что она запущена в соседнем терминале."
            return f"🧱 Высота блоков: {data['height']} | 📡 Пиров: {data['peers']} | Статус: 🟢 Online"
        return "Связь с ядром установлена. Спросите про высоту блоков."

assistant = CHUDOAssistant()

@app.route("/")
def index(): return render_template("index.html")

@app.route("/status")
def status(): return jsonify({"running": NODE_PROCESS is not None, "logs": NODE_LOGS})

@app.route("/api/chat", methods=["POST"])
def chat():
    msg = request.json.get("message", "")
    return jsonify({"response": assistant.answer(msg)})

if __name__ == "__main__":
    app.run(host="127.0.0.1", port=5000, debug=False)
