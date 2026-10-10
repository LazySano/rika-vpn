<script>
  import { onMount } from "svelte";
  import { Globe, RefreshCw, LoaderCircle, Power, Users, Signal } from "@lucide/svelte";
  import { community, fetchVpnGate, connectVpnGate } from "$lib/stores/app.svelte.js";

  const filters = [
    { value: "JP", label: "اليابان" },
    { value: "KR", label: "كوريا" },
    { value: "US", label: "أمريكا" },
    { value: "GB", label: "بريطانيا" },
    { value: "", label: "الكل" },
  ];

  onMount(() => {
    if (community.list.length === 0) fetchVpnGate();
  });

  function setFilter(v) {
    community.filter = v;
    fetchVpnGate();
  }

  const flag = (cc) => `https://flagcdn.com/w40/${cc.toLowerCase()}.png`;
  const mbps = (bps) => Math.round((bps / 1_000_000) * 10) / 10;
</script>

<div class="mx-auto max-w-4xl space-y-6">
  <div class="rounded-[28px] border border-line bg-surface p-6 shadow-sm">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <Globe size={22} class="text-brand" />
        <div>
          <h2 class="text-xl font-extrabold text-ink">سيرفرات مجانية (VPNGate)</h2>
          <p class="text-xs text-muted">مئات السيرفرات من متطوعين — الجودة متفاوتة.</p>
        </div>
      </div>
      <button
        onclick={fetchVpnGate}
        disabled={community.loading}
        class="inline-flex items-center gap-2 rounded-xl bg-brand px-4 py-2 text-sm font-bold text-white transition hover:bg-brand-hover disabled:opacity-70"
      >
        {#if community.loading}<LoaderCircle size={16} class="animate-spin" /> جارٍ الجلب…{:else}<RefreshCw size={16} /> تحديث{/if}
      </button>
    </div>

    <div class="mt-4 flex flex-wrap gap-2">
      {#each filters as f}
        <button
          onclick={() => setFilter(f.value)}
          class="rounded-full border px-4 py-1.5 text-sm font-semibold transition {community.filter ===
          f.value
            ? 'border-brand bg-brand-soft text-brand'
            : 'border-line text-muted hover:text-ink'}">{f.label}</button>
      {/each}
    </div>

    {#if community.error}
      <p class="mt-4 rounded-xl bg-danger/10 px-3 py-2 text-xs text-danger">{community.error}</p>
    {/if}
    <p class="mt-3 rounded-xl bg-surface-2 px-3 py-2 text-[11px] leading-5 text-muted">
      ملاحظة: سيرفرات VPNGate عناوينها علنية ومحظورة في خدمات البث (Netflix/Abema). للبث استخدم VPS
      خاصًا.
    </p>
  </div>

  <div class="rounded-[28px] border border-line bg-surface p-2 shadow-sm">
    {#if community.loading && community.list.length === 0}
      <p class="py-10 text-center text-sm text-muted">جارٍ تحميل السيرفرات…</p>
    {:else if community.list.length === 0}
      <p class="py-10 text-center text-sm text-muted">لا توجد سيرفرات — اضغط «تحديث».</p>
    {:else}
      <div class="max-h-[60vh] overflow-y-auto">
        {#each community.list as srv (srv.hostname + srv.ip)}
          <div
            class="flex items-center gap-3 border-b border-line px-3 py-3 last:border-0"
          >
            <img src={flag(srv.country_short)} alt="" class="h-7 w-7 rounded-full object-cover ring-1 ring-line" />
            <div class="min-w-0 flex-1">
              <div class="truncate text-sm font-bold text-ink" dir="ltr">{srv.hostname}</div>
              <div class="text-xs text-muted">{srv.country_long}</div>
            </div>
            <div class="hidden items-center gap-4 text-xs text-muted sm:flex">
              <span class="flex items-center gap-1"><Signal size={12} />{srv.ping} ملّي ث</span>
              <span class="flex items-center gap-1"><Zap size={12} />{mbps(srv.speed)}</span>
              <span class="flex items-center gap-1"><Users size={12} />{srv.sessions}</span>
            </div>
            <button
              onclick={() => connectVpnGate(srv)}
              class="inline-flex items-center gap-1 rounded-xl bg-brand px-3 py-1.5 text-xs font-bold text-white transition hover:bg-brand-hover"
            >
              <Power size={14} /> اتصال
            </button>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
