import { useState } from "react";
import ChatBot from "./Chatbot.jsx";

export default function FloatingBot() {
  const [open, setOpen] = useState(false);

  return (
    <>
      {open && (
        <div style={styles.panel}>
          <ChatBot />
        </div>
      )}

      <button
        style={styles.fab}
        onClick={() => setOpen(!open)}
        title="Offline AI Assistant"
      >
        🤖
      </button>
    </>
  );
}

const styles = {
  fab: {
    position: "fixed",
    bottom: 20,
    right: 20,
    width: 55,
    height: 55,
    borderRadius: "50%",
    fontSize: 24,
    cursor: "pointer",
    border: "none",
    background: "#2563eb",
    color: "#fff",
    boxShadow: "0 4px 10px rgba(0,0,0,0.3)"
  },
  panel: {
    position: "fixed",
    bottom: 85,
    right: 20,
    background: "#1e1e1e",
    color: "#fff",
    padding: 10,
    borderRadius: 10,
    boxShadow: "0 6px 20px rgba(0,0,0,0.4)",
    zIndex: 1000
  }
};
