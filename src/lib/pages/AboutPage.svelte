<script>
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { ShieldCheck, Info } from "@lucide/svelte";
  import { t } from "$lib/i18n/ar.js";
  import Logo from "$lib/components/Logo.svelte";

  const X_URL = "https://x.com/lazysanosky";

  const benefits = [t.about.b1, t.about.b2, t.about.b3, t.about.b4, t.about.b5];

  async function openDeveloper() {
    try {
      await openUrl(X_URL);
    } catch {
      window.open(X_URL, "_blank");
    }
  }
</script>

<div class="mx-auto max-w-3xl space-y-7">
  <div class="rounded-[28px] border border-line bg-surface p-8 shadow-sm">
    <div class="flex items-center gap-3">
      <Logo size={34} withText={false} />
      <div>
        <h2 class="text-2xl font-extrabold text-ink">{t.about.title}</h2>
        <p class="text-sm text-muted">{t.tagline}</p>
      </div>
    </div>

    <p class="mt-5 leading-8 text-muted">{t.about.desc}</p>

    <div class="mt-6 rounded-2xl bg-brand-soft/60 p-5">
      <div class="text-sm font-bold text-brand">{t.about.benefits}</div>
      <ul class="mt-3 space-y-2 text-sm text-ink/80">
        {#each benefits as b}
          <li class="flex items-start gap-2">
            <ShieldCheck size={18} class="mt-0.5 shrink-0 text-brand" />
            <span>{b}</span>
          </li>
        {/each}
      </ul>
    </div>

    <div class="mt-6 flex flex-col items-start gap-3 rounded-2xl border border-line p-5 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <div class="text-sm text-muted">{t.about.developer}</div>
        <div class="text-lg font-extrabold text-ink">LazySano</div>
      </div>
      <button
        onclick={openDeveloper}
        class="inline-flex items-center gap-2 rounded-full bg-ink px-5 py-2.5 text-sm font-bold text-surface transition hover:opacity-90"
      >
        <span class="grid h-5 w-5 place-items-center rounded bg-white/10 text-[11px] font-black">X</span>
        {t.about.link}
      </button>
    </div>
  </div>

  <div class="rounded-[28px] border border-line bg-surface p-6 shadow-sm">
    <div class="flex items-center gap-2 text-sm font-bold text-ink">
      <Info size={16} class="text-muted" /> {t.about.credits}
    </div>
    <ul class="mt-3 space-y-1.5 text-xs text-muted">
      <li>{t.about.creditMap}</li>
      <li>{t.about.creditFont}</li>
      <li>{t.about.creditFlags}</li>
      <li>RikaVPN النسخة 0.1.0 — مكتوب بلغة Rust وإطار Tauri.</li>
    </ul>
  </div>
</div>
