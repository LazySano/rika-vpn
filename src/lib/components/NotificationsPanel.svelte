<script>
  import {
    CheckCheck,
    Trash2,
    X,
    Info,
    CheckCircle2,
    AlertTriangle,
    XCircle,
    Bell,
  } from "@lucide/svelte";
  import { notify, markAllRead, clearAll, removeItem, timeAgo } from "$lib/stores/notify.svelte.js";

  let { onClose } = $props();

  const iconFor = (type) =>
    type === "success"
      ? CheckCircle2
      : type === "error"
        ? XCircle
        : type === "warning"
          ? AlertTriangle
          : Info;

  const colorFor = (type) =>
    type === "error"
      ? "text-danger"
      : type === "warning"
        ? "text-amber-500"
        : type === "success"
          ? "text-emerald-500"
          : "text-brand";
</script>

<div
  class="absolute end-0 top-14 z-50 w-[360px] max-w-[92vw] overflow-hidden rounded-3xl border border-line bg-surface shadow-2xl"
>
  <div class="flex items-center justify-between border-b border-line px-4 py-3">
    <div class="flex items-center gap-2 font-bold text-ink"><Bell size={16} /> الإشعارات</div>
    <div class="flex items-center gap-1">
      <button
        onclick={markAllRead}
        class="rounded-lg p-2 text-muted transition hover:text-ink"
        aria-label="تعليم الكل كمقروء"
        title="تعليم الكل كمقروء"
      >
        <CheckCheck size={16} />
      </button>
      <button
        onclick={clearAll}
        class="rounded-lg p-2 text-muted transition hover:text-danger"
        aria-label="مسح الكل"
        title="مسح الكل"
      >
        <Trash2 size={16} />
      </button>
      <button
        onclick={onClose}
        class="rounded-lg p-2 text-muted transition hover:text-ink"
        aria-label="إغلاق"
      >
        <X size={16} />
      </button>
    </div>
  </div>

  <div class="max-h-[60vh] overflow-y-auto">
    {#if notify.items.length === 0}
      <p class="px-4 py-10 text-center text-sm text-muted">لا توجد إشعارات بعد</p>
    {:else}
      {#each notify.items as n (n.id)}
        {@const Icon = iconFor(n.type)}
        <div
          class="group flex gap-3 border-b border-line px-4 py-3 last:border-0 {n.read
            ? ''
            : 'bg-brand-soft/40'}"
        >
          <Icon size={18} class="{colorFor(n.type)} mt-0.5 shrink-0" />
          <div class="flex-1">
            <div class="flex items-center justify-between gap-2">
              <span class="text-sm font-bold text-ink">{n.title}</span>
              <span class="shrink-0 text-[10px] text-muted">{timeAgo(n.ts)}</span>
            </div>
            <p class="mt-0.5 text-xs leading-5 text-muted">{n.message}</p>
          </div>
          <button
            onclick={() => removeItem(n.id)}
            aria-label="حذف"
            class="self-start text-muted opacity-0 transition group-hover:opacity-100 hover:text-danger"
          >
            <X size={14} />
          </button>
        </div>
      {/each}
    {/if}
  </div>
</div>
