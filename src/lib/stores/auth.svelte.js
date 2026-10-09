import { load, save } from "./storage.js";

export const auth = $state({
  session: load("rika-session", null),
  error: null,
  busy: false,
});

function users() {
  return load("rika-users", []);
}

function saveUsers(list) {
  save("rika-users", list);
}

async function hash(text) {
  try {
    if (typeof crypto !== "undefined" && crypto.subtle) {
      const buf = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text));
      return [...new Uint8Array(buf)].map((b) => b.toString(16).padStart(2, "0")).join("");
    }
  } catch {
    /* fallthrough */
  }
  let h = 5381;
  for (let i = 0; i < text.length; i++) h = ((h << 5) + h + text.charCodeAt(i)) >>> 0;
  return "f" + h.toString(16);
}

function setSession(user) {
  auth.session = {
    id: user.id,
    name: user.name,
    email: user.email,
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

export async function register(name, email, password) {
  auth.error = null;
  name = (name || "").trim();
  email = (email || "").trim().toLowerCase();
  if (name.length < 2) {
    auth.error = "الاسم قصير جدًا";
    return false;
  }
  if (!/^[^@\s]+@[^@\s]+\.[^@\s]+$/.test(email)) {
    auth.error = "البريد الإلكتروني غير صالح";
    return false;
  }
  if ((password || "").length < 6) {
    auth.error = "كلمة المرور يجب ألا تقل عن 6 أحرف";
    return false;
  }
  const list = users();
  if (list.some((u) => u.email === email)) {
    auth.error = "هذا البريد مسجَّل مسبقًا";
    return false;
  }
  auth.busy = true;
  const passwordHash = await hash(email + ":" + password);
  auth.busy = false;
  const user = {
    id: "u_" + Date.now().toString(36),
    name,
    email,
    passwordHash,
    provider: "email",
    createdAt: Date.now(),
  };
  list.push(user);
  saveUsers(list);
  setSession(user);
  return true;
}

export async function login(email, password) {
  auth.error = null;
  email = (email || "").trim().toLowerCase();
  const user = users().find((u) => u.email === email);
  if (!user) {
    auth.error = "لا يوجد حساب بهذا البريد الإلكتروني";
    return false;
  }
  auth.busy = true;
  const passwordHash = await hash(email + ":" + password);
  auth.busy = false;
  if (passwordHash !== user.passwordHash) {
    auth.error = "كلمة المرور غير صحيحة";
    return false;
  }
  setSession(user);
  return true;
}

export function loginWithGoogle() {
  auth.error =
    "تسجيل الدخول عبر Google يحتاج إعداد معرّف OAuth (Client ID) في الإعدادات. يمكنك الآن إنشاء حساب بالبريد.";
  return false;
}

export function logout() {
  auth.session = null;
  save("rika-session", null);
}
