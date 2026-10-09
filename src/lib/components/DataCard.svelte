<script>
  import { app } from "$lib/stores/app.svelte.js";

  const bars = $derived(app.samples.slice(-24));
  const max = $derived(Math.max(1, ...bars.map((b) => b.down + b.up)));
  const totalMB = $derived((app.metrics.sessionIn + app.metrics.sessionOut) / 1_000_000);
  const downTotal = $derived(app.metrics.sessionIn / 1_000_000);
  const upTotal = $derived(app.metrics.sessionOut / 1_000_000);

  function fmt(n) {
    if (n >= 1024) return (n / 1024).toFixed(2);
    return n.toFixed(1);
  }
</script>

<section class="rounded-[28px] border border-line bg-surface p-6 shadow-sm">
  <div class="flex items-start justify-between">
    <div>
      <div class="text-2xl font-extrabold text-ink">
        {fmt(totalMB)}
        <span class="text-sm font-medium text-muted">{totalMB >= 1024 ? "جيجابايت" : "ميغابايت"}</span>
      </div>
      <div class="text-sm text-muted">بيانات هذه الجلسة</div>
    </div>
    <span class="rounded-2xl border border-line px-3 py-2 text-sm text-muted">مباشر</span>
  </div>

  <div class="relative mt-6">
    <div
      class="pointer-events-none absolute inset-0 flex flex-col justify-between text-[10px] text-muted/60"
    >
      <div class="flex items-center gap-2"><span class="flex-1 border-t border-line"></span></div>
      <div class="flex items-center gap-2"><span class="flex-1 border-t border-line"></span></div>
      <div class="h-px"></div>
    </div>

    <div class="relative flex h-28 items-stretch gap-1">
      {#if bars.length === 0}
        <div class="flex h-full w-full items-center justify-center text-xs text-muted">
          بانتظار حركة الشبكة…
        </div>
      {:else}
        {#each bars as bar}
          <div class="flex h-full flex-1 items-end justify-center gap-[2px]">
            <span
              class="w-1 rounded-full bg-danger"
              style="height:{Math.max(2, (bar.down / max) * 100)}%"
            ></span>
            <span
              class="w-1 rounded-full bg-brand"
              style="height:{Math.max(2, (bar.up / max) * 100)}%"
            ></span>
          </div>
        {/each}
      {/if}
    </div>
  </div>

  <div class="mt-5 flex items-center justify-between text-sm">
    <span class="flex items-center gap-2">
      <span class="h-2 w-2 rounded-full bg-danger"></span>
      <b class="text-ink">{fmt(downTotal)}</b>
      <span class="text-muted">تنزيل</span>
    </span>
    <span class="flex items-center gap-2">
      <span class="h-2 w-2 rounded-full bg-brand"></span>
      <b class="text-ink">{fmt(upTotal)}</b>
      <span class="text-muted">رفع</span>
    </span>
  </div>
</section>
