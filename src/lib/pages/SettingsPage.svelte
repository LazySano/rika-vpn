<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { getVersion } from "@tauri-apps/api/app";
  import {
    Sun,
    Moon,
    Rocket,
    Shield,
    Zap,
    Server,
    Upload,
    FileKey,
    DownloadCloud,
  } from "@lucide/svelte";
  import {
    app,
    toggleTheme,
    importConfig,
    setStartup,
    setKillswitch,
    setAutoconnect,
    setDns,
  } from "$lib/stores/app.svelte.js";
  import { updater, checkForUpdates, installUpdate } from "$lib/stores/updater.svelte.js";

  let lastImported = $state(null);
  let version = $state("");

  onMount(async () => {
    try {
      version = await getVersion();
    } catch {
      version = "0.1.0";
    }
  });

  async function onFile(event) {
    const file = event.target.files?.[0];
    if (!file) return;
    const text = await file.text();
    let protocol = null;
    let geo = null;
    try {
      const meta = await invoke("parse_profile", { name: file.name, text });
      protocol = meta.protocol;
      if (meta.endpoint) {
        const host = meta.endpoint.split(":")[0];
        try {
          geo = await invoke("geoip", { host });
        } catch {
          geo = null;
        }
      }
    } catch {
      protocol = null;
    }
    lastImported = importConfig(file.name, text, protocol, geo);
    event.target.value = "";
  }

  const dnsOptions = [
    { value: "auto", label: "تلقائي" },
    { value: "1.1.1.1", label: "Cloudflare 1.1.1.1" },
    { value: "8.8.8.8", label: "Google 8.8.8.8" },
  ];
</script>

<div class="mx-auto max-w-2xl space-y-7">
  <div class="rounded-[28px] border border-line bg-surface p-8 shadow-sm">
    <h2 class="text-2xl font-extrabold text-ink">الإعدادات</h2>

    <div class="mt-6 space-y-3">
      <div class="flex items-center justify-between rounded-2xl bg-surface-2 p-4">
        <span class="flex items-center gap-3 font-semibold text-ink">
          {#if app.theme === "dark"}<Moon size={18} class="text-brand" />{:else}<Sun size={18} class="text-brand" />{/if}
          المظهر
        </span>
        <div class="flex items-center gap-1 rounded-full bg-surface p-1">
          <button
            onclick={() => app.theme === "dark" && toggleTheme()}
            class="rounded-full px-4 py-1.5 text-sm font-bold transition {app.theme === 'light' ? 'bg-brand text-white' : 'text-muted'}"
          >
            فاتح
          </button>
          <button
            onclick={() => app.theme === "light" && toggleTheme()}
            class="rounded-full px-4 py-1.5 text-sm font-bold transition {app.theme === 'dark' ? 'bg-brand text-white' : 'text-muted'}"
          >
            مظلم
          </button>
        </div>
      </div>

      <div class="flex items-center justify-between rounded-2xl bg-surface-2 p-4">
        <span class="flex items-center gap-3 font-semibold text-ink">
          <Rocket size={18} class="text-brand" /> التشغيل مع بدء النظام
        </span>
        <button
          onclick={() => setStartup(!app.settings.startup)}
          aria-label="التشغيل مع بدء النظام"
          class="relative h-7 w-12 rounded-full transition {app.settings.startup ? 'bg-brand' : 'bg-muted/40'}"
        >
          <span
            class="absolute top-1 h-5 w-5 rounded-full bg-white shadow transition-all {app.settings.startup ? 'start-6' : 'start-1'}"
          ></span>
        </button>
      </div>

      <div class="flex items-center justify-between rounded-2xl bg-surface-2 p-4">
        <span class="flex items-center gap-3 font-semibold text-ink">
          <Zap size={18} class="text-brand" /> الاتصال التلقائي عند البدء
        </span>
        <button
          onclick={() => setAutoconnect(!app.settings.autoconnect)}
          aria-label="الاتصال التلقائي عند البدء"
          class="relative h-7 w-12 rounded-full transition {app.settings.autoconnect ? 'bg-brand' : 'bg-muted/40'}"
        >
          <span
            class="absolute top-1 h-5 w-5 rounded-full bg-white shadow transition-all {app.settings.autoconnect ? 'start-6' : 'start-1'}"
          ></span>
        </button>
      </div>

      <div class="flex items-center justify-between rounded-2xl bg-surface-2 p-4">
        <span class="flex items-center gap-3 font-semibold text-ink">
          <Shield size={18} class="text-brand" /> مفتاح الحماية (منع التسريب)
        </span>
        <button
          onclick={() => setKillswitch(!app.settings.killswitch)}
          aria-label="مفتاح الحماية"
          class="relative h-7 w-12 rounded-full transition {app.settings.killswitch ? 'bg-brand' : 'bg-muted/40'}"
        >
          <span
            class="absolute top-1 h-5 w-5 rounded-full bg-white shadow transition-all {app.settings.killswitch ? 'start-6' : 'start-1'}"
          ></span>
        </button>
      </div>

      <div class="flex items-center justify-between rounded-2xl bg-surface-2 p-4">
        <span class="flex items-center gap-3 font-semibold text-ink">
          <Server size={18} class="text-brand" /> خادم DNS
        </span>
        <select
          value={app.settings.dns}
          onchange={(e) => setDns(e.currentTarget.value)}
          class="rounded-xl bg-surface px-3 py-2 text-sm text-ink outline-none"
        >
          {#each dnsOptions as o}
            <option value={o.value}>{o.label}</option>
          {/each}
        </select>
      </div>

      <div class="rounded-2xl bg-surface-2 p-4">
        <div class="flex items-center gap-3 font-semibold text-ink">
          <FileKey size={18} class="text-brand" /> استيراد ملف تعريف
        </div>
        <p class="mt-1 text-xs text-muted">الصيغ المدعومة: WireGuard ‎(.conf)‎ و OpenVPN ‎(.ovpn)‎.</p>
        <label
          class="mt-3 inline-flex cursor-pointer items-center gap-2 rounded-xl bg-brand px-4 py-2 text-sm font-bold text-white transition hover:bg-brand-hover"
        >
          <Upload size={16} /> اختيار ملف
          <input type="file" accept=".conf,.ovpn,text/plain" class="hidden" onchange={onFile} />
        </label>
        {#if lastImported}
          <p class="mt-2 text-xs text-brand">تم استيراد: {lastImported.name}</p>
        {/if}
      </div>
    </div>
  </div>

  <div class="rounded-[28px] border border-line bg-surface p-8 shadow-sm">
    <div class="flex items-center gap-3">
      <DownloadCloud size={22} class="text-brand" />
      <h2 class="text-2xl font-extrabold text-ink">التحديثات</h2>
    </div>

    <div class="mt-5 flex items-center justify-between rounded-2xl bg-surface-2 p-4">
      <div>
        <div class="font-semibold text-ink">النسخة الحالية</div>
        <div dir="ltr" class="text-sm text-muted">{version || "…"}</div>
      </div>
      <button
        onclick={() => checkForUpdates(false)}
        disabled={updater.checking}
        class="rounded-xl bg-brand px-4 py-2 text-sm font-bold text-white transition hover:bg-brand-hover disabled:opacity-70"
      >
        {updater.checking ? "جارٍ التحقق…" : "التحقق من التحديثات"}
      </button>
    </div>

    {#if updater.available}
      <div class="mt-3 rounded-2xl border border-brand/40 bg-brand-soft/50 p-4">
        <div class="font-bold text-brand">يتوفّر إصدار {updater.available.version}</div>
        {#if updater.available.notes}
          <p class="mt-1 whitespace-pre-line text-xs leading-5 text-muted">
            {updater.available.notes}
          </p>
        {/if}
        <button
          onclick={installUpdate}
          disabled={updater.downloading}
          class="mt-3 rounded-xl bg-brand px-4 py-2 text-sm font-bold text-white transition hover:bg-brand-hover disabled:opacity-70"
        >
          {updater.downloading
            ? `يُنزّل… ${updater.progress}%`
            : "تثبيت التحديث وإعادة التشغيل"}
        </button>
      </div>
    {/if}

    {#if updater.error}
      <p class="mt-3 rounded-xl bg-danger/10 px-3 py-2 text-xs text-danger">{updater.error}</p>
    {/if}

    <p class="mt-3 text-xs leading-5 text-muted">
      تُنزَّل التحديثات من إصدارات GitHub وتُتحقَّق من توقيعها رقميًا قبل التثبيت.
    </p>
  </div>
</div>
