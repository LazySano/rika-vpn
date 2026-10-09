<script>
  import { onMount } from "svelte";
  import { Activity, Timer, Waves, ShieldAlert, Signal, RefreshCw, LoaderCircle } from "@lucide/svelte";
  import { app, runNetworkProbe } from "$lib/stores/app.svelte.js";

  const running = $derived(app.netStats.running);
  const hasData = $derived(app.netStats.avg_ms !== null);

  const quality = $derived.by(() => {
    const s = app.netStats;
    if (s.avg_ms === null) return null;
    if (s.loss_pct > 5 || s.avg_ms > 200) return { label: "ضعيفة", cls: "bg-danger/10 text-danger" };
    if (s.loss_pct > 1 || s.avg_ms > 100)
      return { label: "متوسطة", cls: "bg-amber-500/10 text-amber-600" };
    return { label: "ممتازة", cls: "bg-emerald-500/10 text-emerald-600" };
  });

  async function refresh() {
    try {
      await runNetworkProbe();
    } catch {
      /* ignore */
    }
  }

  onMount(() => {
    if (!hasData) refresh();
  });
</script>

<div class="mx-auto max-w-3xl space-y-7">
  <div class="rounded-[28px] border border-line bg-surface p-8 shadow-sm">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <Activity size={24} class="text-brand" />
        <h2 class="text-2xl font-extrabold text-ink">تحليل الشبكة</h2>
      </div>
      <button
        onclick={refresh}
        disabled={running}
        class="inline-flex items-center gap-2 rounded-xl bg-surface-2 px-4 py-2 text-sm font-semibold text-muted transition hover:text-ink disabled:opacity-70"
      >
        {#if running}<LoaderCircle size={16} class="animate-spin" /> جارٍ القياس…{:else}<RefreshCw size={16} /> إعادة القياس{/if}
      </button>
    </div>

    <div class="mt-6 grid grid-cols-3 gap-4">
      <div class="rounded-2xl bg-surface-2 p-5 text-center">
        <Timer size={20} class="mx-auto text-muted" />
        <div class="mt-2 text-2xl font-extrabold text-ink">
          {hasData ? app.netStats.avg_ms : "—"}
          <span class="text-sm font-medium text-muted"> ملّي ث</span>
        </div>
        <div class="text-xs text-muted">زمن الاستجابة</div>
      </div>
      <div class="rounded-2xl bg-surface-2 p-5 text-center">
        <Waves size={20} class="mx-auto text-muted" />
        <div class="mt-2 text-2xl font-extrabold text-ink">
          {hasData ? app.netStats.jitter_ms : "—"}
          <span class="text-sm font-medium text-muted"> ملّي ث</span>
        </div>
        <div class="text-xs text-muted">التذبذب</div>
      </div>
      <div class="rounded-2xl bg-surface-2 p-5 text-center">
        <ShieldAlert size={20} class="mx-auto text-muted" />
        <div class="mt-2 text-2xl font-extrabold text-ink">
          {hasData ? app.netStats.loss_pct : "—"}
          <span class="text-sm font-medium text-muted"> %</span>
        </div>
        <div class="text-xs text-muted">فقدان الحزم</div>
      </div>
    </div>
  </div>

  <div
    class="flex items-center justify-between rounded-[28px] border border-line bg-surface p-6 shadow-sm"
  >
    <span class="flex items-center gap-3 text-muted">
      <Signal size={20} class="text-brand" /> جودة الاتصال
    </span>
    {#if quality}
      <span class="rounded-full px-4 py-1.5 text-sm font-bold {quality.cls}">{quality.label}</span>
    {:else}
      <span class="text-sm text-muted">بانتظار القياس…</span>
    {/if}
  </div>
</div>
