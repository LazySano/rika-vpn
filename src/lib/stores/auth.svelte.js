import { invoke } from "@tauri-apps/api/core";
import { load, save } from "./storage.js";
import { GOOGLE_CLIENT_ID } from "../config.js";

export const auth = $state({
  session: load("rika-session", null),
  error: null,
  busy: false,
});

function setSession(user) {
  auth.session = {
    id: user.id,
    name: user.name,
    email: user.email,
    avatar: user.avatar ?? null,
    provider: user.provider,
    createdAt: user.createdAt,
  };
  save("rika-session", auth.session);
}

function initials(name) {
  const parts = (name || "").trim().split(/\s+/).filter(Boolean);
  if (parts.length === 0) return "؟";
  if (parts.length === 1) return parts[0].slice(0, 2);
  return (parts[0][0] || "") + (parts[1][0] || "");
}

export function userInitials() {
  return auth.session ? initials(auth.session.name) : "؟";
}

export async function loginWithGoogle() {
  auth.error = null;
  auth.busy = true;
  try {
    const user = await invoke("google_login", { clientId: GOOGLE_CLIENT_ID });
    auth.busy = false;
    setSession({
      id: "g_" + user.sub,
      name: user.name,
      email: user.email,
      avatar: user.picture,
      provider: "google",
      createdAt: Date.now(),
    });
    return true;
  } catch (e) {
    auth.busy = false;
    const message = String(e);
    auth.error = message.includes("GOOGLE_CLIENT_ID_MISSING")
      ? "لم يتم ضبط معرّف Google Client ID بعد."
      : message;
    return false;
  }
}

export function logout() {
  auth.session = null;
  save("rika-session", null);
}
