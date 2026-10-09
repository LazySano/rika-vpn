<script>
  import { Check, Trash2, FileKey } from "@lucide/svelte";
  import { app, countries, useServer, selectProfile, removeProfile } from "$lib/stores/app.svelte.js";

  function protocolLabel(p) {
    if (p === "wireguard") return "WireGuard";
    if (p === "openvpn") return "OpenVPN";
    return "ملف مخصّص";
  }
</script>

<div class="mx-auto max-w-4xl space-y-7">
  <section class="rounded-[28px] border border-line bg-surface p-6 shadow-sm">
    <h2 class="text-xl font-extrabold text-ink">اختر الدولة</h2>
    <p class="mt-1 text-sm text-muted">6 دول متاحة — والمزيد قريبًا.</p>

    <div class="mt-5 grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
      {#each countries as c (c.code)}
        <button
          onclick={() => useServer(c)}
          class="flex items-center gap-3 rounded-2xl border p-4 text-start transition {app.server
            .code === c.code
            ? 'border-brand bg-brand-soft'
            : 'border-line hover:bg-surface-2'}"
        >
          <img src={c.flag} alt="" class="h-9 w-9 rounded-full object-cover ring-1 ring-line" />
          <div class="flex-1">
            <div class="font-bold text-ink">{c.country}</div>
            <div class="text-sm text-muted">{c.city}</div>
          </div>
          {#if app.server.code === c.code}<Check size={18} class="text-brand" />{/if}
        </button>
      {/each}
    </div>
  </section>

  {#if app.profiles.length}
    <section class="rounded-[28px] border border-line bg-surface p-6 shadow-sm">
      <h2 class="text-xl font-extrabold text-ink">ملفاتي المستوردة</h2>
      <div class="mt-5 space-y-3">
        {#each app.profiles as p (p.id)}
          <div
            class="flex items-center gap-3 rounded-2xl border p-4 {app.server.profileId === p.id
              ? 'border-brand bg-brand-soft'
              : 'border-line'}"
          >
            <span class="grid h-9 w-9 place-items-center rounded-full bg-surface-2 text-brand">
              <FileKey size={18} />
            </span>
            <button onclick={() => selectProfile(p.id)} class="flex-1 text-start">
              <div class="font-bold text-ink">{p.name}</div>
              <div class="text-sm text-muted">{protocolLabel(p.protocol)}</div>
            </button>
            {#if app.server.profileId === p.id}<Check size={18} class="text-brand" />{/if}
            <button
              onclick={() => removeProfile(p.id)}
              aria-label="حذف"
              class="grid h-9 w-9 place-items-center rounded-full text-danger transition hover:bg-danger/10"
            >
              <Trash2 size={18} />
            </button>
          </div>
        {/each}
      </div>
    </section>
  {/if}
</div>
