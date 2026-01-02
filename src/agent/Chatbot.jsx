import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./ChatBot.css";

export default function ChatBot() {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  const [messages, setMessages] = useState([]);
  const bottomRef = useRef(null);

  useEffect(() => {
    const unlisten = listen("agent_result", (e) => {
      setMessages((prev) => {
        const copy = [...prev];
        copy[copy.length - 1] = {
          role: "ai",
          text: e.payload || "No response"
        };
        return copy;
      });
    });

    return () => unlisten.then(f => f());
  }, []);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const send = async () => {
    if (!input.trim()) return;

    setMessages((m) => [
      ...m,
      { role: "user", text: input },
      { role: "ai", text: "Thinking..." }
    ]);

    await invoke("ask_agent", { input });
    setInput("");
  };

  if (!open) {
    return (
      <button className="floating-btn" onClick={() => setOpen(true)}>
        🤖
      </button>
    );
  }

  return (
    <div className="chat-container">
      <header>
        Offline AI
        <button onClick={() => setOpen(false)}>✕</button>
      </header>

      <div className="chat-body">
        {messages.map((m, i) => (
          <div key={i} className={`bubble ${m.role}`}>
            {m.text}
          </div>
        ))}
        <div ref={bottomRef} />
      </div>

      <footer>
        <textarea
          placeholder="Ask anything..."
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && !e.shiftKey && send()}
        />
        <button onClick={send}>Send</button>
      </footer>
    </div>
  );
}
