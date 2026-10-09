import { invoke } from "@tauri-apps/api/core";
import { load, save } from "./storage.js";
import { push } from "./notify.svelte.js";

export function detectProtocol(text) {
  if (/\[\s*Interface\s*\]/i.test(text) || /\[\s*Peer\s*\]/i.test(text)) return "wireguard";
  if (/^\s*client\b/im.test(text) || /^\s*remote\s+/im.test(text) || /<ca>/i.test(text))
    return "openvpn";
  return "unknown";
}

export const countries = [
  { code: "us", flag: "/flags/us.png", country: "الولايات المتحدة", city: "نيويورك" },
  { code: "gb", flag: "/flags/gb.png", country: "بريطانيا", city: "لندن" },
  { code: "jp", flag: "/flags/jp.png", country: "اليابان", city: "طوكيو" },
  { code: "cn", flag: "/flags/cn.png", country: "الصين", city: "شنغهاي" },
  { code: "kr", flag: "/flags/kr.png", country: "كوريا الجنوبية", city: "سيول" },
  { code: "vn", flag: "/flags/vn.png", country: "فيتنام", city: "هانوي" },
];

export function newId() {
  if (typeof crypto !== "undefined" && crypto.randomUUID) return crypto.randomUUID();
  return "p_" + Date.now().toString(36) + Math.random().toString(36).slice(2, 7);
}

export const app = $state({
  page: "dashboard",
  theme: "light",
  server: load("rika-server", null) ?? { ...countries[0] },
  profiles: load("rika-profiles", []),
  speedTests: load("rika-speedtests", []),
  settings: {
    startup: false,
    killswitch: false,
    autoconnect: false,
    dns: "auto",
    ...load("rika-settings", {}),
  },
  elevated: false,
  realTunnel: false,
  error: null,
  metrics: { down: 0, up: 0, sessionIn: 0, sessionOut: 0, peak: 0 },
  samples: [],
  netStats: { avg_ms: null, jitter_ms: null, loss_pct: null, running: false, at: null },
  connection: { state: "disconnected", seconds: 0, ip: "—" },
});

let lastCount = null;
let sampleTick = 0;

export function setPage(page) {
  app.page = page;
}

export function useServer(server) {
  app.server = { ...server };
  save("rika-server", app.server);
  if (app.connection.state !== "disconnected") disconnect();
}

export function selectCountry(code) {
  const found = countries.find((c) => c.code === code);
  if (found) useServer(found);
}

export function importConfig(name, text, protocol) {
  const profile = {
    id: newId(),
    name: name || "ملف بدون اسم",
    protocol: protocol || detectProtocol(text),
    text,
    importedAt: Date.now(),
  };
  app.profiles = [profile, ...app.profiles];
  save("rika-profiles", app.profiles);
  selectProfile(profile.id);
  push("success", "تم استيراد ملف تعريف", `${profile.name} — ${protocolLabel(profile.protocol)}`);
  return profile;
}

export function protocolLabel(p) {
  if (p === "wireguard") return "WireGuard";
  if (p === "openvpn") return "OpenVPN";
  return "ملف مخصّص";
}

export function removeProfile(id) {
  app.profiles = app.profiles.filter((p) => p.id !== id);
  save("rika-profiles", app.profiles);
  if (app.server.profileId === id) useServer(countries[0]);
}

export function selectProfile(id) {
  const p = app.profiles.find((x) => x.id === id);
  if (!p) return;
  useServer({
    code: "custom",
    profileId: p.id,
    country: p.name,
    city: protocolLabel(p.protocol),
    flag: null,
  });
}

export function activeProfile() {
  if (!app.server.profileId) return null;
  return app.profiles.find((p) => p.id === app.server.profileId) ?? null;
}

export function applyTheme() {
  if (typeof document === "undefined") return;
  document.documentElement.classList.toggle("dark", app.theme === "dark");
}

export function initTheme() {
  const stored = load("rika-theme", null);
  if (stored === "dark" || stored === "light") app.theme = stored;
  applyTheme();
}

export function toggleTheme() {
  app.theme = app.theme === "dark" ? "light" : "dark";
  applyTheme();
  save("rika-theme", app.theme);
}

export async function refreshElevated() {
  try {
    app.elevated = await invoke("is_elevated");
  } catch {
    app.elevated = false;
  }
}

export async function relaunchElevated() {
  try {
    await invoke("relaunch_elevated");
  } catch {
    /* ignore */
  }
}

export async function pollNetwork() {
  try {
    const c = await invoke("network_counters");
    if (lastCount) {
      const dRx = Math.max(0, c.rx - lastCount.rx);
      const dTx = Math.max(0, c.tx - lastCount.tx);
      app.metrics.down = dRx / 1_000_000;
      app.metrics.up = dTx / 1_000_000;
      app.metrics.sessionIn += dRx;
      app.metrics.sessionOut += dTx;
      app.metrics.peak = Math.max(app.metrics.peak, app.metrics.down + app.metrics.up);
      sampleTick += 1;
      if (sampleTick % 3 === 0) {
        app.samples = [
          ...app.samples,
          { down: app.metrics.down, up: app.metrics.up },
        ].slice(-24);
      }
    }
    lastCount = { rx: c.rx, tx: c.tx };
  } catch {
    /* ignore */
  }
}

export async function tick() {
  await pollNetwork();
  if (app.connection.state === "connected") app.connection.seconds += 1;
}

export async function runSpeedTest() {
  const base = "https://speed.cloudflare.com";
  try {
    const t0 = performance.now();
    await fetch(base + "/__down?bytes=1000", { cache: "no-store" });
    const ping = Math.round(performance.now() - t0);

    const dlStart = performance.now();
    const resp = await fetch(base + "/__down?bytes=25000000", { cache: "no-store" });
    const buf = await resp.arrayBuffer();
    const dlSec = (performance.now() - dlStart) / 1000;
    const down = (buf.byteLength * 8) / (dlSec * 1e6);

    const payload = new Uint8Array(8_000_000);
    for (let i = 0; i < payload.length; i += 4096) payload[i] = (Math.random() * 255) | 0;
    const upStart = performance.now();
    await fetch(base + "/__up", { method: "POST", body: payload });
    const upSec = (performance.now() - upStart) / 1000;
    const up = (payload.byteLength * 8) / (upSec * 1e6);

    const result = {
      down: Math.round(down * 10) / 10,
      up: Math.round(up * 10) / 10,
      ping,
      ts: Date.now(),
    };
    app.speedTests = [result, ...app.speedTests].slice(0, 5);
    save("rika-speedtests", app.speedTests);
    push("success", "اكتمل اختبار السرعة", `تنزيل ${result.down} • رفع ${result.up} م.بت/ث`);
    return result;
  } catch (e) {
    push("error", "تعذّر إكمال اختبار السرعة", String(e));
    throw e;
  }
}

export async function runNetworkProbe() {
  app.netStats.running = true;
  try {
    const r = await invoke("network_probe", { host: "1.1.1.1", port: 53, count: 8 });
    app.netStats = {
      avg_ms: r.avg_ms,
      jitter_ms: r.jitter_ms,
      loss_pct: r.loss_pct,
      running: false,
      at: Date.now(),
    };
    return r;
  } catch (e) {
    app.netStats.running = false;
    push("error", "تعذّر تحليل الشبكة", String(e));
    throw e;
  }
}

export async function setStartup(enabled) {
  app.settings.startup = enabled;
  save("rika-settings", app.settings);
  try {
    await invoke("set_autostart", { enabled });
    push("info", "التشغيل مع بدء النظام", enabled ? "تم التفعيل" : "تم الإيقاف");
  } catch (e) {
    push("error", "تعذّر تغيير الإعداد", String(e));
  }
}

export async function setKillswitch(enabled) {
  app.settings.killswitch = enabled;
  save("rika-settings", app.settings);
  try {
    if (!enabled) await invoke("firewall_block", { block: false });
    push(
      "info",
      "مفتاح الحماية",
      enabled
        ? "سيُحجب الاتصال عند انقطاع النفق لمنع التسريب"
        : "تم الإيقاف",
    );
  } catch (e) {
    push("error", "تعذّر تغيير مفتاح الحماية", String(e));
  }
}

export function setAutoconnect(enabled) {
  app.settings.autoconnect = enabled;
  save("rika-settings", app.settings);
}

export function setDns(value) {
  app.settings.dns = value;
  save("rika-settings", app.settings);
}

export async function maybeAutoconnect() {
  if (!app.settings.autoconnect) return;
  const p = activeProfile();
  if (p && p.protocol === "wireguard") connect();
}

function notifyConnectError(e) {
  if (String(e).includes("ELEVATION_REQUIRED")) {
    app.error = "ELEVATION_REQUIRED";
    push("warning", "مطلوب صلاحيات المسؤول", "أعد تشغيل التطبيق كمسؤول لإتمام الاتصال.");
  } else if (String(e).includes("OPENVPN_MISSING")) {
    app.error = "لم يتم العثور على openvpn.exe. ضعه بجوار التطبيق.";
    push("error", "OpenVPN غير متوفّر", app.error);
  } else {
    app.error = String(e);
    push("error", "فشل الاتصال", String(e));
  }
}

export async function connect() {
  app.error = null;
  const profile = activeProfile();

  if (profile && profile.protocol === "wireguard") {
    app.connection.state = "connecting";
    try {
      const info = await invoke("connect_wireguard", { text: profile.text });
      app.realTunnel = true;
      app.connection.state = "connected";
      app.connection.seconds = 0;
      app.connection.ip = info.assigned_ip;
      if (app.settings.killswitch) await invoke("firewall_block", { block: false });
      push("success", "تم الاتصال", `${app.server.country} — ${info.assigned_ip}`);
    } catch (e) {
      app.realTunnel = false;
      app.connection.state = "disconnected";
      notifyConnectError(e);
    }
    return;
  }

  if (profile && profile.protocol === "openvpn") {
    app.connection.state = "connecting";
    try {
      await invoke("connect_openvpn", { text: profile.text });
      app.realTunnel = true;
      app.connection.state = "connected";
      app.connection.seconds = 0;
      app.connection.ip = "OpenVPN";
      push("success", "تم الاتصال عبر OpenVPN", app.server.country);
    } catch (e) {
      app.realTunnel = false;
      app.connection.state = "disconnected";
      notifyConnectError(e);
    }
    return;
  }

  app.error =
    app.profiles.length > 0
      ? "لم تختر ملف تعريف. اختره من صفحة السيرفرات."
      : "لا يوجد ملف تعريف. استورد ملف WireGuard/OpenVPN أولًا من الإعدادات.";
  push("warning", "لا يمكن الاتصال", app.error);
}

export async function disconnect() {
  if (app.realTunnel) {
    try {
      await invoke("disconnect");
    } catch {
      /* ignore */
    }
  }
  if (app.settings.killswitch && app.realTunnel) {
    try {
      await invoke("firewall_block", { block: true });
      push("warning", "مفتاح الحماية مُفعّل", "تم حجب الاتصال بعد قطع النفق. عطّل المفتاح لاستعادة الإنترنت.");
    } catch {
      /* ignore */
    }
  }
  app.realTunnel = false;
  app.connection.state = "disconnected";
  app.connection.ip = "—";
  push("info", "تم قطع الاتصال", app.server.country);
}

export function toggleConnection() {
  if (app.connection.state === "connected") disconnect();
  else if (app.connection.state === "disconnected") connect();
}
