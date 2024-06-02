import { useEffect, useState } from "react";
import { useAuth } from "./context/AppContext";
// import { User } from "../../declarations/users_backend/users_backend.did";

function App() {
  const { login, logout, isAuthenticated} = useAuth();


  return (
    <main className="app-main">
      Nexus Chats
    </main>
  );
}

export default App;
