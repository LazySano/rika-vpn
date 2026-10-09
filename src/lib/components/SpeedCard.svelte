<script>
  import { Rocket } from "@lucide/svelte";
  import { app } from "$lib/stores/app.svelte.js";

  const w = 320;
  const h = 130;
  const pad = 8;

  const values = $derived(app.samples.slice(-15).map((s) => s.down + s.up));

  const peak = $derived.by(() => {
    const test = app.speedTests[0]?.down ?? 0;
    let live = 0;
    for (const s of app.samples) live = Math.max(live, s.down + s.up);
    return Math.max(test, live, app.metrics.peak);
  });

  function buildPath(vals) {
    const n = vals.length;
    if (n < 2) return null;
    const max = Math.max(1, ...vals);
    const xs = vals.map((_, i) => pad + (i * (w - pad * 2)) / (n - 1));
    const ys = vals.map((v) => h - pad - (v / max) * (h - pad * 2));
    let d = `M ${xs[0]} ${ys[0]}`;
    for (let i = 0; i < n - 1; i++) {
      const cx = (xs[i] + xs[i + 1]) / 2;
      d += ` C ${cx} ${ys[i]}, ${cx} ${ys[i + 1]}, ${xs[i + 1]} ${ys[i + 1]}`;
    }
    return { line: d, area: `${d} L ${xs[n - 1]} ${h} L ${xs[0]} ${h} Z` };
  }

  const path = $derived(buildPath(values));
  const display = $derived(peak >= 10 ? Math.round(peak) : Math.round(peak * 10) / 10);
</script>

<section class="rounded-[28px] border border-line bg-surface p-6 shadow-sm">
  <div class="text-center">
    <div class="text-sm font-semibold text-muted">أعلى سرعة</div>
    <div class="mt-2 flex items-center justify-center gap-2">
      <span class="text-2xl">🚀</span>
      <span class="text-4xl font-extrabold text-ink">{display}</span>
    </div>
    <div class="mt-1 text-sm text-muted">ميغابت/ث</div>
  </div>

  <svg viewBox="0 0 {w} {h}" class="mt-4 h-32 w-full" preserveAspectRatio="none">
    <defs>
      <linearGradient id="rikaLineFill" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0" stop-color="#6c5ce7" stop-opacity="0.30" />
        <stop offset="1" stop-color="#6c5ce7" stop-opacity="0" />
      </linearGradient>
    </defs>
    {#if path}
      <path d={path.area} fill="url(#rikaLineFill)" />
      <path d={path.line} fill="none" stroke="#6c5ce7" stroke-width="2.5" stroke-linecap="round" />
    {:else}
      <line x1={pad} y1={h / 2} x2={w - pad} y2={h / 2} stroke="#6c5ce7" stroke-width="2" stroke-dasharray="4 6" opacity="0.4" />
    {/if}
  </svg>
</section>
