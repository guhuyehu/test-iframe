import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <iframe
        onLoad={() => {
        }}
        style={{ width: "100vw", height: "100vh", border: "none" }}
        allow="clipboard-read; clipboard-write; fullscreen; downloads; storage;"
        sandbox="allow-same-origin allow-scripts allow-popups allow-forms allow-top-navigation allow-top-navigation-by-user-activation allow-downloads allow-modals allow-pointer-lock allow-storage-access-by-user-activation"
        src="https://dify.meierbei.cn/workflow/PIb7eBEmAZowdC65"
      />
    </main>
  );
}

export default App;
