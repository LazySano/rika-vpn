import { load, save } from "./storage.js";

const initial = load("rika-notifs", []);

export const notify = $state({
  items: initial,
  unread: initial.filter((i) => !i.read).length,
});

export function push(type, title, message) {
  const item = {
    id: Date.now().toString(36) + Math.random().toString(36).slice(2, 6),
    type,
    title,
    message,
    ts: Date.now(),
    read: false,
  };
  notify.items = [item, ...notify.items].slice(0, 50);
  notify.unread = notify.items.filter((i) => !i.read).length;
  save("rika-notifs", notify.items);
}

export function markAllRead() {
  notify.items = notify.items.map((i) => ({ ...i, read: true }));
  notify.unread = 0;
  save("rika-notifs", notify.items);
}

export function removeItem(id) {
  notify.items = notify.items.filter((i) => i.id !== id);
  notify.unread = notify.items.filter((i) => !i.read).length;
  save("rika-notifs", notify.items);
}

export function clearAll() {
  notify.items = [];
  notify.unread = 0;
  save("rika-notifs", notify.items);
}

export function timeAgo(ts) {
  const diff = Math.max(0, Date.now() - ts);
  const s = Math.floor(diff / 1000);
  if (s < 60) return "الآن";
  const m = Math.floor(s / 60);
  if (m < 60) return `قبل ${m} د`;
  const h = Math.floor(m / 60);
  if (h < 24) return `قبل ${h} س`;
  return `قبل ${Math.floor(h / 24)} ي`;
}
