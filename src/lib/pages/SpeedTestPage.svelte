<script>
  import { Gauge, ArrowDown, ArrowUp, Clock, LoaderCircle, History } from "@lucide/svelte";
  import { app, runSpeedTest } from "$lib/stores/app.svelte.js";

  let running = $state(false);

  async function run() {
    running = true;
    try {
      await runSpeedTest();
    } catch {
      /* notification already pushed */
    } finally {
      running = false;
    }
  }

  const latest = $derived(app.speedTests[0] ?? null);

  function fmt(ts) {
    return new Date(ts).toLocaleString("ar-EG", {
      day: "2-digit",
      month: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
    });
  }
</script>

<div class="mx-auto max-w-3xl space-y-7">
  <div class="rounded-[28px] border border-line bg-surface p-8 text-center shadow-sm">
    <Gauge size={40} class="mx-auto text-brand" />
    <h2 class="mt-3 text-2xl font-extrabold text-ink">اختبار السرعة</h2>
    <p class="mt-1 text-sm text-muted">قياس فعلي لسرعة التنزيل والرفع وزمن الاستجابة.</p>

    <div class="mt-8 grid grid-cols-3 gap-4">
      <div class="rounded-2xl bg-surface-2 p-5">
        <Clock size={20} class="mx-auto text-muted" />
        <div class="mt-2 text-2xl font-extrabold text-ink">
          {latest ? latest.ping : "—"}
        </div>
        <div class="text-xs text-muted">زمن الاستجابة (ملّي ثانية)</div>
      </div>
      <div class="rounded-2xl bg-surface-2 p-5">
        <ArrowDown size={20} class="mx-auto text-danger" />
        <div class="mt-2 text-2xl font-extrabold text-ink">
          {latest ? latest.down : "—"}
        </div>
        <div class="text-xs text-muted">سرعة التنزيل (م.بت/ث)</div>
      </div>
      <div class="rounded-2xl bg-surface-2 p-5">
        <ArrowUp size={20} class="mx-auto text-brand" />
        <div class="mt-2 text-2xl font-extrabold text-ink">{latest ? latest.up : "—"}</div>
        <div class="text-xs text-muted">سرعة الرفع (م.بت/ث)</div>
      </div>
    </div>

    <button
      onclick={run}
      disabled={running}
      class="mt-8 inline-flex items-center gap-2 rounded-full bg-brand px-10 py-4 text-lg font-bold text-white shadow-xl shadow-brand/30 transition hover:bg-brand-hover disabled:opacity-80"
    >
      {#if running}<LoaderCircle size={20} class="animate-spin" /> جارٍ القياس…{:else}ابدأ الاختبار{/if}
    </button>
  </div>

  <div class="rounded-[28px] border border-line bg-surface p-6 shadow-sm">
    <div class="flex items-center gap-2 font-bold text-ink">
      <History size={18} class="text-brand" /> آخر 5 اختبارات
    </div>

    {#if app.speedTests.length === 0}
      <p class="py-8 text-center text-sm text-muted">لم تُجرِ أي اختبار بعد.</p>
    {:else}
      <div class="mt-4 divide-y divide-line">
        {#each app.speedTests as s, i}
          <div class="flex items-center justify-between py-3 text-sm">
            <span class="text-muted">{fmt(s.ts)}</span>
            <span class="flex items-center gap-4">
              <span class="flex items-center gap-1 text-danger">
                <ArrowDown size={14} /> <b>{s.down}</b>
              </span>
              <span class="flex items-center gap-1 text-brand">
                <ArrowUp size={14} /> <b>{s.up}</b>
              </span>
              <span class="flex items-center gap-1 text-muted">
                <Clock size={14} /> {s.ping}
              </span>
            </span>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
