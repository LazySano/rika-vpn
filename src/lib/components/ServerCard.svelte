<script>
  import { ChevronLeft, ArrowDown, ArrowUp, Power, RefreshCw, Globe } from "@lucide/svelte";
  import WorldMap from "./WorldMap.svelte";
  import { app, toggleConnection, setPage, relaunchElevated } from "$lib/stores/app.svelte.js";
  import { t, formatDuration } from "$lib/i18n/ar.js";

  const state = $derived(app.connection.state);
  const connected = $derived(state === "connected");
  const connecting = $derived(state === "connecting");
</script>

<section class="relative flex flex-col rounded-[28px] border border-line bg-surface p-6 pb-16 shadow-sm">
  <button class="flex items-center justify-between" onclick={() => setPage("servers")}>
    <div class="flex items-center gap-3">
      {#if app.server.flag}
        <img
          src={app.server.flag}
          alt=""
          class="h-10 w-10 rounded-full object-cover ring-1 ring-line"
        />
      {:else}
        <span class="grid h-10 w-10 place-items-center rounded-full bg-brand-soft text-brand">
          <Globe size={20} />
        </span>
      {/if}
      <div class="text-start">
        <div class="text-lg font-extrabold text-ink">{app.server.country}</div>
        <div class="text-sm text-muted">{app.server.city}</div>
      </div>
    </div>
    <ChevronLeft size={20} class="text-muted" />
  </button>

  <div class="my-5">
    <WorldMap />
  </div>

  <div class="mt-2 space-y-5">
    <div class="flex items-center justify-between">
      <span class="text-muted">{connected ? t.conn.connected : t.conn.disconnected}</span>
      <span class="text-2xl font-extrabold tabular-nums text-ink">
        {formatDuration(app.connection.seconds)}
      </span>
    </div>

    <div class="flex items-center justify-between">
      <span class="flex items-center gap-2 text-muted">
        {t.conn.ip}
        <span class="rounded-md bg-surface-2 px-2 py-0.5 text-[10px] font-bold text-muted">
          {t.conn.basic}
        </span>
      </span>
      <span dir="ltr" class="font-bold tabular-nums text-ink">{app.connection.ip}</span>
    </div>

    <div class="flex items-center justify-between">
      <span class="flex items-center gap-2 text-ink">
        <ArrowDown size={18} class="text-danger" />
        <b class="tabular-nums">{app.metrics.down.toFixed(1)}</b>
        <span class="text-muted">م.ب/ث</span>
      </span>
      <span class="flex items-center gap-2 text-ink">
        <ArrowUp size={18} class="text-brand" />
        <b class="tabular-nums">{app.metrics.up.toFixed(1)}</b>
        <span class="text-muted">م.ب/ث</span>
      </span>
    </div>
  </div>

  {#if app.error}
    <div class="mt-5 rounded-2xl bg-danger/10 p-3 text-sm text-danger">
      {#if app.error === "ELEVATION_REQUIRED"}
        <div>يتطلب الاتصال تشغيل التطبيق بصلاحيات المسؤول (مدير).</div>
        <button
          onclick={relaunchElevated}
          class="mt-2 rounded-xl bg-danger px-4 py-1.5 text-xs font-bold text-white transition hover:opacity-90"
        >
          إعادة التشغيل كمسؤول
        </button>
      {:else}
        {app.error}
      {/if}
    </div>
  {/if}

  <div class="pointer-events-none absolute inset-x-0 bottom-0 flex translate-y-1/2 justify-center">
    <button
      onclick={toggleConnection}
      disabled={connecting}
      class="pointer-events-auto inline-flex items-center gap-2.5 rounded-full bg-brand px-10 py-4 text-lg font-bold text-white shadow-xl shadow-brand/30 transition hover:bg-brand-hover disabled:opacity-80"
    >
      {#if connecting}
        <RefreshCw size={20} class="animate-spin" />
        {t.conn.connecting}
      {:else if connected}
        <Power size={20} />
        {t.conn.disconnect}
      {:else}
        <Power size={20} />
        {t.conn.connect}
      {/if}
    </button>
  </div>
</section>
